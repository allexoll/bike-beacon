#![no_std]
#![no_main]

// pick a panicking behavior
use core::panic::PanicInfo;
use core::cell::Cell;
use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;

use accelerometer::{RawAccelerometer, Tracker};
use lis3dh::{Lis3dh, SlaveAddr};
use rtt_target::{rprintln, rtt_init_print};
use stm32l0xx_hal::{
    exti::{ConfigurableLine, Exti, ExtiLine, GpioLine, TriggerEdge},
    pac::{self, interrupt, Interrupt},
    prelude::*,
    pwr::{self, PWR},
    rcc::Config,
    syscfg::SYSCFG,
    rtc::{self, Instant, RTC},
};
use build_timestamp::build_time;


#[derive(Clone, Copy, Debug)]
enum StateMachine {
    OFF,
    OFF_forced,
    ON,
}

static LIS3DH_INT1_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static WAKEUP_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("bike-lights-sw");

    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());
    let mut adc = dp.ADC.constrain(&mut rcc);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut exti = Exti::new(dp.EXTI);
    let mut pwr = PWR::new(dp.PWR, &mut rcc);
    let mut delay = cp.SYST.delay(rcc.clocks);
    let mut scb = cp.SCB;

    // set RTC to build time
    build_time!("%y:%m:%d:%H:%M:%S");
    let instant = Instant::new()
    .set_year(BUILD_TIME[0..2].parse::<u8>().unwrap())
    .set_month(BUILD_TIME[3..5].parse::<u8>().unwrap())
    .set_day(BUILD_TIME[6..8].parse::<u8>().unwrap())
    .set_hour(BUILD_TIME[9..11].parse::<u8>().unwrap())
    .set_minute(BUILD_TIME[12..14].parse::<u8>().unwrap())
    .set_second(BUILD_TIME[15..17].parse::<u8>().unwrap());
    let mut rtc = RTC::new(dp.RTC, &mut rcc, &mut pwr, instant);

    rtc.enable_interrupts(rtc::Interrupts {
        wakeup_timer: true,
        ..rtc::Interrupts::default()
    });

    let _apa_adc = gpioa.pa0.into_analog();
    let mut _tube_adc = gpioa.pa3.into_analog();

    let mut tube_5v_en = gpioa.pa15.into_push_pull_output();
    tube_5v_en.set_high().unwrap();
    let acc_int_1 = gpioa.pa1.into_floating_input();
    let acc_int_2 = gpioa.pa2.into_floating_input();

    let mut pm_5v_en = gpiob.pb1.into_push_pull_output();

    let line_acc_int_1 = GpioLine::from_raw_line(acc_int_1.pin_number()).unwrap();
    let line_acc_int_2 = GpioLine::from_raw_line(acc_int_2.pin_number()).unwrap();

    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);

    let sda = gpiob.pb7.into_open_drain_output();
    let scl = gpiob.pb6.into_open_drain_output();
    let mut i2c = dp.I2C1.i2c(sda, scl, 100.khz(), &mut rcc);

    // ls3dh doesnt ack on first transfer, but does after... dont know why
    let mut buffer:[u8;1] = [0];
    let _ =  i2c.read(SlaveAddr::Default.addr(), &mut buffer);

    // dont forget to turn on ACC pwr before using it (useless in rev0)
    let mut pm_acc_pwr_en = gpiob.pb0.into_push_pull_output();
    pm_acc_pwr_en.set_high().unwrap();
    let mut lis3dh = Lis3dh::new(i2c, SlaveAddr::Default).unwrap();
    
    let mut timer = dp.TIM2.timer(20.hz(), &mut rcc);
    timer.listen();
        cortex_m::interrupt::free(|cs| {
        *ADC_PERIOD.borrow(cs).borrow_mut() = false;
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });
    exti.listen_gpio(
        &mut syscfg,
        acc_int_1.port(),
        line_acc_int_1,
        TriggerEdge::Rising,
    );
    /*exti.listen_gpio(
        &mut syscfg,
        acc_int_2.port(),
        line_acc_int_2,
        TriggerEdge::Rising,
    );*/
    pm_5v_en.set_high().unwrap();


    let mut state = StateMachine::OFF;
    unsafe {
        NVIC::unmask(Interrupt::EXTI0_1);
        NVIC::unmask(Interrupt::RTC);
        NVIC::unmask(Interrupt::TIM2);
    }
    exti.listen_configurable(ConfigurableLine::RtcWakeup, TriggerEdge::Rising);

    lis3dh.config_interrupt().unwrap();

    loop {
        //let val: u16 = adc.read(&mut _tube_adc).unwrap();
        rprintln!("{:?}",state);
        asm::wfi();

        let mut movement_has_happened = false;
        let mut timeout_has_happened = false;

        cortex_m::interrupt::free(|cs|  {
                movement_has_happened = LIS3DH_INT1_INT.borrow(cs).get();
                if movement_has_happened {
                    LIS3DH_INT1_INT.borrow(cs).set(false);
                }

                timeout_has_happened = WAKEUP_INT.borrow(cs).get();
                if timeout_has_happened {
                    WAKEUP_INT.borrow(cs).set(false);
                }
            }
        );

        if movement_has_happened
        {
            rprintln!("moved");
            lis3dh.get_int_source().unwrap();
            rtc.wakeup_timer().start(20u32);

            state = match state {
                StateMachine::OFF => StateMachine::OFF, // stay off
                StateMachine::OFF_forced => StateMachine::ON, // turn on because only way to turn back on
                StateMachine::ON => StateMachine::ON,   // dont care, stay on
            }
        }
        if timeout_has_happened {
            rprintln!("timeout");
            state = match state {
                StateMachine::OFF => StateMachine::OFF, // stay off
                StateMachine::OFF_forced => StateMachine::OFF_forced, // stay off forced
                StateMachine::ON => StateMachine::OFF_forced, // going out of this
            }
        }
        


        //cortex_m::asm::wfi();
        /*exti.wait_for_irq(
            line_acc_int_1,
            pwr.stop_mode(
                &mut scb,
                &mut rcc,
                pwr::StopModeConfig {
                    ultra_low_power: true,
                },
            ),
        );*/
        //pm_5v_en.toggle().unwrap();
        //rprintln!("moving");
        //pm_5v_en.set_high().unwrap();
        //let instant = rtc.now();
        //rprintln!("{:?}", instant);
        //delay.delay_ms(1000u32);
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}

#[interrupt]
fn EXTI0_1()
{
    cortex_m::interrupt::free(|cs| {
        Exti::unpend(GpioLine::from_raw_line(1).unwrap());
        // ask mainloop to clear lis3dh interrupt source
        LIS3DH_INT1_INT.borrow(cs).set(true)
    });
}

#[interrupt]
fn RTC()
{
    Exti::unpend(ConfigurableLine::RtcWakeup);
    rprintln!("rtc");
}

#[interrupt]
fn TIM2() {

    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            // Clear the interrupt flag.
            timer.clear_irq();

            // Change the LED state on each interrupt.
            if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
                if *STATE {
                    led.set_low().unwrap();
                    *STATE = false;
                } else {
                    led.set_high().unwrap();
                    *STATE = true;
                }
            }
        }
    });
}
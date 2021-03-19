#![no_std]
#![no_main]

// pick a panicking behavior
use core::panic::PanicInfo;
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;

use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;

use lis3dh::{Lis3dh, SlaveAddr, Mode};
use rtt_target::{rprintln, rtt_init_print};
use stm32l0xx_hal::{
    exti::{ConfigurableLine, DirectLine, Exti, ExtiLine, GpioLine, TriggerEdge},
    pac::{self, interrupt, Interrupt},
    prelude::*,
    pwr::{self, PWR},
    rcc::{Config, MSIRange},
    syscfg::SYSCFG,
    rtc::{self, Instant, RTC},
    lptim::{self, ClockSrc, LpTimer},
    timer::Timer,
};
use build_timestamp::build_time;

#[derive(Clone, Copy, Debug, PartialEq)]
enum TubeState {
    SolidOff,
    SolidOn,
    Blinking5Hz,
    Snake,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum StateMachine {
    Off,
    ForcingOff,
    ForcedOff,
    On(TubeState),
}

static LIS3DH_INT1_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static WAKEUP_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static TIMER: Mutex<RefCell<Option<Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
static NEW_SAMPLE: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

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
    //let mut delay = cp.SYST.delay(rcc.clocks);
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
    exti.listen_configurable(ConfigurableLine::RtcWakeup, TriggerEdge::Rising);

    let _apa_adc = gpioa.pa0.into_analog();
    let mut _tube_adc = gpioa.pa3.into_analog();

    let mut tube_5v_en = gpioa.pa15.into_push_pull_output();
    tube_5v_en.set_high().unwrap();
    let acc_int_1 = gpioa.pa1.into_floating_input();
    let acc_int_2 = gpioa.pa2.into_floating_input();

    let mut pm_5v_en = gpiob.pb1.into_push_pull_output();

    let line_acc_int_1 = GpioLine::from_raw_line(acc_int_1.pin_number()).unwrap();
    let _line_acc_int_2 = GpioLine::from_raw_line(acc_int_2.pin_number()).unwrap();

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
    
    let mut timer = dp.TIM2.timer(10.hz(), &mut rcc);
    timer.listen();

    exti.listen_gpio(
        &mut syscfg,
        acc_int_1.port(),
        line_acc_int_1,
        TriggerEdge::Rising,
    );

    pm_5v_en.set_high().unwrap();


    let mut state = StateMachine::On(TubeState::Unknown);
    let mut tube_state = TubeState::Unknown;

    cortex_m::interrupt::free(|cs| {
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });
    unsafe {
        NVIC::unmask(Interrupt::EXTI0_1);
        NVIC::unmask(Interrupt::RTC);
        NVIC::unmask(Interrupt::TIM2);
    }
    
    lis3dh.set_mode(Mode::LowPower).unwrap();
    lis3dh.config_interrupt().unwrap();

    let mut adc_window: [u16;20] = [0;20];
    let mut adc_window_index:usize = 0;

    let mut old_state = StateMachine::Off;
    loop {

        if old_state != state {
            rprintln!("{:?}",state);
        }
        old_state = state;

        //pwr.enter_low_power_run_mode(rcc.clocks);
        /*pwr.stop_mode(
            &mut scb,
            &mut rcc,
            pwr::StopModeConfig {
                ultra_low_power: true,
            }
        ).enter();*/
        pwr.sleep_mode(&mut scb).enter();
        //cortex_m::asm::wfi();
        //pwr.exit_low_power_run_mode();
 
        let mut movement_has_happened = false;
        let mut timeout_has_happened = false;
        let mut time_to_sample = false;

        cortex_m::interrupt::free(|cs|  {
                movement_has_happened = LIS3DH_INT1_INT.borrow(cs).get();
                if movement_has_happened {
                    LIS3DH_INT1_INT.borrow(cs).set(false);
                }

                timeout_has_happened = WAKEUP_INT.borrow(cs).get();
                if timeout_has_happened {
                    WAKEUP_INT.borrow(cs).set(false);
                }
                
                time_to_sample = NEW_SAMPLE.borrow(cs).get();
                if time_to_sample {
                    NEW_SAMPLE.borrow(cs).set(false);
                }
            }
        );

        if movement_has_happened
        {
            rprintln!("moved");
            lis3dh.get_int_source().unwrap();
            rtc.wakeup_timer().start(20u32);

            state = match state {
                StateMachine::Off => StateMachine::Off, // stay off
                StateMachine::ForcingOff => StateMachine::On(TubeState::Unknown),
                StateMachine::ForcedOff => {
                    pm_5v_en.set_high().unwrap();
                    StateMachine::On(TubeState::Unknown) // turn on because only way to turn back on
                },
                StateMachine::On(x) => StateMachine::On(x),   // dont care, stay on
            }
        }
        if timeout_has_happened {
            rprintln!("timeout");
            state = match state {
                StateMachine::Off => StateMachine::Off, // stay off
                StateMachine::ForcedOff => StateMachine::ForcedOff, // stay off forced
                StateMachine::ForcingOff => StateMachine::ForcingOff,
                StateMachine::On(_) => {
                    pm_5v_en.set_low().unwrap();
                    StateMachine::ForcingOff // going out of this
                },
            };
        }

        if time_to_sample {
            let val: u16 = adc.read(&mut _tube_adc).unwrap();
            adc_window[adc_window_index] = val;
            adc_window_index +=1;
            adc_window_index %= adc_window.len();
            let mut sum:u32 = 0;
            for i in 0..adc_window.len() {
                sum += adc_window[i] as u32;
            }
            sum /= adc_window.len() as u32;
            let old_tube_state = tube_state;

            let tube_state = match sum {
                0..=125 => TubeState::SolidOff,
                126..=160 => TubeState::Snake,
                161..=200 => TubeState::Blinking5Hz,
                201..=4096 => TubeState::SolidOn,
                _ => unreachable!(),
            };
            // if state changed
            if tube_state != old_tube_state {
                // if we are in the process of forcabely turning off the device
                if state == StateMachine::ForcingOff || state == StateMachine::ForcedOff{
                    // we wait for it to be completely off
                    if tube_state == TubeState::SolidOff{
                        state = StateMachine::ForcedOff;
                    }
                }
                else
                {
                    state = match tube_state {
                        TubeState::SolidOff => StateMachine::Off,
                        x => StateMachine::On(x),
                    }
                }
            }


            //rprintln!("{:?}", tube_state);
        }
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
    cortex_m::interrupt::free(|cs| {
        Exti::unpend(ConfigurableLine::RtcWakeup);
        WAKEUP_INT.borrow(cs).set(true)
    });
}

#[interrupt]
fn TIM2() {

    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            // Clear the interrupt flag.
            timer.clear_irq();

            NEW_SAMPLE.borrow(cs).set(true);
        }
    });
}
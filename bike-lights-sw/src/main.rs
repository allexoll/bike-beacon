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
}



static LIS3DH_INT1_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static BTN_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static WAKEUP_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static TIMER: Mutex<RefCell<Option<Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
static PERIODIC: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("bike-lights-sw");

    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());


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


    let mut pm_5v_en = gpiob.pb1.into_push_pull_output();
    let acc_int_1 = gpioa.pa1.into_floating_input();
    let acc_int_2 = gpioa.pa2.into_floating_input();

    let line_acc_int_1 = GpioLine::from_raw_line(acc_int_1.pin_number()).unwrap();
    let _line_acc_int_2 = GpioLine::from_raw_line(acc_int_2.pin_number()).unwrap();

    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);

    let sda = gpiob.pb7.into_open_drain_output();
    let scl = gpiob.pb6.into_open_drain_output();
    let mut i2c = dp.I2C1.i2c(sda, scl, 100.khz(), &mut rcc);

    

    let tube_btn = gpioa.pa3.into_floating_input();
    let line_tube_btn = GpioLine::from_raw_line(tube_btn.pin_number()).unwrap();

    let mut leds = [
        gpioa.pa4.into_push_pull_output().downgrade(),
        gpioa.pa5.into_push_pull_output().downgrade(),
        gpioa.pa6.into_push_pull_output().downgrade(),
        gpioa.pa7.into_push_pull_output().downgrade(),
        gpioa.pa8.into_push_pull_output().downgrade(),
    ];
    let mut front_pwr_en = gpiob.pb4.into_push_pull_output();

    // dont forget to turn on ACC pwr before using it (useless in rev0)
    let mut pm_acc_pwr_en = gpiob.pb0.into_push_pull_output();
    pm_acc_pwr_en.set_high().unwrap();

    // ls3dh doesnt ack on first transfer, but does after... dont know why
    let mut buffer:[u8;1] = [0];
    let _ =  i2c.read(SlaveAddr::Alternate.addr(), &mut buffer);
    let mut lis3dh = Lis3dh::new(i2c, SlaveAddr::Alternate).unwrap();
    
    let mut timer = dp.TIM2.timer(10.hz(), &mut rcc);
    timer.listen();

    exti.listen_gpio(
        &mut syscfg,
        acc_int_1.port(),
        line_acc_int_1,
        TriggerEdge::Rising,
    );

    exti.listen_gpio(
        &mut syscfg,
        tube_btn.port(),
        line_tube_btn,
        TriggerEdge::Falling,
    );


    let mut tube_state = TubeState::SolidOff;

    cortex_m::interrupt::free(|cs| {
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });
    unsafe {
        NVIC::unmask(Interrupt::EXTI0_1);
        NVIC::unmask(Interrupt::EXTI2_3);
        NVIC::unmask(Interrupt::RTC);
        NVIC::unmask(Interrupt::TIM2);
    }
    
    lis3dh.set_mode(Mode::LowPower).unwrap();
    lis3dh.config_interrupt().unwrap();


    let mut snake_counter = 0;
    let mut blinker_counter = 0;
    loop {

        //rprintln!("{:?}",tube_state);

        //pwr.enter_low_power_run_mode(rcc.clocks);
        /*pwr.stop_mode(
            &mut scb,
            &mut rcc,
            pwr::StopModeConfig {
                ultra_low_power: true,
            }
        ).enter();*/
        //pwr.sleep_mode(&mut scb).enter();
        //cortex_m::asm::wfi();
        //pwr.exit_low_power_run_mode();
 

        let mut movement_has_happened = false;
        let mut timeout_has_happened = false;
        let mut periodic_has_happened = false;
        let mut button_has_happened = false;
        cortex_m::interrupt::free(|cs|  {
                movement_has_happened = LIS3DH_INT1_INT.borrow(cs).get();
                if movement_has_happened {
                    LIS3DH_INT1_INT.borrow(cs).set(false);
                }

                timeout_has_happened = WAKEUP_INT.borrow(cs).get();
                if timeout_has_happened {
                    WAKEUP_INT.borrow(cs).set(false);
                }

                periodic_has_happened = PERIODIC.borrow(cs).get();
                if periodic_has_happened {
                    PERIODIC.borrow(cs).set(false);
                }
                button_has_happened = BTN_INT.borrow(cs).get();
                if button_has_happened {
                    BTN_INT.borrow(cs).set(false);
                }
            }
        );

        if button_has_happened
        {
            tube_state = match tube_state {
                TubeState::SolidOff => {
                    pm_5v_en.set_high().unwrap();
                    TubeState::SolidOn
                },  // turn On
                TubeState::SolidOn => {
                    blinker_counter = 0;
                    TubeState::Blinking5Hz
                }, 
                TubeState::Blinking5Hz => {
                    snake_counter = 0;
                    TubeState::Snake
                },
                TubeState::Snake=> {
                    pm_5v_en.set_low().unwrap(); 
                    TubeState::SolidOff 
                }
            };
            rprintln!("pressed: => {:?}", tube_state);
        }

        if movement_has_happened
        {
            rprintln!("moved");
            lis3dh.get_int_source().unwrap();
            rtc.wakeup_timer().start(20u32);
        }
        if timeout_has_happened {
            rprintln!("timeout");
            tube_state = TubeState::SolidOff;
        }
        if periodic_has_happened {
            match tube_state {
                TubeState::SolidOff => {
                    front_pwr_en.set_low().unwrap();
                    for i in leds.iter_mut() {
                        i.set_low().unwrap();
                    }
                },
                TubeState::SolidOn => {
                    front_pwr_en.set_high().unwrap();
                    for i in leds.iter_mut() {
                        i.set_high().unwrap();
                    }
                },
                TubeState::Blinking5Hz => {
                    blinker_counter += 1;
                    if blinker_counter %10 == 0 {
                        front_pwr_en.set_high().unwrap();
                        for i in leds.iter_mut() {
                            i.set_high().unwrap();
                        }
                    }
                    else {
                        front_pwr_en.set_low().unwrap();
                        for i in leds.iter_mut() {
                            i.set_low().unwrap();
                        }
                    }
                }
                TubeState::Snake => {
                    snake_counter +=1;
                    front_pwr_en.toggle().unwrap();
                    for i in 0..=4 {
                        if snake_counter%5 == i {
                            leds[i].set_high().unwrap();
                        }
                        else{
                            leds[i].set_low().unwrap();
                        }
                    }
                }
            }
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
fn EXTI2_3()
{
    cortex_m::interrupt::free(|cs| {
        Exti::unpend(GpioLine::from_raw_line(3).unwrap());
        BTN_INT.borrow(cs).set(true)
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

            PERIODIC.borrow(cs).set(true);
        }
    });
}
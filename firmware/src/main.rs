#![no_std]
#![no_main]

// pick a panicking behavior
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;

use defmt::Format;
use defmt_rtt as _;
use max170xx::Max17048;
use panic_probe as _; // global logger

use lis3dh::{Lis3dh, SlaveAddr};
use stm32l0xx_hal::{
    exti::{Exti, ExtiLine, GpioLine, TriggerEdge},
    gpio::*,
    pac::{self, interrupt, Interrupt},
    prelude::*,
    pwr::PWR,
    rcc::Config,
    //rtc::{self, Rtc},
    syscfg::SYSCFG,
    //lptim::{self, ClockSrc, LpTimer},
    timer::Timer,
};

// light pattern
#[derive(Clone, Copy, Debug, PartialEq, Format)]
enum TubeState {
    SolidOn,
    Blinking5Hz,
    Snake,
}

#[derive(Clone, Copy, Debug, PartialEq, Format)]
enum TubeOnOff {
    On,
    Off,
}

// button manager
#[derive(Clone, Copy, Debug, PartialEq, Format)]
enum ButtonState {
    Released,
    ClickedShort,
    ClickedLong,
    ClickedLongLong,
}

#[derive(Clone, Copy, Debug, PartialEq, Format)]
enum LongDelay {
    Running(u32),
    Timeout,
    Idle,
}

const TIMEOUT: u32 = 90;
// shared values beetween main context and interrupts
static LIS3DH_INT1_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static BTN_GPIO: Mutex<RefCell<Option<gpioa::PA3<Input<Floating>>>>> =
    Mutex::new(RefCell::new(None));

//static WAKEUP_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static LONG_TIMEOUT: Mutex<Cell<LongDelay>> = Mutex::new(Cell::new(LongDelay::Idle));
static TIMER: Mutex<RefCell<Option<Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
static PERIODIC: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static TICKS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static PRESS_DURATION: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static PRESSED: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[entry]
fn main() -> ! {
    defmt::info!("bike-lights-sw");
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut exti = Exti::new(dp.EXTI);
    let mut pwr = PWR::new(dp.PWR, &mut rcc);
    //let mut delay = cp.SYST.delay(rcc.clocks);
    let mut scb = cp.SCB;

    //let mut rtc = Rtc::new(dp.RTC, &mut rcc, &mut pwr, None).unwrap();
    //let mut rtc = LongDelay::Idle;

    // rtc.enable_interrupts(rtc::Interrupts {
    //     wakeup_timer: true,
    //     ..rtc::Interrupts::default()
    // });
    //exti.listen_configurable(ConfigurableLine::RtcWakeup, TriggerEdge::Rising);

    // configure pins
    let mut pm5v_en = gpiob.pb1.into_push_pull_output();
    let pm_chg_present = gpioa.pa9.into_pull_up_input();
    let acc_int_1 = gpioa.pa1.into_floating_input();
    let sda = gpiob.pb7.into_open_drain_output();
    let scl = gpiob.pb6.into_open_drain_output();
    let tube_btn = gpioa.pa3.into_floating_input();
    let mut leds = [
        gpioa.pa4.into_push_pull_output().downgrade(),
        gpioa.pa5.into_push_pull_output().downgrade(),
        gpioa.pa6.into_push_pull_output().downgrade(),
        gpioa.pa7.into_push_pull_output().downgrade(),
        gpioa.pa8.into_push_pull_output().downgrade(),
    ];

    // configure Interrupts
    let line_acc_int = GpioLine::from_raw_line(acc_int_1.pin_number()).unwrap();
    let line_tube_btn = GpioLine::from_raw_line(tube_btn.pin_number()).unwrap();

    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);

    // configure I2C Bus
    let mut i2c = dp.I2C1.i2c(sda, scl, 100_000.Hz(), &mut rcc);
    let mut buffer: [u8; 1] = [0];
    // needed?
    let _ = i2c.read(SlaveAddr::Alternate.addr(), &mut buffer);

    // share I2C bus
    let manager = shared_bus::BusManagerSimple::new(i2c);

    let lisd3h_proxy = manager.acquire_i2c();
    //let mut lis3dh = Lis3dh::new(lisd3h_proxy, SlaveAddr::Alternate).unwrap();
    let mut lis3dh = Lis3dh::new_i2c(lisd3h_proxy, SlaveAddr::Alternate).unwrap();
    let max17048_proxy = manager.acquire_i2c();
    let mut max17048 = Max17048::new(max17048_proxy);

    let mut timer = dp.TIM2.timer(20.Hz(), &mut rcc);
    timer.listen();

    exti.listen_gpio(
        &mut syscfg,
        acc_int_1.port(),
        line_acc_int,
        TriggerEdge::Both,
    );

    exti.listen_gpio(
        &mut syscfg,
        tube_btn.port(),
        line_tube_btn,
        TriggerEdge::Both,
    );

    let mut tube_state = TubeState::SolidOn;
    let mut tube_on_off = TubeOnOff::Off;
    cortex_m::interrupt::free(|cs| {
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
        *BTN_GPIO.borrow(cs).borrow_mut() = Some(tube_btn);
    });
    unsafe {
        NVIC::unmask(Interrupt::EXTI0_1);
        NVIC::unmask(Interrupt::EXTI2_3);
        //NVIC::unmask(Interrupt::RTC);
        NVIC::unmask(Interrupt::TIM2);
    }

    //lis3dh.set_mode(Mode::LowPower).unwrap();

    let threshold = lis3dh::Threshold::g(lis3dh::Range::default(), 1.6);    // this should be the same as 100 lsb

    // Configure the threshold value for interrupt 1 to 1.1g
    lis3dh.configure_irq_threshold(lis3dh::Interrupt1, threshold).unwrap();

    // The time in 1/ODR an axis value should be above threshold in order for an
    // interrupt to be raised
    let duration = lis3dh::Duration::miliseconds(lis3dh::DataRate::Hz_400, 0.0);        // TODO: find the correct value
    lis3dh.configure_irq_duration(lis3dh::Interrupt1, duration).unwrap();

    // Congfigure IRQ source for interrupt 1
    lis3dh.configure_irq_src(
        lis3dh::Interrupt1,
        lis3dh::InterruptMode::Movement,        // unsure about this? i did not use it previously (OrCombination)
        lis3dh::InterruptConfig::high(),    // CFG ZHIE XHIE YHIE
    ).unwrap();

    // Configure IRQ pin 1
    lis3dh.configure_interrupt_pin(lis3dh::IrqPin1Config {
        // Raise if interrupt 1 is raised
        ia1_en: true,
        // Disable for all other interrupts
        ..lis3dh::IrqPin1Config::default()
    }).unwrap();

    // Go to low power mode and wake up for 25ms if measurement above 1.1g is done
    let duration = lis3dh::Duration::miliseconds(lis3dh::DataRate::Hz_400, 2.5);
    lis3dh.configure_switch_to_low_power(threshold, duration).unwrap();
    lis3dh.set_datarate(lis3dh::DataRate::Hz_400).unwrap();

    //lis3dh.config_interrupt().unwrap();
    //lis3dh.get_int_source().unwrap();

    let mut snake_counter = 0;
    let mut blinker_counter = 0;
    let mut old_pressed = false;
    loop {
        // if charger is connected,
        // get battery charge percentage, turn all leds off and then turn one led to show the 1/5th slice of the charge
        if pm_chg_present.is_high().unwrap() {
            let charge = max17048.charge_rate().unwrap();
            let charge = charge as f32 / 20.0;
            let charge = charge as usize;
            for led in leds.iter_mut() {
                led.set_low().unwrap();
            }
            for led in leds.iter_mut().take(charge) {
                led.set_high().unwrap();
            }
        } else {
            #[cfg(not(feature = "no-sleep"))]
            pwr.sleep_mode(&mut scb).enter();
            //cortex_m::asm::wfi();
            //pwr.exit_low_power_run_mode();

            let mut movement_has_happened = false;
            let mut timeout_has_happened = false;
            let mut periodic_has_happened = false;
            let mut button_has_happened = ButtonState::Released;

            cortex_m::interrupt::free(|cs| {
                timeout_has_happened = LONG_TIMEOUT.borrow(cs).get() == LongDelay::Timeout;
                if timeout_has_happened {
                    LONG_TIMEOUT.borrow(cs).set(LongDelay::Idle);
                }

                movement_has_happened = LIS3DH_INT1_INT.borrow(cs).get();
                if movement_has_happened {
                    LIS3DH_INT1_INT.borrow(cs).set(false);
                }

                periodic_has_happened = PERIODIC.borrow(cs).get();
                if periodic_has_happened {
                    PERIODIC.borrow(cs).set(false);
                }
                let mut new_pressed = PRESSED.borrow(cs).get();
                let press_duration = PRESS_DURATION.borrow(cs).get();
                if new_pressed != old_pressed || press_duration > 10 {
                    if press_duration > 20 {
                        button_has_happened = ButtonState::ClickedLongLong;
                        PRESS_DURATION.borrow(cs).set(0);
                        PRESSED.borrow(cs).set(false);
                        new_pressed = false;
                    } else if press_duration > 10 {
                        button_has_happened = ButtonState::ClickedLong;
                        PRESS_DURATION.borrow(cs).set(0);
                        PRESSED.borrow(cs).set(false);
                        new_pressed = false;
                    } else if !new_pressed {
                        button_has_happened = ButtonState::ClickedShort;
                    }
                }
                old_pressed = new_pressed;
            });

            let mut should_start_timer = false;
            let mut should_stop_timer = false;

            if button_has_happened != ButtonState::Released {
                defmt::info!("btn: {}", button_has_happened);
                if button_has_happened == ButtonState::ClickedShort {
                    if tube_on_off == TubeOnOff::Off {
                        tube_on_off = TubeOnOff::On;
                        pm5v_en.set_high().unwrap();
                        blinker_counter = 0;
                        snake_counter = 0;
                        should_start_timer = true;
                    } else {
                        tube_state = match tube_state {
                            TubeState::SolidOn => {
                                blinker_counter = 0;
                                TubeState::Blinking5Hz
                            }
                            TubeState::Blinking5Hz => {
                                snake_counter = 0;
                                TubeState::Snake
                            }
                            TubeState::Snake => TubeState::SolidOn,
                        };
                    }
                }
                if button_has_happened == ButtonState::ClickedLong {
                    tube_on_off = TubeOnOff::Off;
                    pm5v_en.set_low().unwrap();
                    for i in leds.iter_mut() {
                        i.set_low().unwrap();
                    }
                    should_stop_timer = true;
                }
                if button_has_happened == ButtonState::ClickedLongLong {
                    defmt::info!("long long");
                    // charge rate is from 0 to 100%
                    let charge_rate = max17048.charge_rate().unwrap();
                    defmt::info!("charge rate: {}", charge_rate);
                    // map charge_rate to 1 to 5
                    let charge_rate = (charge_rate / 20.0) as usize + 1;
                    // turn on and off leds according to charge rate
                    for (i, led) in leds.iter_mut().enumerate() {
                        if i < charge_rate {
                            led.set_high().unwrap();
                        } else {
                            led.set_low().unwrap();
                        }
                    }
                }
                defmt::info!("pressed: => {}:{}", tube_on_off, tube_state);
            }

            if movement_has_happened {
                defmt::info!("moved");
                lis3dh.get_irq_src(lis3dh::Interrupt1).unwrap();
                //lis3dh.get_int_source().unwrap();
                cortex_m::interrupt::free(|cs| {
                    LONG_TIMEOUT
                        .borrow(cs)
                        .set(LongDelay::Running(TIMEOUT * 20));
                });
                should_start_timer = true;
                // rtc.wakeup_timer().cancel().unwrap();
                // rtc.wakeup_timer().start(TIMEOUT);
                tube_on_off = TubeOnOff::On;
                pm5v_en.set_high().unwrap();
            }
            if timeout_has_happened {
                defmt::info!("timeout");
                tube_on_off = TubeOnOff::Off;
                pm5v_en.set_low().unwrap();
                for i in leds.iter_mut() {
                    i.set_low().unwrap();
                }
                should_stop_timer = true;
            }
            if periodic_has_happened && tube_on_off == TubeOnOff::On {
                    match tube_state {
                        TubeState::SolidOn => {
                            for i in leds.iter_mut() {
                                i.set_high().unwrap();
                            }
                        }
                        TubeState::Blinking5Hz => {
                            blinker_counter += 1;
                            if blinker_counter % 10 == 0 {
                                for i in leds.iter_mut() {
                                    i.set_high().unwrap();
                                }
                            } else {
                                for i in leds.iter_mut() {
                                    i.set_low().unwrap();
                                }
                            }
                        }
                        TubeState::Snake => {
                            snake_counter += 1;
                
                            for (i, led) in leds.iter_mut().enumerate() {
                                if (snake_counter / 2) % 5 == i {
                                    led.set_high().unwrap();
                                } else {
                                    led.set_low().unwrap();
                                }
                            }
                        }
                    }
            }
            cortex_m::interrupt::free(|cs| {
                if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
                    if should_stop_timer {
                        // better way to pause that?
                        //timer.unlisten();
                        timer.pause();
                    }
                    if should_start_timer {
                        // unpause?
                        //timer.listen();
                        timer.unpause();
                    }
                }
            })
        }
    }
}

#[interrupt]
fn EXTI0_1() {
    cortex_m::interrupt::free(|cs| {
        Exti::unpend(GpioLine::from_raw_line(1).unwrap());
        // ask mainloop to clear lis3dh interrupt source
        LIS3DH_INT1_INT.borrow(cs).set(true)
    });
}

#[interrupt]
fn EXTI2_3() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut btn) = BTN_GPIO.borrow(cs).borrow_mut().deref_mut() {
            PRESSED.borrow(cs).set(btn.is_low().unwrap());
            if btn.is_low().unwrap() {
                // reset press duration
                PRESS_DURATION.borrow(cs).set(0);
            }
        }
        Exti::unpend(GpioLine::from_raw_line(3).unwrap());
    });
}

// #[interrupt]
// fn RTC() {
//     cortex_m::interrupt::free(|cs| {
//         Exti::unpend(ConfigurableLine::RtcWakeup);
//         WAKEUP_INT.borrow(cs).set(true)
//     });
// }

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            // Clear the interrupt flag.
            timer.clear_irq();
            if PRESSED.borrow(cs).get() {
                PRESS_DURATION
                    .borrow(cs)
                    .set(PRESS_DURATION.borrow(cs).get() + 1);
            }
            // increment tick runner
            TICKS.borrow(cs).set(TICKS.borrow(cs).get() + 1);
            // tell main thread that a periodic irq has happened
            PERIODIC.borrow(cs).set(true);
            // if long timeout (previously RTC) is running, process it
            LONG_TIMEOUT
                .borrow(cs)
                .set(match LONG_TIMEOUT.borrow(cs).get() {
                    LongDelay::Running(x) => {
                        if x == 0 {
                            LongDelay::Timeout
                        } else {
                            LongDelay::Running(x - 1)
                        }
                    }
                    any => any,
                });
        }
    });
}

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
use panic_probe as _; // global logger

use lis3dh::{Lis3dh, Mode, SlaveAddr};
use stm32l0xx_hal::{
    exti::{ConfigurableLine, Exti, ExtiLine, GpioLine, TriggerEdge},
    gpio::*,
    pac::{self, interrupt, Interrupt},
    prelude::*,
    pwr::PWR,
    rcc::Config,
    rtc::{self, Rtc},
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
}

const TIMEOUT:u32 = 90;
// shared values beetween main context and interrupts
static LIS3DH_INT1_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static BTN_GPIO: Mutex<RefCell<Option<gpioa::PA3<Input<Floating>>>>> =
    Mutex::new(RefCell::new(None));

static WAKEUP_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

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

    let mut rtc = Rtc::new(dp.RTC, &mut rcc, &mut pwr, None).unwrap();

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
    let mut i2c = dp.I2C1.i2c(sda, scl, 100_000.Hz(), &mut rcc);

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
    let mut buffer: [u8; 1] = [0];
    let _ = i2c.read(SlaveAddr::Alternate.addr(), &mut buffer);
    let mut lis3dh = Lis3dh::new(i2c, SlaveAddr::Alternate).unwrap();

    let mut timer = dp.TIM2.timer(20.Hz(), &mut rcc);
    timer.listen();

    exti.listen_gpio(
        &mut syscfg,
        acc_int_1.port(),
        line_acc_int_1,
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
        NVIC::unmask(Interrupt::RTC);
        NVIC::unmask(Interrupt::TIM2);
    }

    lis3dh.set_mode(Mode::LowPower).unwrap();
    // custom function to enable moving irq if acc > 1g any direction
    lis3dh.config_interrupt().unwrap();
    lis3dh.get_int_source().unwrap();

    let mut snake_counter = 0;
    let mut blinker_counter = 0;
    let mut old_pressed = false;
    loop {
        //pwr.enter_low_power_run_mode(rcc.clocks);
        /*pwr.stop_mode(
            &mut scb,
            &mut rcc,
            pwr::StopModeConfig {
                ultra_low_power: true,
            }
        ).enter();*/

        #[cfg(not(feature = "no-sleep"))]
        pwr.sleep_mode(&mut scb).enter();
        //cortex_m::asm::wfi();
        //pwr.exit_low_power_run_mode();

        let mut movement_has_happened = false;
        let mut timeout_has_happened = false;
        let mut periodic_has_happened = false;
        let mut button_has_happened = ButtonState::Released;
        cortex_m::interrupt::free(|cs| {
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
            let mut new_pressed = PRESSED.borrow(cs).get();
            let press_duration = PRESS_DURATION.borrow(cs).get();
            if new_pressed != old_pressed || press_duration > 10 {
                if press_duration > 10 {
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

        if button_has_happened != ButtonState::Released {
            defmt::info!("btn: {}", button_has_happened);
            if button_has_happened == ButtonState::ClickedShort {
                if tube_on_off == TubeOnOff::Off {
                    tube_on_off = TubeOnOff::On;
                    pm_5v_en.set_high().unwrap();
                    blinker_counter = 0;
                    snake_counter = 0;
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
                pm_5v_en.set_low().unwrap();
                for i in leds.iter_mut() {
                    i.set_low().unwrap();
                }
            }
            defmt::info!("pressed: => {}:{}", tube_on_off, tube_state);
        }

        if movement_has_happened {
            defmt::info!("moved");
            lis3dh.get_int_source().unwrap();
            rtc.wakeup_timer().cancel().unwrap();
            rtc.wakeup_timer().start(TIMEOUT);
            tube_on_off = TubeOnOff::On;
            pm_5v_en.set_high().unwrap();
        }
        if timeout_has_happened {
            defmt::info!("timeout");
            tube_on_off = TubeOnOff::Off;
            pm_5v_en.set_low().unwrap();
            for i in leds.iter_mut() {
                i.set_low().unwrap();
            }
        }
        if periodic_has_happened {
            if tube_on_off == TubeOnOff::On {
                match tube_state {
                    TubeState::SolidOn => {
                        front_pwr_en.set_high().unwrap();
                        for i in leds.iter_mut() {
                            i.set_high().unwrap();
                        }
                    }
                    TubeState::Blinking5Hz => {
                        blinker_counter += 1;
                        if blinker_counter % 10 == 0 {
                            front_pwr_en.set_high().unwrap();
                            for i in leds.iter_mut() {
                                i.set_high().unwrap();
                            }
                        } else {
                            front_pwr_en.set_low().unwrap();
                            for i in leds.iter_mut() {
                                i.set_low().unwrap();
                            }
                        }
                    }
                    TubeState::Snake => {
                        snake_counter += 1;
                        front_pwr_en.toggle().unwrap();
                        for i in 0..=4 {
                            if (snake_counter / 2) % 5 == i {
                                leds[i].set_high().unwrap();
                            } else {
                                leds[i].set_low().unwrap();
                            }
                        }
                    }
                }
            }
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

#[interrupt]
fn RTC() {
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
            if PRESSED.borrow(cs).get() {
                PRESS_DURATION
                    .borrow(cs)
                    .set(PRESS_DURATION.borrow(cs).get() + 1);
            }
            TICKS.borrow(cs).set(TICKS.borrow(cs).get() + 1);
            PERIODIC.borrow(cs).set(true);
        }
    });
}

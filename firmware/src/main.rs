#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use core::convert::Infallible;
use core::panic::PanicInfo;

use accelerometer::Accelerometer;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;

//use defmt_rtt as _;

use max170xx::Max17048;

use lis3dh::{Lis3dh, Lis3dhI2C, SlaveAddr};
// use stm32l0xx_hal::lptim::{self, LpTimer, Periodic};
// use stm32l0xx_hal::pac::LPTIM;
// use stm32l0xx_hal::pwr::StopModeConfig;
// use stm32l0xx_hal::rcc::MSIRange;
use stm32l0xx_hal::{
    exti::{Exti, ExtiLine, GpioLine, TriggerEdge},
    gpio::{GpioExt, Input, Pin, PullUp},
    pac::{self, interrupt, Interrupt},
    prelude::*,
    pwr::PWR,
    rcc::Config,
    //rtc::{self, Rtc},
    syscfg::SYSCFG,
    //lptim::{self, ClockSrc, LpTimer},
    timer::Timer,
};

use rtt_target::{self, rprintln, rtt_init_print};
// implementation for the bike light.
// there is 5 leds controllable individually
// there is an accelerometer
// there is a battery gauge
// there is a button

// when the accelerometer moves more than 1.1G towards the ground, the display state
// changes to "Wheeling(pattern)", and starts a timeout timer of 90 seconds.

// when the timeout timer is running and the accelerometer moves more than `break_threshold` G
// towards the front, the display state changes to "Break", and starts a timeout timer of 2 seconds.

// when the charging state switches from "discharging" to "charging", or the button is pressed, the display state
// changes to "Charging", and starts a timeout timer of 2 seconds, before returning to "Off".

// when the button is pressed shortly (less than 300ms), and the state is "Off", the display state
// changes to "Wheeling(pattern-next)", and starts a timeout timer of 90 seconds.

// when the state is "Wheeling" and the button is pressed long (1s), the display state changes to "Off".

// when the button is pressed twice in less than 1s, the display shows the charging state for 2 seconds.

// enum for the wheeling pattern
#[cfg_attr(feature = "defmt_enable", derive(Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum WheelingPattern {
    Snake,
    Blink10Hz,
    SolidOn,
    ShowCharge(u32),
}
impl WheelingPattern {
    fn next(&mut self) {
        *self = match self {
            WheelingPattern::Snake => WheelingPattern::Blink10Hz,
            WheelingPattern::Blink10Hz => WheelingPattern::SolidOn,
            WheelingPattern::SolidOn => WheelingPattern::Snake,
            WheelingPattern::ShowCharge(x) => WheelingPattern::ShowCharge(*x),
        }
    }
}

// implementing the state machine

#[cfg_attr(feature = "defmt_enable", derive(Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    Off,
    Wheeling(WheelingPattern, u32),
    Break(u32),
    Charging(Option<u32>),
}

// button event
#[cfg_attr(feature = "defmt_enable", derive(Format))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ButtonEvent {
    Pressed,
    Released,
}

#[allow(dead_code)]
enum ChargerEvent {
    ChargerConnected,
    ChargerDisconnected,
}

enum ChargingEvent {
    Charging,
    Discharging,
}

// main to irq

// Button GPIO shared pin
static BTN_GPIO: Mutex<RefCell<Option<Pin<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
// charger shared pin
static CHARGER_GPIO: Mutex<RefCell<Option<Pin<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
// charging state shared pin
static CHARGING_GPIO: Mutex<RefCell<Option<Pin<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
// shared timer for the timeout
static TIMER: Mutex<RefCell<Option<Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));

// IRQ to main

// static mutexes to hold the interrupts events notification for the main thread
static ACCEL_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static BUTTON_INT: Mutex<Cell<Option<ButtonEvent>>> = Mutex::new(Cell::new(None));
static BATTERY_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
// and for charger present/ charging state
static CHARGER_INT: Mutex<Cell<Option<ChargerEvent>>> = Mutex::new(Cell::new(None));
static CHARGING_INT: Mutex<Cell<Option<ChargingEvent>>> = Mutex::new(Cell::new(None));
static PERIODIC_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ClickEvent {
    Short,
    Long,
    VeryLong,
}

struct ButtonMenu {
    state: ButtonEvent,
    last_event_counter: u32,
}

impl ButtonMenu {
    fn new() -> Self {
        Self {
            state: ButtonEvent::Released,
            last_event_counter: 0,
        }
    }

    // tick
    fn tick(&mut self) -> Option<()> {
        // if the button is pressed, we increment the counter
        if self.state == ButtonEvent::Pressed {
            self.last_event_counter += 1;
        }
        // return Some on the tick between different intervals
        match self.last_event_counter {
            20 => Some(()),
            50 => Some(()),
            _ => None,
        }
    }

    /// process a button event.
    /// store the menu state, and return the click event
    /// short click: between 2 and 20 ticks.
    /// long click: between 20 and 50 ticks.
    /// very long click: more than 50 ticks long.
    /// return the event, or None if no event.
    fn process_event(&mut self, ev: ButtonEvent) -> Option<ClickEvent> {
        // if released, return the appropriate event, and reset the counter
        if ev == ButtonEvent::Released {
            let ev = match self.last_event_counter {
                0..=1 => None,
                2..=20 => Some(ClickEvent::Short),
                21..=50 => Some(ClickEvent::Long),
                _ => Some(ClickEvent::VeryLong),
            };
            self.last_event_counter = 0;
            ev
        } else {
            self.last_event_counter = 0;
            self.state = ev;
            None
        }
    }
}

const TIMEOUT: u32 = 20 * 90;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("bike-lights-sw");
    let mut cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());
    let mut exti = Exti::new(dp.EXTI);
    let mut pwr = PWR::new(dp.PWR, &mut rcc);
    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // array of 5 leds:
    // starting from pa4, into push-pull output, downgraded to be collected into an array
    let mut leds = [
        gpioa.pa4.into_push_pull_output().downgrade(),
        gpioa.pa5.into_push_pull_output().downgrade(),
        gpioa.pa6.into_push_pull_output().downgrade(),
        gpioa.pa7.into_push_pull_output().downgrade(),
        gpioa.pa8.into_push_pull_output().downgrade(),
    ];

    // whenever one led is on, PM5V_EN (pb1, push pull), must be on
    let mut pm5v_en = gpiob.pb1.into_push_pull_output();

    // accelerator int pin is on pa0, with pull up.
    let accel_int = gpioa.pa1.into_pull_up_input();

    // battery gauge int pin is on pa1, with pull up.
    let battery_gauge_int = gpioa.pa2.into_pull_up_input();

    // the button is on pa3, with pull up.
    let button = gpioa.pa3.into_pull_up_input();

    // PM_CHG_PRESENT on pa9, floating input
    let pm_chg_present = gpioa.pa9.into_pull_up_input();

    // PM_CHG_CHARGING on pa10, floating input
    let pm_chg_charging = gpioa.pa10.into_pull_up_input();

    // i2c pins are: scl on pb6, sda on pb7
    let i2c_scl = gpiob.pb6.into_open_drain_output();
    let i2c_sda = gpiob.pb7.into_open_drain_output();

    // configure the i2c
    let i2c = dp.I2C1.i2c(i2c_sda, i2c_scl, 100_000.Hz(), &mut rcc);
    // share I2C bus
    let manager = shared_bus::BusManagerSimple::new(i2c);

    // configure Interrupts
    let line_acc_int = GpioLine::from_raw_line(accel_int.pin_number()).unwrap();
    let line_button = GpioLine::from_raw_line(button.pin_number()).unwrap();
    let _line_bat_gauge_int = GpioLine::from_raw_line(battery_gauge_int.pin_number()).unwrap();
    // also interrupt for pm_chg_present and pm_chg_charging
    let line_pm_chg_present = GpioLine::from_raw_line(pm_chg_present.pin_number()).unwrap();
    let _line_pm_chg_charging = GpioLine::from_raw_line(pm_chg_charging.pin_number()).unwrap();

    exti.listen_gpio(
        &mut syscfg,
        accel_int.port(),
        line_acc_int,
        TriggerEdge::Rising,
    );

    exti.listen_gpio(&mut syscfg, button.port(), line_button, TriggerEdge::Both);

    // exti.listen_gpio(
    //     &mut syscfg,
    //     battery_gauge_int.port(),
    //     line_bat_gauge_int,
    //     TriggerEdge::Both,
    // );

    exti.listen_gpio(
        &mut syscfg,
        pm_chg_present.port(),
        line_pm_chg_present,
        TriggerEdge::Both,
    );

    // exti.listen_gpio(
    //     &mut syscfg,
    //     pm_chg_charging.port(),
    //     line_pm_chg_charging,
    //     TriggerEdge::Both,
    // );

    // exti.listen_direct(stm32l0xx_hal::exti::DirectLine::Lptim1);

    // create the accelerometer
    let mut accelerometer = match configure_lis3dh(manager.acquire_i2c()) {
        Ok(lis3dh) => Some(lis3dh),
        Err(_e) => {
            // show 0b00001 on leds
            leds[0].set_high().unwrap();
            cortex_m::asm::delay(10_000_000);
            // reset mcu
            cortex_m::peripheral::SCB::sys_reset();
        }
    };

    // create the battery gauge
    let mut max17048 = Max17048::new(manager.acquire_i2c());

    // create the timer
    let mut timer = dp.TIM2.timer(20.Hz(), &mut rcc);
    timer.listen();

    // give the button
    cortex_m::interrupt::free(|cs| {
        BTN_GPIO.borrow(cs).replace(Some(button.downgrade()));
        CHARGER_GPIO
            .borrow(cs)
            .replace(Some(pm_chg_present.downgrade()));
        CHARGING_GPIO
            .borrow(cs)
            .replace(Some(pm_chg_charging.downgrade()));
        TIMER.borrow(cs).replace(Some(timer));
    });

    // initialise the states
    let mut state = State::Charging(Some(40));
    let mut wheeling_pattern = WheelingPattern::Snake;
    let mut running_counter: u32 = 0;
    let mut button_menu = ButtonMenu::new();

    let mut has_not_been_setup_since_boot = true;

    let mut soc = unsafe { max17048.soc().unwrap().to_int_unchecked::<u8>() };
    rprintln!("soc: {}", soc);

    // unmask needed interrupt request lines in the NVIC
    unsafe {
        NVIC::unmask(Interrupt::EXTI0_1);
        NVIC::unmask(Interrupt::EXTI2_3);
        NVIC::unmask(Interrupt::EXTI4_15);
        NVIC::unmask(Interrupt::TIM2);
    }

    let mut previous_state = State::Off;
    loop {
        // store locally all IRQ to main events
        let mut acc_int = false;
        let mut button_int = None;
        // let mut bat_gauge_int = false;
        let mut charger_int = None;
        // let mut charging_int = None;
        let mut timer_int = false;

        // check for events
        cortex_m::interrupt::free(|cs| {
            acc_int = ACCEL_INT.borrow(cs).replace(false);
            button_int = BUTTON_INT.borrow(cs).replace(None);
            //     bat_gauge_int = BATTERY_INT.borrow(cs).replace(false);
            charger_int = CHARGER_INT.borrow(cs).replace(None);
            //     charging_int = CHARGING_INT.borrow(cs).replace(None);
            timer_int = PERIODIC_INT.borrow(cs).replace(false);
        });

        // handle the events
        if acc_int {
            rprintln!("acc_int");
            state = State::Wheeling(wheeling_pattern, TIMEOUT);
        }

        if let Some(button) = button_int {
            rprintln!("button: {:?}", button);
            #[cfg(feature = "defmt_enable")]
            defmt::info!("Button event: {:?}", button);
            has_not_been_setup_since_boot = false;
            if let Some(menu_event) = button_menu.process_event(button) {
                //rprintln!("menu event: {:?}", menu_event);
                #[cfg(feature = "defmt_enable")]
                defmt::info!("Menu event: {:?}", menu_event);
                match menu_event {
                    ClickEvent::Long => {
                        // long press: turn off
                        state = State::Off;
                    }
                    ClickEvent::Short => {
                        // if the state is wheeling, show next pattern
                        match state {
                            State::Wheeling(_, rest) => {
                                wheeling_pattern.next();
                                state = State::Wheeling(wheeling_pattern, rest);
                            }
                            State::Charging(_) => {
                                soc = unsafe { max17048.soc().unwrap().to_int_unchecked::<u8>() };
                                state = State::Charging(Some(40));
                            }
                            State::Off => state = State::Wheeling(wheeling_pattern, TIMEOUT),
                            State::Break(_) => {}
                        }
                    }
                    ClickEvent::VeryLong => {
                        soc = unsafe { max17048.soc().unwrap().to_int_unchecked::<u8>() };
                        state = State::Wheeling(WheelingPattern::ShowCharge(40), TIMEOUT);
                    }
                }
            }
        }

        // if bat_gauge_int {
        //     // read the battery gauge
        //     soc = match max17048.soc(){
        //         Ok(s) => s as u8,
        //         Err(_) => 0,
        //     };
        //     #[cfg(feature = "defmt_enable")]
        //     defmt::info!("Battery: {}%", soc);
        // }

        if let Some(event) = charger_int {
            #[cfg(feature = "defmt_enable")]
            defmt::info!("Charger event: {:?}", event);
            match event {
                ChargerEvent::ChargerConnected => {
                    soc = unsafe { max17048.soc().unwrap().to_int_unchecked::<u8>() };
                    rprintln!("charger connected");
                    rprintln!("soc: {}", soc);
                    state = State::Charging(Some(40));
                }
                ChargerEvent::ChargerDisconnected => state = State::Off,
            }
        }

        // if let Some(_event) = charging_int {
        //     #[cfg(feature = "defmt_enable")]
        //     defmt::info!("Charging event: {:?}", _event);
        // }

        if timer_int {
            rprintln!("state: {:?}", state);
            #[cfg(feature = "defmt_enable")]
            defmt::trace!("Timer event {}", running_counter);
            running_counter = running_counter.wrapping_add(1);
            if button_menu.tick().is_some() {
                // turn off the leds for this tick
            }
            if let State::Wheeling(_, _) = state {
                if let Some(ref mut acc) = accelerometer {
                    match acc.accel_norm() {
                        Ok(acceleration) => {
                            // if the acceleration is high enough, change the state
                            if acceleration.z > 0.10 && acceleration.z < 0.95 {
                                rprintln!("breaking");
                                state = State::Break(10);
                            }
                        }
                        Err(e) => {
                            rprintln!("acceleration error: {:?}", e);
                        }
                    }
                }
            }
            // state self update
            state = match state {
                State::Off => State::Off,
                State::Wheeling(pattern, time_left) => {
                    if time_left == 0 {
                        State::Off
                    } else {
                        match pattern {
                            WheelingPattern::ShowCharge(show_charging_time_left) => {
                                if show_charging_time_left == 0 {
                                    State::Wheeling(wheeling_pattern, TIMEOUT)
                                } else {
                                    State::Wheeling(
                                        WheelingPattern::ShowCharge(show_charging_time_left - 1),
                                        TIMEOUT,
                                    )
                                }
                            }
                            x => State::Wheeling(x, time_left - 1),
                        }
                    }
                }
                State::Break(time_left) => {
                    if time_left == 0 {
                        State::Wheeling(wheeling_pattern, TIMEOUT)
                    } else {
                        State::Break(time_left - 1)
                    }
                }
                State::Charging(x) => {
                    if let Some(time_left) = x {
                        if time_left == 0 {
                            State::Charging(None)
                        } else {
                            State::Charging(Some(time_left - 1))
                        }
                    } else {
                        State::Charging(None)
                    }
                }
            };

            // displaying according to the state
            match state {
                State::Off => {
                    // turn off all leds
                    leds.iter_mut().for_each(|l| l.set_low().unwrap());
                }
                State::Wheeling(pattern, _) => match pattern {
                    WheelingPattern::Snake => {
                        // turn all leds off except running_counter % 5
                        leds.iter_mut().for_each(|l| l.set_low().unwrap());
                        leds[(running_counter / 2) as usize % 5].set_high().unwrap();
                        if has_not_been_setup_since_boot {
                            // turn on led 0
                            leds[0].set_high().unwrap();
                        }
                    }
                    WheelingPattern::Blink10Hz => {
                        // blink all leds at running_counter % 10 == 0
                        if running_counter % 10 == 0 {
                            leds.iter_mut().for_each(|l| l.set_high().unwrap());
                        } else {
                            leds.iter_mut().for_each(|l| l.set_low().unwrap());
                        }
                    }
                    WheelingPattern::SolidOn => {
                        // turn all leds on
                        leds.iter_mut().for_each(|l| l.set_high().unwrap());
                    }
                    WheelingPattern::ShowCharge(charging_counter) => {
                        leds.iter_mut().for_each(|l| l.set_low().unwrap());
                        let soc_slice = (soc.min(99) as usize) / 20;
                        for i in 0..soc_slice {
                            leds[4 - i].set_high().unwrap();
                        }
                        // the last one is turned on x/20th of the time according to the content of the slice
                        let last_slice_value = (soc.min(99)) % 20;
                        if 20 - (charging_counter % 20) < last_slice_value as u32 {
                            leds[4 - soc_slice].set_high().unwrap();
                        }
                    }
                },
                State::Break(_) => {
                    // turn all leds on
                    leds.iter_mut().for_each(|l| l.set_high().unwrap());
                }
                State::Charging(x) => {
                    if let Some(charging_counter) = x {
                        leds.iter_mut().for_each(|l| l.set_low().unwrap());
                        let soc_slice = (soc.min(99) as usize) / 20;
                        for i in 0..soc_slice {
                            leds[4 - i].set_high().unwrap();
                        }
                        // the last one is turned on x/20th of the time according to the content of the slice
                        let last_slice_value = (soc.min(99)) % 20;
                        if 20 - (charging_counter % 20) < last_slice_value as u32 {
                            leds[4 - soc_slice].set_high().unwrap();
                        }
                    }
                    // turn of leds and turn off 5V
                    else {
                        leds.iter_mut().for_each(|l| l.set_low().unwrap());
                    }
                }
            };
        }
        // if this is the first iteration as State::Off
        if previous_state != State::Off && state == State::Off {
            cortex_m::interrupt::free(|cs| {
                if let Some(ref mut timer) = &mut *TIMER.borrow(cs).borrow_mut() {
                    pm5v_en.set_low().unwrap();
                    // turn off the leds
                    leds.iter_mut().for_each(|l| l.set_low().unwrap());
                    timer.pause();
                }
            });
        }
        // if previous state was State::Off and this is not
        if previous_state == State::Off && state != State::Off {
            cortex_m::interrupt::free(|cs| {
                if let Some(ref mut timer) = &mut *TIMER.borrow(cs).borrow_mut() {
                    pm5v_en.set_high().unwrap();
                    leds.iter_mut().for_each(|l| l.set_low().unwrap());
                    timer.resume();
                }
            });
        }
        previous_state = state;

        #[cfg(not(feature = "no-sleep"))]
        pwr.sleep_mode(&mut cp.SCB).enter();
        // pwr.stop_mode(
        //     &mut scb,
        //     &mut rcc,
        //     StopModeConfig {
        //         ultra_low_power: true,
        //     },
        // )
        // .enter();
        // #[cfg(feature = "no-sleep")]
    }
}

// this is the interrupt handler for the accelerometer
#[interrupt]
fn EXTI0_1() {
    cortex_m::interrupt::free(|cs| {
        // check if the interrupt is for the accelerometer
        if Exti::is_pending(GpioLine::from_raw_line(1).unwrap()) {
            // clear the interrupt
            Exti::unpend(GpioLine::from_raw_line(1).unwrap());
            // notify main thread with mutex
            ACCEL_INT.borrow(cs).set(true);
        }
    });
}

#[interrupt]
fn EXTI2_3() {
    rprintln!("EXTI2_3");
    cortex_m::interrupt::free(|cs| {
        // check if the interrupt is for the fuel gauge
        if Exti::is_pending(GpioLine::from_raw_line(2).unwrap()) {
            // clear the interrupt
            Exti::unpend(GpioLine::from_raw_line(2).unwrap());
            // notify main thread with mutex
            BATTERY_INT.borrow(cs).set(true);
        }
        // check if the interrupt is for the button
        if Exti::is_pending(GpioLine::from_raw_line(3).unwrap()) {
            // clear the interrupt
            Exti::unpend(GpioLine::from_raw_line(3).unwrap());
            if let Some(ref mut btn) = &mut *BTN_GPIO.borrow(cs).borrow_mut() {
                BUTTON_INT.borrow(cs).set(if btn.is_low().unwrap() {
                    Some(ButtonEvent::Pressed)
                } else {
                    Some(ButtonEvent::Released)
                });
            }
        }
    });
}

#[interrupt]
fn EXTI4_15() {
    cortex_m::interrupt::free(|cs| {
        // check if the interrupt is for the Charger
        if Exti::is_pending(GpioLine::from_raw_line(9).unwrap()) {
            // read pin
            if let Some(ref mut charger) = &mut *CHARGER_GPIO.borrow(cs).borrow_mut() {
                CHARGER_INT.borrow(cs).set(if charger.is_low().unwrap() {
                    cortex_m::peripheral::SCB::sys_reset();
                } else {
                    Some(ChargerEvent::ChargerDisconnected)
                });
            }
            // clear the interrupt
            Exti::unpend(GpioLine::from_raw_line(9).unwrap());
        }
        // check if the interrupt is for the Charging
        if Exti::is_pending(GpioLine::from_raw_line(10).unwrap()) {
            // read pin
            if let Some(ref mut charging) = &mut *CHARGING_GPIO.borrow(cs).borrow_mut() {
                CHARGING_INT.borrow(cs).set(if charging.is_low().unwrap() {
                    Some(ChargingEvent::Charging)
                } else {
                    Some(ChargingEvent::Discharging)
                });
            }
            // clear the interrupt
            Exti::unpend(GpioLine::from_raw_line(10).unwrap());
        }
    });
}

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut timer) = &mut *TIMER.borrow(cs).borrow_mut() {
            // Clear the interrupt flag.
            timer.clear_irq();
            PERIODIC_INT.borrow(cs).set(true);
        }
    });
}

#[panic_handler] // built-in ("core") attribute
fn core_panic(_info: &PanicInfo) -> ! {
    //defmt::error!("PANIC: {}", defmt::Debug2Format(_info)); // e.g. using RTT
    cortex_m::peripheral::SCB::sys_reset()
}

// configure the accelerometer and return a result.
// T: inner type for
fn configure_lis3dh<T, E>(bus: T) -> Result<Lis3dh<Lis3dhI2C<T>>, lis3dh::Error<E, Infallible>>
where
    T: embedded_hal::blocking::i2c::WriteRead<Error = E>
        + embedded_hal::blocking::i2c::Write<Error = E>,
{
    let mut device = Lis3dh::new_i2c(bus, SlaveAddr::Alternate)?;
    device.set_datarate(lis3dh::DataRate::Hz_400)?;
    let threshold = lis3dh::Threshold::g(lis3dh::Range::default(), 1.6);
    rprintln!("threshold: {:?}", threshold);
    device.configure_irq_threshold(lis3dh::Interrupt1, threshold)?;
    // The time in 1/ODR an axis value should be above threshold in order for an
    // interrupt to be raised
    let duration = lis3dh::Duration::miliseconds(lis3dh::DataRate::Hz_400, 0.0); // TODO: find the correct value
    device.configure_irq_duration(lis3dh::Interrupt1, duration)?;

    // Congfigure IRQ source for interrupt 1
    device.configure_irq_src(
        lis3dh::Interrupt1,
        lis3dh::InterruptMode::OrCombination, // unsure about this? i did not use it previously (OrCombination)
        lis3dh::InterruptConfig::high(),      // CFG ZHIE XHIE YHIE
    )?;

    // Configure IRQ pin 1
    device.configure_interrupt_pin(lis3dh::IrqPin1Config {
        // Raise if interrupt 1 is raised
        ia1_en: true,
        // Disable for all other interrupts
        ..lis3dh::IrqPin1Config::default()
    })?;

    // Go to low power mode and wake up for 25ms if measurement above 1.1g is done
    let duration = lis3dh::Duration::miliseconds(lis3dh::DataRate::Hz_400, 2.5);
    device.configure_switch_to_low_power(threshold, duration)?;
    device.set_datarate(lis3dh::DataRate::Hz_400)?;
    Ok(device)
}

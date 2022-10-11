#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use core::panic::PanicInfo;

use accelerometer::Accelerometer;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;

#[cfg(feature = "defmt_enable")]
use defmt::Format;
#[cfg(feature = "defmt_enable")]
use defmt_rtt as _;
use max170xx::Max17048;
//use panic_probe as _; // global logger

use lis3dh::{Lis3dh, SlaveAddr};
use stm32l0xx_hal::lptim::{self, LpTimer};
use stm32l0xx_hal::pwr::StopModeConfig;
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
#[derive(Copy, Clone, PartialEq, Eq)]
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
#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Off,
    Wheeling(WheelingPattern, u32),
    Break(u32),
    Charging(Option<u32>),
}

// button event
#[cfg_attr(feature = "defmt_enable", derive(Format))]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ButtonEvent {
    Pressed,
    Released,
}

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
static CHARGER_GPIO: Mutex<RefCell<Option<Pin<Input<Floating>>>>> = Mutex::new(RefCell::new(None));
// charging state shared pin
static CHARGING_GPIO: Mutex<RefCell<Option<Pin<Input<Floating>>>>> = Mutex::new(RefCell::new(None));
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

// button state machine, to detect short click, long click, double click
enum InternalButtonState {
    Idle,
    DuringFirstClick,
    PotentiallyBetweenClicks,
    DuringSecondClick,
    LongClick,
}
enum ClickEvent {
    Short,
    Long,
    Double,
}

struct ButtonMenu {
    state: InternalButtonState,
    last_event_counter: u32,
}

impl ButtonMenu {
    fn new() -> Self {
        Self {
            state: InternalButtonState::Idle,
            last_event_counter: 0,
        }
    }

    // tick
    fn tick(&mut self) {
        self.last_event_counter = self.last_event_counter.wrapping_add(1);
    }

    // process a button event.
    // short click: less than 60 ticks
    // double click: two short clicks in less than 200 ticks
    // long click: more than 200 ticks.
    // return the event, or None if no event.
    fn process_event(&mut self, ev: ButtonEvent) -> Option<ClickEvent> {
        match self.state {
            InternalButtonState::Idle => {
                if ev == ButtonEvent::Pressed {
                    self.state = InternalButtonState::DuringFirstClick;
                    self.last_event_counter = 0;
                }
            }
            InternalButtonState::DuringFirstClick => {
                if ev == ButtonEvent::Released {
                    if self.last_event_counter < 60 {
                        self.state = InternalButtonState::PotentiallyBetweenClicks;
                        self.last_event_counter = 0;
                        return Some(ClickEvent::Short);
                    } else {
                        self.state = InternalButtonState::LongClick;
                        self.last_event_counter = 0;
                        return Some(ClickEvent::Long);
                    }
                }
            }
            InternalButtonState::PotentiallyBetweenClicks => {
                if ev == ButtonEvent::Pressed {
                    self.state = InternalButtonState::DuringSecondClick;
                    self.last_event_counter = 0;
                }
            }
            InternalButtonState::DuringSecondClick => {
                if ev == ButtonEvent::Released {
                    if self.last_event_counter < 60 {
                        self.state = InternalButtonState::Idle;
                        self.last_event_counter = 0;
                        return Some(ClickEvent::Double);
                    } else {
                        self.state = InternalButtonState::LongClick;
                        self.last_event_counter = 0;
                        return Some(ClickEvent::Long);
                    }
                }
            }
            InternalButtonState::LongClick => {
                if ev == ButtonEvent::Released {
                    self.state = InternalButtonState::Idle;
                    self.last_event_counter = 0;
                }
            }
        }
        None
    }
}

const TIMEOUT: u32 = 18_000;

#[entry]
fn main() -> ! {
    #[cfg(feature = "defmt_enable")]
    defmt::info!("bike-lights-sw");
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());
    let mut exti = Exti::new(dp.EXTI);
    let mut pwr = PWR::new(dp.PWR, &mut rcc);
    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);
    let mut scb = cp.SCB;

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
    let pm_chg_present = gpioa.pa9.into_floating_input();

    // PM_CHG_CHARGING on pa10, floating input
    let pm_chg_charging = gpioa.pa10.into_floating_input();

    // i2c pins are: scl on pb6, sda on pb7
    let scl = gpiob.pb6.into_open_drain_output();
    let sda = gpiob.pb7.into_open_drain_output();

    // configure the i2c
    let i2c = dp.I2C1.i2c(sda, scl, 100_000.Hz(), &mut rcc);
    // share I2C bus
    let manager = shared_bus::BusManagerSimple::new(i2c);

    // configure Interrupts
    let line_acc_int = GpioLine::from_raw_line(accel_int.pin_number()).unwrap();
    let line_button = GpioLine::from_raw_line(button.pin_number()).unwrap();
    let line_bat_gauge_int = GpioLine::from_raw_line(battery_gauge_int.pin_number()).unwrap();
    // also interrupt for pm_chg_present and pm_chg_charging
    let line_pm_chg_present = GpioLine::from_raw_line(pm_chg_present.pin_number()).unwrap();
    let line_pm_chg_charging = GpioLine::from_raw_line(pm_chg_charging.pin_number()).unwrap();

    exti.listen_gpio(
        &mut syscfg,
        accel_int.port(),
        line_acc_int,
        TriggerEdge::Rising,
    );

    exti.listen_gpio(&mut syscfg, button.port(), line_button, TriggerEdge::Both);

    exti.listen_gpio(
        &mut syscfg,
        battery_gauge_int.port(),
        line_bat_gauge_int,
        TriggerEdge::Both,
    );

    exti.listen_gpio(
        &mut syscfg,
        pm_chg_present.port(),
        line_pm_chg_present,
        TriggerEdge::Both,
    );

    exti.listen_gpio(
        &mut syscfg,
        pm_chg_charging.port(),
        line_pm_chg_charging,
        TriggerEdge::Both,
    );

    exti.listen_direct(stm32l0xx_hal::exti::DirectLine::Lptim1);

    // create the accelerometer
    let mut lis3dh = Lis3dh::new_i2c(manager.acquire_i2c(), SlaveAddr::Alternate).unwrap();
    lis3dh.set_datarate(lis3dh::DataRate::Hz_400).unwrap();
    let threshold = lis3dh::Threshold::g(lis3dh::Range::default(), 1.6);
    lis3dh
        .configure_irq_threshold(lis3dh::Interrupt1, threshold)
        .unwrap();
    // The time in 1/ODR an axis value should be above threshold in order for an
    // interrupt to be raised
    let duration = lis3dh::Duration::miliseconds(lis3dh::DataRate::Hz_400, 0.0); // TODO: find the correct value
    lis3dh
        .configure_irq_duration(lis3dh::Interrupt1, duration)
        .unwrap();

    // Congfigure IRQ source for interrupt 1
    lis3dh
        .configure_irq_src(
            lis3dh::Interrupt1,
            lis3dh::InterruptMode::Movement, // unsure about this? i did not use it previously (OrCombination)
            lis3dh::InterruptConfig::high(), // CFG ZHIE XHIE YHIE
        )
        .unwrap();

    // Configure IRQ pin 1
    lis3dh
        .configure_interrupt_pin(lis3dh::IrqPin1Config {
            // Raise if interrupt 1 is raised
            ia1_en: true,
            // Disable for all other interrupts
            ..lis3dh::IrqPin1Config::default()
        })
        .unwrap();

    // Go to low power mode and wake up for 25ms if measurement above 1.1g is done
    let duration = lis3dh::Duration::miliseconds(lis3dh::DataRate::Hz_400, 2.5);
    lis3dh
        .configure_switch_to_low_power(threshold, duration)
        .unwrap();
    lis3dh.set_datarate(lis3dh::DataRate::Hz_400).unwrap();

    // create the battery gauge
    let mut max17048 = Max17048::new(manager.acquire_i2c());

    // create the timer
    let mut timer = LpTimer::init_periodic(
        dp.LPTIM,
        &mut pwr,
        &mut rcc,
        stm32l0xx_hal::lptim::ClockSrc::Lsi,
    );
    timer.enable_interrupts(lptim::Interrupts {
        autoreload_match: true,
        ..lptim::Interrupts::default()
    });
    // give the button
    cortex_m::interrupt::free(|cs| {
        BTN_GPIO.borrow(cs).replace(Some(button.downgrade()));
        CHARGER_GPIO
            .borrow(cs)
            .replace(Some(pm_chg_present.downgrade()));
        CHARGING_GPIO
            .borrow(cs)
            .replace(Some(pm_chg_charging.downgrade()));
    });

    timer.start(200.Hz());

    // initialise the states
    let mut state = State::Off;
    let mut wheeling_pattern = WheelingPattern::Snake;
    let mut running_counter: u32 = 0;
    let mut button_menu = ButtonMenu::new();

    let mut has_not_been_setup_since_boot = true;
    // unmask needed interrupt request lines in the NVIC
    unsafe {
        NVIC::unmask(Interrupt::EXTI0_1);
        NVIC::unmask(Interrupt::EXTI2_3);
        NVIC::unmask(Interrupt::EXTI4_15);
        NVIC::unmask(Interrupt::LPTIM1);
    }
    loop {
        // store locally all IRQ to main events
        let mut acc_int = false;
        let mut button_int = None;
        let mut bat_gauge_int = false;
        let mut charger_int = None;
        let mut charging_int = None;
        let mut timer_int = false;

        // check for events
        cortex_m::interrupt::free(|cs| {
            acc_int = ACCEL_INT.borrow(cs).replace(false);
            button_int = BUTTON_INT.borrow(cs).replace(None);
            bat_gauge_int = BATTERY_INT.borrow(cs).replace(false);
            charger_int = CHARGER_INT.borrow(cs).replace(None);
            charging_int = CHARGING_INT.borrow(cs).replace(None);
            timer_int = PERIODIC_INT.borrow(cs).replace(false);
        });

        // handle the events
        if acc_int {
            // read the accelerometer
            let acc = lis3dh.accel_norm();
            if let Ok(acc) = acc {
                // if the downwards acceleration is above 1.6g, there was a shock so we are wheeling
                if acc.y > 1.6 {
                    // if we were not wheeling before, start the wheeling pattern
                    state = State::Wheeling(wheeling_pattern, TIMEOUT);
                }
                // if we are breaking
                if acc.z < -0.6 {
                    // if we were not breaking before, start the breaking pattern
                    state = State::Break(200);
                }
            }
        }

        if let Some(button) = button_int {
            #[cfg(feature = "defmt_enable")]
            defmt::info!("Button event: {:?}", button);
            has_not_been_setup_since_boot = false;
            if let Some(menu_event) = button_menu.process_event(button) {
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
                                state = State::Charging(Some(400));
                            }
                            State::Off => state = State::Wheeling(wheeling_pattern, TIMEOUT),
                            _ => {}
                        }
                    }
                    ClickEvent::Double => {
                        state = State::Wheeling(WheelingPattern::ShowCharge(400), TIMEOUT);
                    }
                }
            }
        }

        if bat_gauge_int {
            // read the battery gauge
            let soc = max17048.soc().unwrap() as u8;
            #[cfg(feature = "defmt_enable")]
            defmt::info!("Battery: {}%", soc);
        }

        if let Some(event) = charger_int {
            #[cfg(feature = "defmt_enable")]
            defmt::info!("Charger event: {:?}", event);
            match event {
                ChargerEvent::ChargerConnected => state = State::Charging(Some(400)),
                ChargerEvent::ChargerDisconnected => state = State::Off,
            }
        }

        if let Some(event) = charging_int {
            #[cfg(feature = "defmt_enable")]
            defmt::info!("Charging event: {:?}", event);
        }

        if timer_int {
            #[cfg(feature = "defmt_enable")]
            defmt::trace!("Timer event {}", running_counter);
            running_counter = running_counter.wrapping_add(1);
            button_menu.tick();
        }

        // displaying according to the state

        match state {
            State::Off => {
                pm5v_en.set_low().unwrap();
                // turn off all leds
                leds.iter_mut().for_each(|l| l.set_low().unwrap());
            }
            State::Wheeling(pattern, rest) => match pattern {
                WheelingPattern::Snake => {
                    // turn all leds off except running_counter % 5
                    pm5v_en.set_high().unwrap();
                    leds.iter_mut().for_each(|l| l.set_low().unwrap());
                    leds[running_counter as usize % 5].set_high().unwrap();
                    if has_not_been_setup_since_boot {
                        // turn on led 0
                        leds[0].set_high().unwrap();
                    }
                }
                WheelingPattern::Blink10Hz => {
                    // blink all leds at running_counter % 10 == 0
                    pm5v_en.set_high().unwrap();
                    if running_counter % 10 == 0 {
                        leds.iter_mut().for_each(|l| l.set_high().unwrap());
                    } else {
                        leds.iter_mut().for_each(|l| l.set_low().unwrap());
                    }
                }
                WheelingPattern::SolidOn => {
                    // turn all leds on
                    pm5v_en.set_high().unwrap();
                    leds.iter_mut().for_each(|l| l.set_high().unwrap());
                }
                WheelingPattern::ShowCharge(_) => {
                    // show state of charge
                    let soc = max17048.soc().unwrap() as u8;
                    pm5v_en.set_high().unwrap();
                    leds.iter_mut().for_each(|l| l.set_low().unwrap());
                    for i in 0..soc / 20 {
                        leds[i as usize].set_high().unwrap();
                    }
                    // decrement the counter, when it reaches 0, go back to wheeling
                    if let WheelingPattern::ShowCharge(mut counter) = pattern {
                        counter -= 1;
                        if counter == 0 {
                            state = State::Wheeling(wheeling_pattern, rest);
                        } else {
                            state = State::Wheeling(WheelingPattern::ShowCharge(counter), rest);
                        }
                    }
                }
            },
            State::Break(_) => {
                // turn all leds on
                pm5v_en.set_high().unwrap();
                leds.iter_mut().for_each(|l| l.set_high().unwrap());
                // decrement the counter, when it reaches 0, go back to wheeling
                if let State::Break(mut counter) = state {
                    counter -= 1;
                    if counter == 0 {
                        state = State::Wheeling(wheeling_pattern, TIMEOUT);
                    } else {
                        state = State::Break(counter);
                    }
                }
            }
            State::Charging(_) => {
                // started chargin, so blink 5 times in 400 ticks (u32 enum), then go to Charging(None)
                if let State::Charging(Some(mut counter)) = state {
                    counter -= 1;
                    if counter == 0 {
                        state = State::Charging(None);
                    } else {
                        state = State::Charging(Some(counter));
                    }
                    // blink from 0 to 40, then 80 to 120, then 160 to 200, then 240 to 280, then 320 to 360
                    // and show state of charge
                    pm5v_en.set_high().unwrap();
                    let soc = max17048.soc().unwrap() as u8;
                    leds.iter_mut().for_each(|l| l.set_low().unwrap());
                    // only turn on if in the range set before
                    if counter % 40 < 40 {
                        for i in 0..soc / 20 {
                            leds[i as usize].set_high().unwrap();
                        }
                    }
                }
                // turn of leds and turn off 5V
                else {
                    pm5v_en.set_low().unwrap();
                    leds.iter_mut().for_each(|l| l.set_low().unwrap());
                }
            }
        };

        #[cfg(not(feature = "no-sleep"))]
        pwr.stop_mode(
            &mut scb,
            &mut rcc,
            StopModeConfig {
                ultra_low_power: true,
            },
        )
        .enter();
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
            if let Some(ref mut btn) = BTN_GPIO.borrow(cs).borrow_mut().deref_mut() {
                BUTTON_INT.borrow(cs).set(match btn.is_low().unwrap() {
                    true => Some(ButtonEvent::Pressed),
                    false => Some(ButtonEvent::Released),
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
            if let Some(ref mut charger) = CHARGER_GPIO.borrow(cs).borrow_mut().deref_mut() {
                CHARGER_INT.borrow(cs).set(match charger.is_low().unwrap() {
                    true => Some(ChargerEvent::ChargerConnected),
                    false => Some(ChargerEvent::ChargerDisconnected),
                });
            }
            // clear the interrupt
            Exti::unpend(GpioLine::from_raw_line(9).unwrap());
        }
        // check if the interrupt is for the Charging
        if Exti::is_pending(GpioLine::from_raw_line(10).unwrap()) {
            // read pin
            if let Some(ref mut charging) = CHARGING_GPIO.borrow(cs).borrow_mut().deref_mut() {
                CHARGING_INT
                    .borrow(cs)
                    .set(match charging.is_low().unwrap() {
                        true => Some(ChargingEvent::Charging),
                        false => Some(ChargingEvent::Discharging),
                    });
            }
            // clear the interrupt
            Exti::unpend(GpioLine::from_raw_line(10).unwrap());
        }
    });
}

#[interrupt]
fn LPTIM1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            // Clear the interrupt flag.
            timer.clear_irq();
            // tell main thread that a periodic irq has happened
            PERIODIC_INT.borrow(cs).set(true);
        }
    });
}

#[panic_handler] // built-in ("core") attribute
fn core_panic(_info: &PanicInfo) -> ! {
    #[cfg(feature = "defmt_enable")]
    {
        defmt::error!("PANIC: {}", defmt::Debug2Format(_info)); // e.g. using RTT
    }
    loop {}
}

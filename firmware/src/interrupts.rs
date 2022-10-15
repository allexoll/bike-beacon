pub mod interrupts {

    use core::cell::{Cell, RefCell};
    use core::convert::Infallible;
    use core::panic::PanicInfo;

    use accelerometer::Accelerometer;
    use cortex_m::interrupt::Mutex;
    use cortex_m::peripheral::NVIC;

    use max170xx::Max17048;

    use lis3dh::{Lis3dh, Lis3dhI2C, SlaveAddr};
    use rtt_target::rprintln;
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

    use crate::{ChargerEvent, ChargingEvent, ButtonEvent};
    // main to irq

    // Button GPIO shared pin
    pub static BTN_GPIO: Mutex<RefCell<Option<Pin<Input<PullUp>>>>> =
        Mutex::new(RefCell::new(None));
    // charger shared pin
    pub static CHARGER_GPIO: Mutex<RefCell<Option<Pin<Input<PullUp>>>>> =
        Mutex::new(RefCell::new(None));
    // charging state shared pin
    pub static CHARGING_GPIO: Mutex<RefCell<Option<Pin<Input<PullUp>>>>> =
        Mutex::new(RefCell::new(None));
    // shared timer for the timeout
    pub static TIMER: Mutex<RefCell<Option<Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));

    // IRQ to main

    // static mutexes to hold the interrupts events notification for the main thread
    pub static ACCEL_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
    pub static BUTTON_INT: Mutex<Cell<Option<ButtonEvent>>> = Mutex::new(Cell::new(None));
    pub static BATTERY_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
    // and for charger present/ charging state
    pub static CHARGER_INT: Mutex<Cell<Option<ChargerEvent>>> = Mutex::new(Cell::new(None));
    pub static CHARGING_INT: Mutex<Cell<Option<ChargingEvent>>> = Mutex::new(Cell::new(None));
    pub static PERIODIC_INT: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

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
}

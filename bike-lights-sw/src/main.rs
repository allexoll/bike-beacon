#![no_std]
#![no_main]

// pick a panicking behavior
use core::panic::PanicInfo;
use cortex_m_rt::entry;

use accelerometer::{RawAccelerometer, Tracker};
use lis3dh::{Lis3dh, SlaveAddr};
use rtt_target::{rprintln, rtt_init_print};
use stm32l0xx_hal::{
    exti::{Exti, ExtiLine, GpioLine, TriggerEdge},
    pac,
    prelude::*,
    pwr::{self, PWR},
    rcc::Config,
    syscfg::SYSCFG,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut exti = Exti::new(dp.EXTI);
    let mut pwr = PWR::new(dp.PWR, &mut rcc);
    let mut delay = cp.SYST.delay(rcc.clocks);
    let mut scb = cp.SCB;

    let acc_int_1 = gpioa.pa1.into_floating_input();
    let acc_int_2 = gpioa.pa2.into_floating_input();

    let mut pm_5v_en = gpiob.pb1.into_push_pull_output();

    let line_acc_int_1 = GpioLine::from_raw_line(acc_int_1.pin_number()).unwrap();
    let line_acc_int_2 = GpioLine::from_raw_line(acc_int_2.pin_number()).unwrap();

    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);

    let sda = gpiob.pb7.into_open_drain_output();
    let scl = gpiob.pb6.into_open_drain_output();
    let i2c = dp.I2C1.i2c(sda, scl, 100.khz(), &mut rcc);

    let mut lis3dh = Lis3dh::new(i2c, SlaveAddr::Alternate).unwrap();
    exti.listen_gpio(
        &mut syscfg,
        acc_int_1.port(),
        line_acc_int_1,
        TriggerEdge::Rising,
    );
    exti.listen_gpio(
        &mut syscfg,
        acc_int_2.port(),
        line_acc_int_2,
        TriggerEdge::Rising,
    );
    let mut tracker = Tracker::new(3700.0);
    loop {
        exti.wait_for_irq(
            line_acc_int_1,
            pwr.stop_mode(
                &mut scb,
                &mut rcc,
                pwr::StopModeConfig {
                    ultra_low_power: true,
                },
            ),
        );
        pm_5v_en.set_high().unwrap();
        delay.delay_ms(100u32);
        let accel = lis3dh.accel_raw().unwrap();
        let orientation = tracker.update(accel);
        rprintln!("{:?}", orientation);
        pm_5v_en.set_low().unwrap();
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}

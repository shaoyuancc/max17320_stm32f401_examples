#![no_std]
#![no_main]

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use hal::{pac, prelude::*};
use panic_semihosting as _;
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
        let mut delay = cp.SYST.delay(&clocks);

        // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        let gpiob = dp.GPIOB.split();
        let scl = gpiob
            .pb8
            .into_alternate()
            .internal_pull_up(true)
            .set_open_drain();
        let sda = gpiob
            .pb9
            .into_alternate()
            .internal_pull_up(true)
            .set_open_drain();
        let i2c = dp.I2C1.i2c((scl, sda), 400.kHz(), &clocks);

        let mut bat = max17320::MAX17320::new(i2c, 5.0).expect("vl");

        delay.delay_ms(300_u16);

        hprintln!("status: {}", bat.read_status().unwrap()).unwrap();
        hprintln!("capacity: {}mAh", bat.read_capacity().unwrap()).unwrap();
        hprintln!("device name: {}", bat.read_device_name().unwrap()).unwrap();
        hprintln!("state of charge: {}%", bat.read_state_of_charge().unwrap()).unwrap();
        hprintln!("vcell: {}v", bat.read_vcell().unwrap()).unwrap();
        hprintln!("cell1: {}v", bat.read_cell1().unwrap()).unwrap();
        hprintln!("temp: {}°C", bat.read_temp().unwrap()).unwrap();
        hprintln!("die temp: {}°C", bat.read_die_temp().unwrap()).unwrap();
        hprintln!("current: {}mA", bat.read_current().unwrap()).unwrap();

        let tte = bat.read_time_to_empty().expect("tte");
        hprintln!("tte: {}", tte).unwrap();

        let ttf = bat.read_time_to_full().expect("ttf");
        hprintln!("ttf: {}", ttf).unwrap();

        let prot_status = bat.read_protection_status().expect("prs");
        hprintln!("prot_status: {}", prot_status).unwrap();

        let prot_alert = bat.read_protection_alert().expect("pra");
        hprintln!("prot_alert: {}", prot_alert).unwrap();

        loop {}
    }

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

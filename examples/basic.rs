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

        let mut bat = max17320::MAX17320::new(i2c, 5.0).expect("mx");

        hprintln!("nPackCfg before: {:b}", bat.read_pack_config().unwrap()).unwrap();

        bat.set_pack_config(
            2,
            0,
            max17320::ThermistorType::Ntc10KOhm,
            max17320::ChargePumpVoltageConfiguration::Cp6V,
            max17320::AlwaysOnRegulatorConfiguration::Disabled,
            max17320::BatteryPackUpdate::UpdateEvery22p4s,
        )
        .expect("cfgr");

        hprintln!("nPackCfg after: {:b}", bat.read_pack_config().unwrap()).unwrap();

        // Alert thresholds
        hprintln!(
            "VAlrtTh before: {:?}",
            bat.read_volatage_alert_threshold().unwrap()
        )
        .unwrap();
        bat.set_voltage_alert_threshold(1.0, 3.0).expect("svat");
        hprintln!(
            "VAlrtTh after: {:?}",
            bat.read_volatage_alert_threshold().unwrap()
        )
        .unwrap();

        hprintln!(
            "TAlrtTh before: {:?}",
            bat.read_temperature_alert_threshold().unwrap()
        )
        .unwrap();
        bat.set_temperature_alert_threshold(-20, 60).expect("stat");
        hprintln!(
            "TAlrtTh after: {:?}",
            bat.read_temperature_alert_threshold().unwrap()
        )
        .unwrap();

        hprintln!("status: {}", bat.read_status().unwrap()).unwrap();
        hprintln!("capacity: {}mAh", bat.read_capacity().unwrap()).unwrap();
        hprintln!("device name: {}", bat.read_device_name().unwrap()).unwrap();
        hprintln!("state of charge: {}%", bat.read_state_of_charge().unwrap()).unwrap();
        hprintln!("vcell: {}v", bat.read_vcell().unwrap()).unwrap();
        hprintln!("cell1: {}v", bat.read_cell1().unwrap()).unwrap();
        hprintln!("temp: {}°C", bat.read_temperature().unwrap()).unwrap();
        hprintln!("die temp: {}°C", bat.read_die_temperature().unwrap()).unwrap();
        hprintln!("current: {}mA", bat.read_current().unwrap()).unwrap();
        hprintln!("tte: {}", bat.read_time_to_empty().unwrap()).unwrap();
        hprintln!("ttf: {}", bat.read_time_to_full().unwrap()).unwrap();
        hprintln!("prot_status: {}", bat.read_protection_status().unwrap()).unwrap();
        hprintln!("prot_alert: {}", bat.read_protection_alert().unwrap()).unwrap();
    }
    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

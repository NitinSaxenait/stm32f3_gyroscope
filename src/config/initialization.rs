#![no_std]
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use stm32f3_discovery::{
    leds::Leds,
    stm32f3xx_hal::{delay::Delay, prelude},
};

use l3gd20::{L3gd20, Measurements};
use stm32f3_discovery::stm32f3xx_hal::{prelude::*, spi::Spi, stm32};

/// This program is going to provide the initialization to get and print the sensor readings with
/// some delay.
/// #Argument
/// None
///
/// #Return
/// This program is going to return a tuple of l3g-package, delay and itm.
/// l3gd20 -> a package to access the gyroscope sensor to get the readings.
/// Delay -> a time delay to pause the running program for few seconds.
/// ITM -> a tool to print the sensor data on the itm console.
pub fn initialization() -> (Delay, ITM, Measurements) {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let device_peripherals = stm32::Peripherals::take().unwrap();

    let mut flash = device_peripherals.FLASH.constrain();
    let mut rcc = device_peripherals.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioe = device_peripherals.GPIOE.split(&mut rcc.ahb);
    let mut gpioa = device_peripherals.GPIOA.split(&mut rcc.ahb);
    let led = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let pin5 = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let pin6 = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let pin7 = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let spi = Spi::spi1(
        device_peripherals.SPI1,
        (pin5, pin6, pin7),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );
    let mut l3 = match L3gd20::new(spi, led) {
        Ok(l3) => l3,
        Err(error) => {
            panic!("Can't connect to the L3gd20 {:?}", error)
        }
    };
    let gyro_readings = match l3.all() {
        Ok(readings) => readings,
        Err(error) => {
            panic!("Cant get the gyro measurements {:?}", error)
        }
    };
    let delay = Delay::new(core_peripherals.SYST, clocks);

    (delay, core_peripherals.ITM, gyro_readings)
}

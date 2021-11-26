#![no_main]
#![no_std]

use stm32f3_Gyroscope::config::initialization::*;
use stm32f3_discovery::stm32f3xx_hal::hal::blocking::delay::DelayMs;

/// This program is going to print the Gyroscope sensor readings(x, y, z-axis) on the itm-dump console with
/// 1 sec of delay.
///
/// #Arguments
/// None
///
/// #Return
/// This is a no_std no_main code which will neither return nor stop.
#[entry]
fn main() -> ! {
    let (mut delay, mut itm, gyro) = initialization();

    loop {
        let x_axis = gyro.gyro.x;
        let y_axis = gyro.gyro.y;
        let z_axis = gyro.gyro.z;
        iprintln!(
            &mut itm.stim[0],
            "Readings \n x= {:?} y= {:?} z= {:?}",
            x_axis,
            y_axis,
            z_axis
        );

        delay.delay_ms(1000u16);
    }
}

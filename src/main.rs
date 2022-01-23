#![no_std]
#![no_main]

use arduino_hal::port::{mode, Pin};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut leds: [Pin<mode::Output>; 4] = [
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
    ];

    loop {
        (0..10).map(|i| i * 100).for_each(|ms| {
            leds.iter_mut().for_each(|led| {
                led.toggle();
                arduino_hal::delay_ms(ms as u16);
            })
        });
        (10..0).map(|i| i * 100).for_each(|ms| {
            leds.iter_mut().for_each(|led| {
                led.toggle();
                arduino_hal::delay_ms(ms as u16);
            })
        });
    }
}

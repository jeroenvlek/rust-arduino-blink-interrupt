#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led0 = pins.d2.into_output();
    let mut led1 = pins.d3.into_output();
    let mut led2 = pins.d4.into_output();
    let mut led3 = pins.d5.into_output();

    loop {
        (0..20).map(|i| i * 100).for_each(|ms| {
            led0.toggle();
            arduino_hal::delay_ms(ms as u16);
            led1.toggle();
            arduino_hal::delay_ms(ms as u16);
            led2.toggle();
            arduino_hal::delay_ms(ms as u16);
            led3.toggle();
            arduino_hal::delay_ms(ms as u16);
        });
        (20..0).map(|i| i * 100).for_each(|ms| {
            led0.toggle();
            arduino_hal::delay_ms(ms as u16);
            led1.toggle();
            arduino_hal::delay_ms(ms as u16);
            led2.toggle();
            arduino_hal::delay_ms(ms as u16);
            led3.toggle();
            arduino_hal::delay_ms(ms as u16);
        });
    }
}

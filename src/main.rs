#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use core::ops::Range;
use core::sync::atomic::{AtomicBool, Ordering};
use arduino_hal::port::{mode, Pin};
use either::*;

static REVERSE: AtomicBool = AtomicBool::new(false);

fn is_reversed() -> bool {
    return avr_device::interrupt::free(|_| { REVERSE.load(Ordering::SeqCst) });
}

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    avr_device::interrupt::free(|_| {
        let current = REVERSE.load(Ordering::SeqCst);
        REVERSE.store(!current, Ordering::SeqCst);
    });
}

fn blink_for_range(range : Range<u16>, leds : &mut[Pin<mode::Output>]) {
    range.map(|i| i * 100).for_each(|ms| {
        let iter = if is_reversed() {
            Left(leds.iter_mut().rev())
        } else {
            Right(leds.iter_mut())
        };
        iter.for_each(|led| {
            led.toggle();
            arduino_hal::delay_ms(ms as u16);
        })
    });
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    pins.d2.into_pull_up_input(); // is this necessary?

    let mut leds: [Pin<mode::Output>; 4] = [
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
    ];

    unsafe { avr_device::interrupt::enable() };

    loop {
        blink_for_range(0..10, &mut leds);
        blink_for_range(10..0, &mut leds);
    }
}

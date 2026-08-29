#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::{
    el::{blink_el, blink_el_ret, blink_el_with_delay},
    gpio::GpioPin::{Pin6, Pin17, Pin21, Pin22, Pin27},
    util::small_delay,
};

mod el;
mod gpio;
mod stub;
mod sys;
mod util;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn blinker() -> ! {
    small_delay();
    let blinkel = blink_el_ret();

    loop {}

    let mut pin21 = gpio::Pin::new(Pin21);
    let mut pin22 = gpio::Pin::new(Pin22);
    let mut pin6 = gpio::Pin::new(Pin6);
    pin21.set_for_output();
    pin22.set_for_output();
    pin6.set_for_output();

    loop {
        pin21.set_low();
        pin22.set_high();
        pin6.set_low();
        small_delay();
        pin21.set_low();
        pin22.set_low();
        pin6.set_high();
        small_delay();
        pin21.set_high();
        pin22.set_low();
        pin6.set_low();
        small_delay();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

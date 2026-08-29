#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

use crate::gpio::GpioPin::{Pin6, Pin17, Pin21, Pin22, Pin27};

mod el;
mod gpio;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn blinker() -> ! {
    let mut leleds = el::ElBlink::new(Pin17, Pin27, Pin22, Pin6);
    leleds.blink();

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
        for _ in 0..2500000 {
            unsafe { asm!("nop") }
        }
        pin21.set_low();
        pin22.set_low();
        pin6.set_high();
        for _ in 0..2500000 {
            unsafe { asm!("nop") }
        }
        pin21.set_high();
        pin22.set_low();
        pin6.set_low();
        for _ in 0..2500000 {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

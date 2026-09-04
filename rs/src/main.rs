#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::{
    el::{blink_el, blink_el_ret, blink_el_with_delay},
    gpio::{
        GPSET0,
        GpioPin::{Pin5, Pin6, Pin12, Pin17, Pin21, Pin22, Pin23, Pin26, Pin27},
    },
    util::{small_delay, wait_cycle},
};

mod el;
mod gpio;
mod irq;
mod stub;
mod sys;
mod util;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn blinker() -> ! {
    irq::irq_init_vectors();
    irq::irq_enable();
    irq::setup_gpio_irq();
    // irq::irq_enable();

    let blinkel = blink_el_ret();

    let mut p17 = gpio::Pin::new(Pin17);
    let mut p27 = gpio::Pin::new(Pin27);
    p17.set_for_output();
    p27.set_for_output();

    loop {
        p17.set_high();
        p27.set_low();
        wait_cycle(300000);
        p17.set_low();
        p27.set_high();
        wait_cycle(300000);
    }

    let mut pin26 = gpio::Pin::new(Pin26);
    let mut pin17 = gpio::Pin::new(Pin17);
    let mut pin12 = gpio::Pin::new(Pin12);
    pin26.set_for_output();
    pin17.set_for_output();
    pin12.set_for_input();
    pin12.set_pull_up();

    let mut prev_l = false;

    loop {
        let l = pin12.get_level();
        if l != prev_l {
            if l {
                pin26.set_high();
            } else {
                pin26.set_low();
            }
            prev_l = l
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

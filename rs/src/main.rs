#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::{
    el::{blink_el, blink_el_ret, blink_el_with_delay},
    gpio::{
        GPSET0,
        GpioPin::{Pin5, Pin6, Pin12, Pin17, Pin21, Pin22, Pin23, Pin26, Pin27},
    },
    ic595::Ic595,
    mini_uart::uart_send_string,
    user::run_user,
    util::{small_delay, wait_cycle},
};

mod el;
mod gpio;
mod ic595;
mod irq;
mod mini_uart;
mod peripherals;
mod stub;
mod sys;
mod table;
mod user;
mod util;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn blinker() -> ! {
    mini_uart::init_mini_uart();
    uart_send_string("Hello, Are!\n");

    irq::irq_init_vectors();
    irq::irq_enable();
    irq::setup_gpio_irq();

    unsafe {
        run_user();
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

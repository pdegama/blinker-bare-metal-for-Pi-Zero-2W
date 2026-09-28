use core::ptr;

use crate::{
    gpio,
    peripherals::{self, AUX_MU_IO_REG, AUX_MU_LSR_REG},
};

pub fn init_mini_uart() {
    let mut p14 = gpio::Pin::new(gpio::GpioPin::Pin14);
    let mut p15 = gpio::Pin::new(gpio::GpioPin::Pin15);

    p14.set_for(gpio::GpioMode::Alt5);
    p15.set_for(gpio::GpioMode::Alt5);

    p14.set_pull_disable();
    p15.set_pull_disable();

    unsafe {
        ptr::write_volatile(peripherals::AUX_ENABLES as *mut u32, 0b1);
        ptr::write_volatile(peripherals::AUX_MU_CNTL_REG as *mut u32, 0b0);
        ptr::write_volatile(peripherals::AUX_MU_IER_REG as *mut u32, 0x0001);
        ptr::write_volatile(peripherals::AUX_MU_LCR_REG as *mut u32, 0b11);
        ptr::write_volatile(peripherals::AUX_MU_MCR_REG as *mut u32, 0b0);
        ptr::write_volatile(peripherals::AUX_MU_BAUD_REG as *mut u32, 270);
        ptr::write_volatile(peripherals::AUX_MU_CNTL_REG as *mut u32, 0b11);
    }

    uart_send(b'\r');
    uart_send(b'\n');
    uart_send(b'\n');
}

pub fn uart_send(c: u8) {
    while !(unsafe { ptr::read_volatile(AUX_MU_LSR_REG as *mut u32) } & 0x20 > 0) {}
    unsafe {
        ptr::write_volatile(AUX_MU_IO_REG as *mut u32, c as u32);
    }
}

pub fn uart_recv() -> u8 {
    while !(unsafe { ptr::read_volatile(AUX_MU_LSR_REG as *mut u32) } & 0x1 > 0) {}
    unsafe { (ptr::read_volatile(AUX_MU_IO_REG as *mut u32) & 0xff) as u8 }
}

pub fn uart_send_string(s: &str) {
    for c in s.bytes() {
        if c == b'\n' {
            uart_send(b'\r');
        }

        uart_send(c);
    }
}

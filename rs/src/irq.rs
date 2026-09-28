use core::ptr;

use crate::{
    gpio::{
        self, GPSET0,
        GpioPin::{self, Pin12, Pin23, Pin26},
    },
    mini_uart::{uart_recv, uart_send, uart_send_string},
    peripherals::{AUX_MU_IIR_REG, PBASE, PerReg},
    table::{self, gpio_handler},
};

const IRQ_BASE: PerReg = PBASE + 0xB000;
const PENDING_IRQS_1: PerReg = IRQ_BASE + 0x204;
const PENDING_IRQS_2: PerReg = IRQ_BASE + 0x208;
const ENABLE_IRQS_1: PerReg = IRQ_BASE + 0x210;
const ENABLE_IRQS_2: PerReg = IRQ_BASE + 0x214;

unsafe extern "C" {
    pub fn irq_init_vectors();
    pub fn irq_enable();
    pub fn irq_disable();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn blink_error_led() -> gpio::Pin {
    let mut p32 = gpio::Pin::new(Pin23);
    p32.set_for_output();
    p32.set_high();
    p32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_irq() {
    let mut p1: u32;
    let mut p2: u32;

    unsafe {
        p1 = ptr::read_volatile(PENDING_IRQS_1 as *mut u32);
        p2 = ptr::read_volatile(PENDING_IRQS_2 as *mut u32);

        if p1 & 1 << 29 > 0 {
            let l = uart_recv();

            if let Some(f) = table::uart_handler {
                f(l);
            }
        }

        if p2 & 3 << 17 > 0 {
            let bits: u64 = (ptr::read_volatile(gpio::GPEDS1 as *mut u32) as u64) << 32
                | (ptr::read_volatile(gpio::GPEDS0 as *mut u32) as u64);

            if let Some(handler) = gpio_handler {
                handler(bits);
            }
        }
    }
}

pub fn setup_gpio_irq() {
    unsafe {
        let v = ptr::read_volatile(ENABLE_IRQS_2 as *mut u32);
        ptr::write_volatile(ENABLE_IRQS_2 as *mut u32, v | 3 << 17);

        let v = ptr::read_volatile(ENABLE_IRQS_1 as *mut u32);
        ptr::write_volatile(ENABLE_IRQS_1 as *mut u32, v | 1 << 29);
    }
}

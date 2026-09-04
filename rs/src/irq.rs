use core::ptr;

use crate::gpio::{
    self, GPSET0,
    GpioPin::{self, Pin12, Pin23, Pin26},
    PBASE,
};

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
pub unsafe extern "C" fn handle_irq() -> gpio::Pin {
    let mut pin26 = gpio::Pin::new(Pin26);
    pin26.set_for_output();
    if !pin26.get_level() {
        pin26.set_high();
    } else {
        pin26.set_low();
    };
    let pin12 = gpio::Pin::new(Pin12);
    pin12.clear_event();

    pin26
}

pub fn setup_gpio_irq() {
    let mut pin12 = gpio::Pin::new(Pin12);
    pin12.set_for_input();
    pin12.set_pull_down();
    pin12.clear_event();
    pin12.set_risining_eg();

    let IRQ_BASE = PBASE + 0xB000;
    let ENABLE_IRQS_2 = IRQ_BASE + 0x214;

    unsafe {
        let v = ptr::read_volatile(ENABLE_IRQS_2 as *mut u32);
        ptr::write_volatile(ENABLE_IRQS_2 as *mut u32, v | 1 << 17);
    }
}

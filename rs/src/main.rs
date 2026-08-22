#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::write_volatile;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn blinker() -> ! {
    let GPFSEL2: u32 = 0x3f200008;
    let GPSET0: u32 = 0x3f20001c;
    let GPCLR0: u32 = 0x3f200028;
    let OUTPUT_PINS: u32 = 1 << 3 | 1 << 6;

    write_volatile(GPFSEL2 as *mut u32, OUTPUT_PINS);

    loop {
        // Set high
        write_volatile(GPSET0 as *mut u32, 1 << 21);
        write_volatile(GPCLR0 as *mut u32, 1 << 22);
        for _ in 0..2500000 {
            unsafe { asm!("nop") }
        }
        // Set low
        write_volatile(GPSET0 as *mut u32, 1 << 22);
        write_volatile(GPCLR0 as *mut u32, 1 << 21);
        for _ in 0..2500000 {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

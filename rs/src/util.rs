use core::arch::asm;

unsafe extern "C" {
    pub fn get_el() -> u64;
    pub fn delay();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn small_delay() {
    for _ in 0..2500000 {
        unsafe { asm!("nop") }
    }
}

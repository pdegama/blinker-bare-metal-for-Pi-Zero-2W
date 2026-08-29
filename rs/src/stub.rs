use crate::{
    el::ElBlink,
    gpio::{self, GpioPin::Pin17},
    util::small_delay,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_stub_led() {
    let mut g22 = gpio::Pin::new(Pin17);
    g22.set_for_output();
    g22.set_high();

    small_delay();
}

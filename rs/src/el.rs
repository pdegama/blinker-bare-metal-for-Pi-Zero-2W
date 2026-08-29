use crate::gpio::{self, GpioPin};

unsafe extern "C" {
    fn get_el() -> u64;
}

pub struct ElBlink {
    el0_led_pin: gpio::Pin,
    el1_led_pin: gpio::Pin,
    el2_led_pin: gpio::Pin,
    el3_led_pin: gpio::Pin,
}

impl ElBlink {
    pub fn new(
        el0: gpio::GpioPin,
        el1: gpio::GpioPin,
        el2: gpio::GpioPin,
        el3: gpio::GpioPin,
    ) -> Self {
        let mut el0g = gpio::Pin::new(el0);
        let mut el1g = gpio::Pin::new(el1);
        let mut el2g = gpio::Pin::new(el2);
        let mut el3g = gpio::Pin::new(el3);

        el0g.set_for_output();
        el1g.set_for_output();
        el2g.set_for_output();
        el3g.set_for_output();

        Self {
            el0_led_pin: el0g,
            el1_led_pin: el1g,
            el2_led_pin: el2g,
            el3_led_pin: el3g,
        }
    }

    pub fn blink(&mut self) {
        let mut el = 0;
        unsafe {
            el = get_el();
        }
        self.el0_led_pin.set_low();
        self.el1_led_pin.set_low();
        self.el2_led_pin.set_low();
        self.el3_led_pin.set_low();
        if el == 0 {
            self.el0_led_pin.set_high();
        }
        if el == 1 {
            self.el1_led_pin.set_high();
        }
        if el == 2 {
            self.el2_led_pin.set_high();
        }
        if el == 3 {
            self.el3_led_pin.set_high();
        }
    }
}

use crate::{
    gpio::{
        self,
        GpioPin::{Pin17, Pin22, Pin27},
    },
    util::wait_cycle,
};

pub struct Ic595 {
    ser_pin: gpio::Pin,
    rclk_pin: gpio::Pin,
    sclk_pin: gpio::Pin,
}

impl Ic595 {
    pub fn new() -> Self {
        let mut p17 = gpio::Pin::new(Pin17);
        let mut p27 = gpio::Pin::new(Pin27);
        let mut p22 = gpio::Pin::new(Pin22);

        p17.set_for_output();
        p27.set_for_output();
        p22.set_for_output();

        Ic595 {
            ser_pin: p17,
            rclk_pin: p27,
            sclk_pin: p22,
        }
    }

    pub fn load_shift(&mut self, bits: u8) {
        unsafe {
            for i in 0..8 {
                // wait_cycle(4_000_000);

                let b = (bits >> 7 - i) & 0b1;

                if b > 0 {
                    self.ser_pin.set_high();
                } else {
                    self.ser_pin.set_low();
                }

                self.sclk_pin.set_high();
                wait_cycle(5);
                self.sclk_pin.set_low();
            }

            self.rclk_pin.set_high();
            wait_cycle(5);
            self.rclk_pin.set_low();
        }
    }
}

impl Drop for Ic595 {
    fn drop(&mut self) {
        self.ser_pin.set_for_input();
        self.rclk_pin.set_for_input();
        self.sclk_pin.set_for_input();
    }
}

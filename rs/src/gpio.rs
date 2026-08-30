use core::ptr::{self, read_volatile};

use crate::gpio::GpioMode::{Input, Output};

type GpioReg = u32;

pub const PBASE: GpioReg = 0x3f000000;
pub const GPIO_BASE: GpioReg = PBASE + 0x200000;
pub const GPFSEL0: GpioReg = GPIO_BASE + 0x00;
pub const GPFSEL1: GpioReg = GPIO_BASE + 0x04;
pub const GPFSEL2: GpioReg = GPIO_BASE + 0x08;
pub const GPFSEL3: GpioReg = GPIO_BASE + 0x0c;
pub const GPFSEL4: GpioReg = GPIO_BASE + 0x10;
pub const GPFSEL5: GpioReg = GPIO_BASE + 0x14;
pub const GPSET0: GpioReg = GPIO_BASE + 0x1c;
pub const GPSET1: GpioReg = GPIO_BASE + 0x20;
pub const GPCLR0: GpioReg = GPIO_BASE + 0x28;
pub const GPCLR1: GpioReg = GPIO_BASE + 0x2c;
pub const GPLEV0: GpioReg = GPIO_BASE + 0x34;
pub const GPLEV1: GpioReg = GPIO_BASE + 0x38;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GpioPin {
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
    Pin5 = 5,
    Pin6 = 6,
    Pin7 = 7,
    Pin8 = 8,
    Pin9 = 9,
    Pin10 = 10,
    Pin11 = 11,
    Pin12 = 12,
    Pin13 = 13,
    Pin14 = 14,
    Pin15 = 15,
    Pin16 = 16,
    Pin17 = 17,
    Pin18 = 18,
    Pin19 = 19,
    Pin20 = 20,
    Pin21 = 21,
    Pin22 = 22,
    Pin23 = 23,
    Pin24 = 24,
    Pin25 = 25,
    Pin26 = 26,
    Pin27 = 27,
    Pin28 = 28,
    Pin29 = 29,
    Pin30 = 30,
    Pin31 = 31,
    Pin32 = 32,
    Pin33 = 33,
    Pin34 = 34,
    Pin35 = 35,
    Pin36 = 36,
    Pin37 = 37,
    Pin38 = 38,
    Pin39 = 39,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum GpioMode {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

pub struct Pin {
    pin: GpioPin,
    mode: GpioMode,
    level: bool,
}

impl Pin {
    pub fn new(pin: GpioPin) -> Self {
        Self {
            pin: pin,
            mode: GpioMode::Input,
            level: false,
        }
    }

    fn get_function_reg(&self) -> GpioReg {
        let x = (self.pin as u32 / 10) * 0x04;
        GPFSEL0 + x
    }

    fn get_set_reg(&self) -> GpioReg {
        let x = (self.pin as u32 / 32) * 0x04;
        GPSET0 + x
    }

    fn get_clr_reg(&self) -> GpioReg {
        let x = (self.pin as u32 / 32) * 0x04;
        GPCLR0 + x
    }

    fn get_lev_reg(&self) -> GpioReg {
        let x = (self.pin as u32 / 32) * 0x04;
        GPLEV0 + x
    }

    pub fn set_for_input(&mut self) {
        self.set_for(Input);
    }

    pub fn set_for_output(&mut self) {
        self.set_for(Output);
    }

    pub fn set_for(&mut self, mode: GpioMode) {
        let fun_reg = self.get_function_reg();
        unsafe {
            let prev_value: u32 = read_volatile(fun_reg as *mut u32);
            let x: u32 = (self.pin as u32 % 10 * 3);
            let y = (prev_value & !(0b111 << x));
            let z = (mode as u32) << x;
            ptr::write_volatile(fun_reg as *mut u32, y | z);
        }
        self.mode = mode;
    }

    fn set_level(&mut self, reg: GpioReg) {
        let x = (self.pin as u32) % 32;
        let y = 0b1 << x;

        unsafe {
            ptr::write_volatile(reg as *mut u32, y);
        }
    }

    pub fn set_low(&mut self) {
        let r = self.get_clr_reg();
        self.set_level(r);
        self.level = false;
    }

    pub fn set_high(&mut self) {
        let r = self.get_set_reg();
        self.set_level(r);
        self.level = true;
    }

    pub fn get_level(&mut self) -> bool {
        let reg = self.get_lev_reg();
        let bit = (self.pin as u32) % 32;

        let level = unsafe { (ptr::read_volatile(reg as *const u32) & (1 << bit)) != 0 };

        self.level = level;
        level
    }
}

impl Drop for Pin {
    fn drop(&mut self) {
        self.set_for_input();
    }
}

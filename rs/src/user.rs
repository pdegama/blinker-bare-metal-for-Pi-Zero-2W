use crate::{
    gpio::{
        self,
        GpioPin::{Pin12, Pin16, Pin19, Pin20, Pin26},
        Pin,
    },
    ic595::Ic595,
    mini_uart::{self, uart_send, uart_send_string},
    table::{self, gpio_handler},
};

pub fn run_user() {
    unsafe {
        set_handlers();

        let mut pin12 = gpio::Pin::new(Pin12);
        pin12.set_for_input();
        pin12.set_pull_down();
        pin12.clear_event();
        pin12.set_risining_eg();

        let mut pin19 = gpio::Pin::new(Pin19);
        pin19.set_for_input();
        pin19.set_pull_up();
        pin19.clear_event();
        pin19.set_falling_eg();

        let mut pin16 = gpio::Pin::new(Pin16);
        pin16.set_for_input();
        pin16.set_pull_up();
        pin16.clear_event();
        pin16.set_falling_eg();

        let mut pin20 = gpio::Pin::new(Pin20);
        pin20.set_for_input();
        pin20.set_pull_up();
        //pin16.clear_event();
        //pin16.set_falling_eg();
    }
}

fn handle_msg_from_are(msg: u8) {
    unsafe {
        if let Some(c) = table::uart_channel {
            c("Get: ");
            uart_send(msg);
            c("\n");
        }

        shift_sr_bits(msg);
    }
}

fn shift_sr_bits(des: u8) {
    unsafe {
        let x = (des - 48) % 8;
        let y: u8 = 0b11111111 & !(0b1 << x);
        let mut sr = Ic595::new();
        sr.load_shift(y);

        if let Some(c) = table::uart_channel {
            c("Shift: ");
            for i in 0..8 {
                let l = 7 - i;
                if (y >> l & 0b1 > 0) {
                    uart_send(b'1');
                } else {
                    uart_send(b'0');
                }
            }
            c("\n");
        }
    }
}

static mut active_bit: u8 = 0;
fn handle_pins(bits: u64) {
    unsafe {
        if bits & 0b1 << 12 > 0 {
            uart_send_string("BUTTON PUSHED\n");
            let mut pin26 = gpio::Pin::new(Pin26);
            pin26.set_for_output();
            if !pin26.get_level() {
                pin26.set_high();
            } else {
                pin26.set_low();
            };
            let pin12 = gpio::Pin::new(Pin12);
            pin12.clear_event();
        }

        if bits & 0b1 << 19 > 0 {
            uart_send_string("19\n");
            let pin19 = gpio::Pin::new(Pin19);
            pin19.clear_event();
        }

        if bits & 0b1 << 16 > 0 {
            let pin16 = gpio::Pin::new(Pin16);
            let mut pin20 = gpio::Pin::new(Pin20);
            if pin20.get_level() {
                if active_bit == 7 {
                    active_bit = 0;
                } else {
                    active_bit += 1;
                }
            } else {
                if active_bit == 0 {
                    active_bit = 7;
                } else {
                    active_bit -= 1;
                }
            }
            pin16.clear_event();
            shift_sr_bits(active_bit);
        }
    }
}

fn set_handlers() {
    unsafe {
        table::uart_channel = Some(mini_uart::uart_send_string);
        table::uart_handler = Some(handle_msg_from_are);
        table::gpio_handler = Some(handle_pins);
    }
}

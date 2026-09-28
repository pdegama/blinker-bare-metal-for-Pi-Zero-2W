type uart_handler_t = fn(u8);
type uart_channel_t = fn(s: &str);
pub static mut uart_channel: Option<uart_channel_t> = None;
pub static mut uart_handler: Option<uart_handler_t> = None;

type gpio_handler_t = fn(u64);
pub static mut gpio_handler: Option<gpio_handler_t> = None;

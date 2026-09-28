pub type PerReg = u32;
pub const PBASE: PerReg = 0x3f000000;
pub const AUX_BASE: PerReg = PBASE + 0x215000;

// AUX registers
pub const AUX_IRQ: PerReg = AUX_BASE + 0x00;
pub const AUX_ENABLES: PerReg = AUX_BASE + 0x04;

// Mini UART
pub const AUX_MU_IO_REG: PerReg = AUX_BASE + 0x40;
pub const AUX_MU_IER_REG: PerReg = AUX_BASE + 0x44;
pub const AUX_MU_IIR_REG: PerReg = AUX_BASE + 0x48;
pub const AUX_MU_LCR_REG: PerReg = AUX_BASE + 0x4C;
pub const AUX_MU_MCR_REG: PerReg = AUX_BASE + 0x50;
pub const AUX_MU_LSR_REG: PerReg = AUX_BASE + 0x54;
pub const AUX_MU_MSR_REG: PerReg = AUX_BASE + 0x58;
pub const AUX_MU_SCRATCH: PerReg = AUX_BASE + 0x5C;
pub const AUX_MU_CNTL_REG: PerReg = AUX_BASE + 0x60;
pub const AUX_MU_STAT_REG: PerReg = AUX_BASE + 0x64;
pub const AUX_MU_BAUD_REG: PerReg = AUX_BASE + 0x68;

// SPI 1
pub const AUX_SPI0_CNTL0_REG: PerReg = AUX_BASE + 0x80;
pub const AUX_SPI0_CNTL1_REG: PerReg = AUX_BASE + 0x84;
pub const AUX_SPI0_STAT_REG: PerReg = AUX_BASE + 0x88;
pub const AUX_SPI0_IO_REG: PerReg = AUX_BASE + 0x90;
pub const AUX_SPI0_PEEK_REG: PerReg = AUX_BASE + 0x94;

// SPI 2
pub const AUX_SPI1_CNTL0_REG: PerReg = AUX_BASE + 0xC0;
pub const AUX_SPI1_CNTL1_REG: PerReg = AUX_BASE + 0xC4;
pub const AUX_SPI1_STAT_REG: PerReg = AUX_BASE + 0xC8;
pub const AUX_SPI1_IO_REG: PerReg = AUX_BASE + 0xD0;
pub const AUX_SPI1_PEEK_REG: PerReg = AUX_BASE + 0xD4;

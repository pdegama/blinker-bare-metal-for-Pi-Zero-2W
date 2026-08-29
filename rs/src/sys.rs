// registers.rs

pub const SCTLR_RESERVED: u64 = (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11);

pub const SCTLR_EE_LITTLE_ENDIAN: u64 = 0 << 25;
pub const SCTLR_EOE_LITTLE_ENDIAN: u64 = 0 << 24;
pub const SCTLR_I_CACHE_DISABLED: u64 = 0 << 12;
pub const SCTLR_D_CACHE_DISABLED: u64 = 0 << 2;
pub const SCTLR_MMU_DISABLED: u64 = 0 << 0;
pub const SCTLR_MMU_ENABLED: u64 = 1 << 0;

#[unsafe(no_mangle)]
pub static SCTLR_VALUE_MMU_DISABLED: u64 = SCTLR_RESERVED
    | SCTLR_EE_LITTLE_ENDIAN
    | SCTLR_I_CACHE_DISABLED
    | SCTLR_D_CACHE_DISABLED
    | SCTLR_MMU_DISABLED;

// D13.2.47
pub const HCR_RW: u64 = 1 << 31;
#[unsafe(no_mangle)]
pub static HCR_VALUE: u64 = HCR_RW;

// D13.2.112
pub const SCR_RESERVED: u64 = 3 << 4;
pub const SCR_RW: u64 = 1 << 10;
pub const SCR_NS: u64 = 1 << 0;

#[unsafe(no_mangle)]
pub static SCR_VALUE: u64 = SCR_RESERVED | SCR_RW | SCR_NS;

// C5.2.19
pub const SPSR_MASK_ALL: u64 = 7 << 6;
pub const SPSR_EL1h: u64 = 5 << 0;
pub const SPSR_EL2h: u64 = 9 << 0;

#[unsafe(no_mangle)]
pub static SPSR_VALUE: u64 = SPSR_MASK_ALL | SPSR_EL1h;

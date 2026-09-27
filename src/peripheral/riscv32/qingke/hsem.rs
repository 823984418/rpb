use crate::register::{Reg32, Reserved};

/// 硬件信号量
/// 这是 WCH 的专用外设
#[derive(Debug)]
#[repr(C)]
pub struct HSEM {
    pub RX: [Reg32; 32],

    RESERVED0: Reserved<[u8; 0x80]>,

    pub RLRX: [Reg32; 32],

    RESERVED1: Reserved<[u8; 0x80]>,

    pub LSE: Reg32,

    RESERVED2: Reserved<[u8; 4]>,

    pub CLR: Reg32,

    pub KEY: Reg32,

    RESERVED3: Reserved<[u8; 0xF0]>,

    pub IER: Reg32,

    RESERVED4: Reserved<[u8; 4]>,

    pub ISR: Reg32,

    RESERVED5: Reserved<[u8; 4]>,

    pub ISM: Reg32,

    RESERVED6: Reserved<[u8; 4]>,

    pub LSM: Reg32,
}

impl HSEM {
    pub const BASE: usize = 0xE000C000;
}

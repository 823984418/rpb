use bitflags::bitflags;

use crate::register::{Reg, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct CRC {
    pub DATAR: Reg<DATAR>,
    pub IDATAR: Reg<IDATAR>,
    RESERVED0: Reserved<u8>,
    RESERVED1: Reserved<u16>,
    pub CTLR: Reg<CTLR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATAR: u32 {
        const DR = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR: u8 {
        const IDATAR = 0xFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const RESET = 0x01;
    }
}

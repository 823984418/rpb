use bitflags::bitflags;

use crate::register::{Reg, Reg32};

#[derive(Debug)]
#[repr(C)]
pub struct RNG {
    pub CR: Reg<CR>,
    pub SR: Reg<SR>,
    pub DR: Reg32,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const RNGEN = 0x00000004;
        const IE = 0x00000008;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const DRDY = 0x00000001;
        const CECS = 0x00000002;
        const SECS = 0x00000004;
        const CEIS = 0x00000020;
        const SEIS = 0x00000040;
    }
}

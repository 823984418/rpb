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
        const IE = 1 << 3;
        const RNGEN = 1 << 2;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const SEIS = 1 << 6;
        const CEIS = 1 << 5;
        const SECS = 1 << 2;
        const CECS = 1 << 1;
        const DRDY = 1 << 0;
    }
}

use bitflags::bitflags;

use crate::register::Reg;

/// IWDT register block.
#[derive(Debug)]
#[repr(C)]
pub struct IWDT {
    pub CR: Reg<CR>,
    pub CLR: Reg<CLR>,
}

bitflags! {
    /// `CR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const LOCK_MASK = 0xffff << 16;

        const IWDTON = 1 << 15;

        const IWDTPR_MASK = 0x7 << 12;

        const IWDTRLR_MASK = 0xfff << 0;
    }
}

bitflags! {
    /// `CLR` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CLR: u32 {
        const IWDTCLR_MASK = 0xffff << 0;
    }
}

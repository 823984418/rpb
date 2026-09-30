use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// N_TEST register block.
#[derive(Debug)]
#[repr(C)]
pub struct N_TEST {
    pub NTW: Reg<NTW>,
    pub NTID0: Reg<NTID0>,
    pub NTID1: Reg<NTID1>,
    pub NTID2: Reg<NTID2>,
    pub NTC0: Reg32,
    pub NTC1: Reg32,
    pub NTC2: Reg32,
}

bitflags! {
    /// `NTW` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NTW: u32 {
        const WDT_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `NTID0` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NTID0: u32 {
        const CODE_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `NTID1` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NTID1: u32 {
        const CODE_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `NTID2` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NTID2: u32 {
        const CODE_MASK = 0xffff << 0;
    }
}

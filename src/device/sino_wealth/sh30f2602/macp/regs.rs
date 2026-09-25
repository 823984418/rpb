use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// MACP register block.
#[derive(Debug)]
#[repr(C)]
pub struct MACP {
    pub CORDCSR0: Reg<CORDCSR0>,
    pub OPRDX0: Reg32,
    pub OPRDY0: Reg32,
    pub OPRDZ0: Reg32,
    pub CORDCSR1: Reg<CORDCSR1>,
    pub OPRDX1: Reg32,
    pub OPRDY1: Reg32,
    pub OPRDZ1: Reg32,
    /// Reserved (0x20..=0xff).
    RESERVED0: [Reg32; 56],
    pub DIVCSR0: Reg<DIVCSR0>,
    pub DIVDND0: Reg32,
    pub DIVSOR0: Reg32,
    pub DIVRLT0: Reg32,
    pub DIVCSR1: Reg<DIVCSR1>,
    pub DIVDND1: Reg32,
    pub DIVSOR1: Reg32,
    pub DIVRLT1: Reg32,
}

bitflags! {
    /// `CORDCSR0` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CORDCSR0: u32 {
        const MODE = 1 << 7;

        const XYMRS = 1 << 3;

        const OVF = 1 << 1;

        const RUN = 1 << 0;
    }
}

bitflags! {
    /// `CORDCSR1` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CORDCSR1: u32 {
        const MODE = 1 << 7;

        const XYMRS = 1 << 3;

        const OVF = 1 << 1;

        const RUN = 1 << 0;
    }
}

bitflags! {
    /// `DIVCSR0` (0x100).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DIVCSR0: u32 {
        const SAT = 1 << 2;

        const SIGN = 1 << 1;

        const RUN = 1 << 0;
    }
}

bitflags! {
    /// `DIVCSR1` (0x110).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DIVCSR1: u32 {
        const SAT = 1 << 2;

        const SIGN = 1 << 1;

        const RUN = 1 << 0;
    }
}

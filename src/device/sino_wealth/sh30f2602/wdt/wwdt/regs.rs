use bitflags::bitflags;

use crate::register::Reg;

/// WWDT register block.
#[derive(Debug)]
#[repr(C)]
pub struct WWDT {
    pub CR: Reg<CR>,
    pub SR: Reg<SR>,
    pub CLR: Reg<CLR>,
    pub WTR: Reg<WTR>,
}

bitflags! {
    /// `CR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const LOCK_MASK = 0xffff << 16;

        const WWDTON = 1 << 15;

        const WWDTIE = 1 << 14;

        const WWDTPR_MASK = 0x7 << 8;

        const WWDTRLR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `SR` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const WWDTIF = 1 << 15;

        const TCNT_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `CLR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CLR: u32 {
        const WWDTCLR_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `WTR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WTR: u32 {
        const LOCK_MASK = 0xffff << 16;

        const WWDTWTR_MASK = 0xff << 0;
    }
}

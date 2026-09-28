use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct IWDG {
    pub CTLR: Reg<CTLR>,
    pub PSCR: Reg<PSCR>,
    pub RLDR: Reg<RLDR>,
    pub STATR: Reg<STATR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const KEY_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PSCR: u32 {
        const PR_2 = 1 << 2;
        const PR_1 = 1 << 1;
        const PR_0 = 1 << 0;
        const PR_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RLDR: u32 {
        const RL_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const RVU = 1 << 1;
        const PVU = 1 << 0;
    }
}

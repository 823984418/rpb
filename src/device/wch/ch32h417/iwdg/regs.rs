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
        const KEY = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PSCR: u32 {
        const PR = 0x07;
        const PR_0 = 0x01;
        const PR_1 = 0x02;
        const PR_2 = 0x04;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RLDR: u32 {
        const RL = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const PVU = 0x01;
        const RVU = 0x02;
    }
}

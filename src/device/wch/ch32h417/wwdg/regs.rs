use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct WWDG {
    pub CTLR: Reg<CTLR>,
    pub CFGR: Reg<CFGR>,
    pub STATR: Reg<STATR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const WDGA = 1 << 7;

        const T6 = 1 << 6;
        const T5 = 1 << 5;
        const T4 = 1 << 4;
        const T3 = 1 << 3;
        const T2 = 1 << 2;
        const T1 = 1 << 1;
        const T0 = 1 << 0;
        const T_MASK = 0x7F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const EWI = 1 << 9;

        const WDGTB1 = 1 << 8;
        const WDGTB0 = 1 << 7;
        const WDGTB_MASK = 0x3 << 7;

        const W6 = 1 << 6;
        const W5 = 1 << 5;
        const W4 = 1 << 4;
        const W3 = 1 << 3;
        const W2 = 1 << 2;
        const W1 = 1 << 1;
        const W0 = 1 << 0;
        const W_MASK = 0x7F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const EWIF = 1 << 0;
    }
}

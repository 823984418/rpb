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
        const T = 0x7F;
        const T0 = 0x01;
        const T1 = 0x02;
        const T2 = 0x04;
        const T3 = 0x08;
        const T4 = 0x10;
        const T5 = 0x20;
        const T6 = 0x40;

        const WDGA = 0x80;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const W = 0x007F;
        const W0 = 0x0001;
        const W1 = 0x0002;
        const W2 = 0x0004;
        const W3 = 0x0008;
        const W4 = 0x0010;
        const W5 = 0x0020;
        const W6 = 0x0040;

        const WDGTB = 0x0180;
        const WDGTB0 = 0x0080;
        const WDGTB1 = 0x0100;

        const EWI = 0x0200;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const EWIF = 0x01;
    }
}

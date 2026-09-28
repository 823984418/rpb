use bitflags::bitflags;

use crate::register::{Reg, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct LPTIM {
    pub ISR: Reg<ISR>,
    pub ICR: Reg<ICR>,
    pub IER: Reg<IER>,
    pub CFGR: Reg<CFGR>,
    pub CR: Reg<CR>,
    pub CMP: Reg<CMP>,
    Reserved0: Reserved<u16>,
    pub ARR: Reg<ARR>,
    Reserved1: Reserved<u16>,
    pub CNT: Reg<CNT>,
    Reserved2: Reserved<u16>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const DIRSYNC = 1 << 7;
        const DOWN = 1 << 6;
        const UP = 1 << 5;
        const ARROK = 1 << 4;
        const CMPOK = 1 << 3;
        const EXTTRIG = 1 << 2;
        const ARRM = 1 << 1;
        const CMPM = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ICR: u32 {
        const DOWNCF = 1 << 6;
        const UPCF = 1 << 5;
        const ARROKCF = 1 << 4;
        const CMPOKCF = 1 << 3;
        const EXTTRIGCF = 1 << 2;
        const ARRMCF = 1 << 1;
        const CMPMCF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u32 {
        const DOWNIE = 1 << 6;
        const UPIE = 1 << 5;
        const ARROKIE = 1 << 4;
        const CMPOKIE = 1 << 3;
        const EXTTRIGIE = 1 << 2;
        const ARRMIE = 1 << 1;
        const CMPMIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const FORCEPWM = 1 << 27;

        const CLKSEL_MASK = 0x3 << 25;

        const ENC = 1 << 24;
        const CONTMODE = 1 << 23;
        const PRELOAD = 1 << 22;
        const WAVPOL = 1 << 21;
        const WAVE = 1 << 20;
        const TIMOUT = 1 << 19;

        const TRIGEN_MASK = 0x3 << 17;

        const TRIGSEL_MASK = 0x3 << 13;

        const PRESC_MASK = 0x7 << 9;

        const TRGFLT_MASK = 0x3 << 6;

        const CKFLT_MASK = 0x3 << 3;

        const CKPOL_MASK = 0x3 << 1;

        const CKSEL = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const DIR_EXTEN = 1 << 4;
        const OUTEN = 1 << 3;
        const CNTSTRT = 1 << 2;
        const SNGSTRT = 1 << 1;
        const ENABLE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP: u16 {
        const CMP_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ARR: u16 {
        const ARR_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNT: u16 {
        const CNT_MASK = 0xFFFF << 0;
    }
}

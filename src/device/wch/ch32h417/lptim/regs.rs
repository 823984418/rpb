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
        const CMPM = 0x00000001;
        const ARRM = 0x00000002;
        const EXTTRIG = 0x00000004;
        const CMPOK = 0x00000008;
        const ARROK = 0x00000010;
        const UP = 0x00000020;
        const DOWN = 0x00000040;
        const DIRSYNC = 0x00000080;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ICR: u32 {
        const CMPMCF = 0x00000001;
        const ARRMCF = 0x00000002;
        const EXTTRIGCF = 0x00000004;
        const CMPOKCF = 0x00000008;
        const ARROKCF = 0x00000010;
        const UPCF = 0x00000020;
        const DOWNCF = 0x00000040;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u32 {
        const CMPMIE = 0x00000001;
        const ARRMIE = 0x00000002;
        const EXTTRIGIE = 0x00000004;
        const CMPOKIE = 0x00000008;
        const ARROKIE = 0x00000010;
        const UPIE = 0x00000020;
        const DOWNIE = 0x00000040;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const CKSEL = 0x00000001;
        const CKPOL = 0x00000006;
        const CKFLT = 0x00000018;
        const TRGFLT = 0x000000C0;
        const PRESC = 0x00000E00;
        const TRIGSEL = 0x00006000;
        const TRIGEN = 0x00060000;
        const TIMOUT = 0x00080000;
        const WAVE = 0x00100000;
        const WAVPOL = 0x00200000;
        const PRELOAD = 0x00400000;
        const CONTMODE = 0x00800000;
        const ENC = 0x01000000;
        const CLKSEL = 0x06000000;
        const FORCEPWM = 0x08000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const ENABLE = 0x00000001;
        const SNGSTRT = 0x00000002;
        const CNTSTRT = 0x00000004;
        const OUTEN = 0x00000008;
        const DIR_EXTEN = 0x00000010;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP: u16 {
        const CMP = 0x0000FFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ARR: u16 {
        const ARR = 0x0000FFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNT: u16 {
        const COUNT = 0x0000FFFF;
    }
}

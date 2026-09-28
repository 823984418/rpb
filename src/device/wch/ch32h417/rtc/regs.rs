use bitflags::bitflags;

use crate::register::{Reg, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct RTC {
    pub CTLRH: Reg<CTLRH>,
    RESERVED0: Reserved<u16>,
    pub CTLRL: Reg<CTLRL>,
    RESERVED1: Reserved<u16>,
    pub PSCRH: Reg<PSCRH>,
    RESERVED2: Reserved<u16>,
    pub PSCRL: Reg<PSCRL>,
    RESERVED3: Reserved<u16>,
    pub DIVH: Reg<DIVH>,
    RESERVED4: Reserved<u16>,
    pub DIVL: Reg<DIVL>,
    RESERVED5: Reserved<u16>,
    pub CNTH: Reg<CNTH>,
    RESERVED6: Reserved<u16>,
    pub CNTL: Reg<CNTL>,
    RESERVED7: Reserved<u16>,
    pub ALRMH: Reg<ALRMH>,
    RESERVED8: Reserved<u16>,
    pub ALRML: Reg<ALRML>,
    RESERVED9: Reserved<u16>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLRH: u16 {
        const OWIE = 1 << 2;
        const ALRIE = 1 << 1;
        const SECIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLRL: u16 {
        const RTOFF = 1 << 5;
        const CNF = 1 << 4;
        const RSF = 1 << 3;
        const OWF = 1 << 2;
        const ALRF = 1 << 1;
        const SECF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PSCRH: u16 {
        const PRL_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PSCRL: u16 {
        const PRL_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DIVH: u16 {
        const RTC_DIV_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DIVL: u16 {
        const RTC_DIV_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTH: u16 {
        const RTC_CNT_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTL: u16 {
        const RTC_CNT_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ALRMH: u16 {
        const RTC_ALRM_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ALRML: u16 {
        const RTC_ALRM_MASK = 0xFFFF << 0;
    }
}

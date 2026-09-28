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
        const SECIE = 0x01;
        const ALRIE = 0x02;
        const OWIE = 0x04;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLRL: u16 {
        const SECF = 0x01;
        const ALRF = 0x02;
        const OWF = 0x04;
        const RSF = 0x08;
        const CNF = 0x10;
        const RTOFF = 0x20;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PSCRH: u16 {
        const PRL = 0x000F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PSCRL: u16 {
        const PRL = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DIVH: u16 {
        const RTC_DIV = 0x000F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DIVL: u16 {
        const RTC_DIV = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTH: u16 {
        const RTC_CNT = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTL: u16 {
        const RTC_CNT = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ALRMH: u16 {
        const RTC_ALRM = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ALRML: u16 {
        const RTC_ALRM = 0xFFFF;
    }
}

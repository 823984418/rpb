use bitflags::bitflags;

use crate::register::{Reg, Reg32};

#[derive(Debug)]
#[repr(C)]
pub struct QSPI {
    pub CR: Reg<CR>,
    pub DCR: Reg<DCR>,
    pub SR: Reg<SR>,
    pub FCR: Reg<FCR>,
    pub DLR: Reg32,
    pub CCR: Reg<CCR>,
    pub AR: Reg32,
    pub ABR: Reg32,
    pub DR: Reg32,
    pub PSMKR: Reg32,
    pub PSMAR: Reg32,
    pub PIR: Reg<PIR>,
    pub LPTR: Reg<LPTR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const PRESCALER_MASK = 0xFF << 24;

        const PMM = 1 << 23;
        const APMS = 1 << 22;
        const TOIE = 1 << 20;
        const SMIE = 1 << 19;
        const FTIE = 1 << 18;
        const TCIE = 1 << 17;
        const TEIE = 1 << 16;

        const FTHRES_MASK = 0x1F << 8;

        const FSEL = 1 << 7;
        const DFM = 1 << 6;
        const START = 1 << 5;
        const TCEN = 1 << 3;
        const DMAEN = 1 << 2;
        const ABORT = 1 << 1;
        const EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DCR: u32 {
        const FSIZE_MASK = 0x1F << 16;

        const CSHT_MASK = 0x7 << 8;

        const CKMODE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const IDLEF = 1 << 16;

        const FLEVEL_MASK = 0x3F << 8;

        const BUSY = 1 << 5;
        const TOF = 1 << 4;
        const SMF = 1 << 3;
        const FTF = 1 << 2;
        const TCF = 1 << 1;
        const TEF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FCR: u32 {
        const CTOF = 1 << 4;
        const CSMF = 1 << 3;
        const CTCF = 1 << 1;
        const CTEF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCR: u32 {
        const SIOO = 1 << 28;

        const FMODE_MASK = 0x3 << 26;

        const DMODE_MASK = 0x3 << 24;

        const DCYC_MASK = 0x1F << 18;

        const ABSIZE_MASK = 0x3 << 16;

        const ABMODE_MASK = 0x3 << 14;

        const ADSIZE_MASK = 0x3 << 12;

        const ADMODE_MASK = 0x3 << 10;

        const IMODE_MASK = 0x3 << 8;

        const INSTRUCTION_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PIR: u32 {
        const INTERVAL_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LPTR: u32 {
        const TIMEOUT_MASK = 0xFFFF << 0;
    }
}

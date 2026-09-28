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
        const PRESCALER_7 = 1 << 31;
        const PRESCALER_6 = 1 << 30;
        const PRESCALER_5 = 1 << 29;
        const PRESCALER_4 = 1 << 28;
        const PRESCALER_3 = 1 << 27;
        const PRESCALER_2 = 1 << 26;
        const PRESCALER_1 = 1 << 25;
        const PRESCALER_0 = 1 << 24;
        const PRESCALER_MASK = 0xFF << 24;

        const PMM = 1 << 23;
        const APMS = 1 << 22;
        const TOIE = 1 << 20;
        const SMIE = 1 << 19;
        const FTIE = 1 << 18;
        const TCIE = 1 << 17;
        const TEIE = 1 << 16;

        const FTHRES_4 = 1 << 12;
        const FTHRES_3 = 1 << 11;
        const FTHRES_2 = 1 << 10;
        const FTHRES_1 = 1 << 9;
        const FTHRES_0 = 1 << 8;
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
        const FSIZE_4 = 1 << 20;
        const FSIZE_3 = 1 << 19;
        const FSIZE_2 = 1 << 18;
        const FSIZE_1 = 1 << 17;
        const FSIZE_0 = 1 << 16;
        const FSIZE_MASK = 0x1F << 16;

        const CSHT_2 = 1 << 10;
        const CSHT_1 = 1 << 9;
        const CSHT_0 = 1 << 8;
        const CSHT_MASK = 0x7 << 8;

        const CKMODE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const IDLEF = 1 << 16;

        const FLEVEL_5 = 1 << 13;
        const FLEVEL_4 = 1 << 12;
        const FLEVEL_3 = 1 << 11;
        const FLEVEL_2 = 1 << 10;
        const FLEVEL_1 = 1 << 9;
        const FLEVEL_0 = 1 << 8;
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

        const FMODE_1 = 1 << 27;
        const FMODE_0 = 1 << 26;
        const FMODE_MASK = 0x3 << 26;

        const DMODE_1 = 1 << 25;
        const DMODE_0 = 1 << 24;
        const DMODE_MASK = 0x3 << 24;

        const DCYC_4 = 1 << 22;
        const DCYC_3 = 1 << 21;
        const DCYC_2 = 1 << 20;
        const DCYC_1 = 1 << 19;
        const DCYC_0 = 1 << 18;
        const DCYC_MASK = 0x1F << 18;

        const ABSIZE_1 = 1 << 17;
        const ABSIZE_0 = 1 << 16;
        const ABSIZE_MASK = 0x3 << 16;

        const ABMODE_1 = 1 << 15;
        const ABMODE_0 = 1 << 14;
        const ABMODE_MASK = 0x3 << 14;

        const ADSIZE_1 = 1 << 13;
        const ADSIZE_0 = 1 << 12;
        const ADSIZE_MASK = 0x3 << 12;

        const ADMODE_1 = 1 << 11;
        const ADMODE_0 = 1 << 10;
        const ADMODE_MASK = 0x3 << 10;

        const IMODE_1 = 1 << 9;
        const IMODE_0 = 1 << 8;
        const IMODE_MASK = 0x3 << 8;

        const INSTRUCTION_7 = 1 << 7;
        const INSTRUCTION_6 = 1 << 6;
        const INSTRUCTION_5 = 1 << 5;
        const INSTRUCTION_4 = 1 << 4;
        const INSTRUCTION_3 = 1 << 3;
        const INSTRUCTION_2 = 1 << 2;
        const INSTRUCTION_1 = 1 << 1;
        const INSTRUCTION_0 = 1 << 0;
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

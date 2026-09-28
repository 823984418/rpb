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
        const EN = 0x00000001;
        const ABORT = 0x00000002;
        const DMAEN = 0x00000004;
        const TCEN = 0x00000008;

        const START = 0x00000020;
        const DFM = 0x00000040;
        const FSEL = 0x00000080;

        const FTHRES = 0x00001F00;
        const FTHRES_0 = 0x00000100;
        const FTHRES_1 = 0x00000200;
        const FTHRES_2 = 0x00000400;
        const FTHRES_3 = 0x00000800;
        const FTHRES_4 = 0x00001000;

        const TEIE = 0x00010000;
        const TCIE = 0x00020000;
        const FTIE = 0x00040000;
        const SMIE = 0x00080000;
        const TOIE = 0x00100000;
        const APMS = 0x00400000;
        const PMM = 0x00800000;

        const PRESCALER = 0xFF000000;
        const PRESCALER_0 = 0x01000000;
        const PRESCALER_1 = 0x02000000;
        const PRESCALER_2 = 0x04000000;
        const PRESCALER_3 = 0x08000000;
        const PRESCALER_4 = 0x10000000;
        const PRESCALER_5 = 0x20000000;
        const PRESCALER_6 = 0x40000000;
        const PRESCALER_7 = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DCR: u32 {
        const CKMODE = 0x00000001;

        const CSHT = 0x00000700;
        const CSHT_0 = 0x00000100;
        const CSHT_1 = 0x00000200;
        const CSHT_2 = 0x00000400;

        const FSIZE = 0x001F0000;
        const FSIZE_0 = 0x00010000;
        const FSIZE_1 = 0x00020000;
        const FSIZE_2 = 0x00040000;
        const FSIZE_3 = 0x00080000;
        const FSIZE_4 = 0x00100000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const TEF = 0x00000001;
        const TCF = 0x00000002;
        const FTF = 0x00000004;
        const SMF = 0x00000008;
        const TOF = 0x00000010;
        const BUSY = 0x00000020;

        const FLEVEL = 0x00003F00;
        const FLEVEL_0 = 0x00000100;
        const FLEVEL_1 = 0x00000200;
        const FLEVEL_2 = 0x00000400;
        const FLEVEL_3 = 0x00000800;
        const FLEVEL_4 = 0x00001000;
        const FLEVEL_5 = 0x00002000;

        const IDLEF = 0x00010000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FCR: u32 {
        const CTEF = 0x00000001;
        const CTCF = 0x00000002;
        const CSMF = 0x00000008;
        const CTOF = 0x00000010;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCR: u32 {
        const INSTRUCTION = 0x000000FF;
        const INSTRUCTION_0 = 0x00000001;
        const INSTRUCTION_1 = 0x00000002;
        const INSTRUCTION_2 = 0x00000004;
        const INSTRUCTION_3 = 0x00000008;
        const INSTRUCTION_4 = 0x00000010;
        const INSTRUCTION_5 = 0x00000020;
        const INSTRUCTION_6 = 0x00000040;
        const INSTRUCTION_7 = 0x00000080;

        const IMODE = 0x00000300;
        const IMODE_0 = 0x00000100;
        const IMODE_1 = 0x00000200;

        const ADMODE = 0x00000C00;
        const ADMODE_0 = 0x00000400;
        const ADMODE_1 = 0x00000800;

        const ADSIZE = 0x00003000;
        const ADSIZE_0 = 0x00001000;
        const ADSIZE_1 = 0x00002000;

        const ABMODE = 0x0000C000;
        const ABMODE_0 = 0x00004000;
        const ABMODE_1 = 0x00008000;

        const ABSIZE = 0x00030000;
        const ABSIZE_0 = 0x00010000;
        const ABSIZE_1 = 0x00020000;

        const DCYC = 0x007C0000;
        const DCYC_0 = 0x00040000;
        const DCYC_1 = 0x00080000;
        const DCYC_2 = 0x00100000;
        const DCYC_3 = 0x00200000;
        const DCYC_4 = 0x00400000;

        const DMODE = 0x03000000;
        const DMODE_0 = 0x01000000;
        const DMODE_1 = 0x02000000;

        const FMODE = 0x0C000000;
        const FMODE_0 = 0x04000000;
        const FMODE_1 = 0x08000000;

        const SIOO = 0x10000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PIR: u32 {
        const INTERVAL = 0x0000FFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LPTR: u32 {
        const TIMEOUT = 0x0000FFFF;
    }
}

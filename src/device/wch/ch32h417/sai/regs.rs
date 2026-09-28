use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct SAI_BLOCK {
    pub CFGR1: Reg<CFGR1>,
    pub CFGR2: Reg<CFGR2>,
    pub FRCR: Reg<FRCR>,
    pub SLOTR: Reg<SLOTR>,
    pub INTENR: Reg<INTENR>,
    pub SR: Reg<SR>,
    RESERVED0: Reserved<u32>,
    pub DATAR: Reg32,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR1: u32 {
        const OSR = 1 << 26;

        const MCKDIV_MASK = 0x3F << 20;

        const NODIV = 1 << 19;
        const DMAEN = 1 << 17;
        const EN = 1 << 16;
        const MONO = 1 << 12;

        const SYNCEN_1 = 1 << 11;
        const SYNCEN_0 = 1 << 10;
        const SYNCEN_MASK = 0x3 << 10;

        const CKSTR = 1 << 9;
        const LSBFIRST = 1 << 8;

        const DS_2 = 1 << 7;
        const DS_1 = 1 << 6;
        const DS_0 = 1 << 5;
        const DS_MASK = 0x7 << 5;

        const PRTCFG_1 = 1 << 3;
        const PRTCFG_0 = 1 << 2;
        const PRTCFG_MASK = 0x3 << 2;

        const MODE_1 = 1 << 1;
        const MODE_0 = 1 << 0;
        const MODE_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR2: u32 {
        const COMP_MASK = 0x3 << 14;

        const CPL = 1 << 13;

        const MUTECNT_MASK = 0x3F << 7;

        const MUTEVAL = 1 << 6;
        const MUTE = 1 << 5;
        const TRIS = 1 << 4;
        const FFLUSH = 1 << 3;

        const FTH_2 = 1 << 2;
        const FTH_1 = 1 << 1;
        const FTH_0 = 1 << 0;
        const FTH_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FRCR: u32 {
        const FSOFF = 1 << 18;
        const FSPOL = 1 << 17;
        const FSDEF = 1 << 16;

        const FSALL_MASK = 0x7F << 8;

        const FRL_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SLOTR: u32 {
        const SLOTEN_MASK = 0xFFFF << 16;

        const NBSLOT_MASK = 0xF << 8;

        const SLOTSZ_MASK = 0x3 << 6;

        const FBOFF_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTENR: u32 {
        const LFSDETIE = 1 << 6;
        const AFSDETIE = 1 << 5;
        const CNRDYIE = 1 << 4;
        const FREQIE = 1 << 3;
        const WCKCFGIE = 1 << 2;
        const MUTEDETIE = 1 << 1;
        const OVRUDRIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const FLTH_2 = 0x3 << 16;
        const FLTH_1 = 1 << 17;
        const FLTH_0 = 1 << 16;
        const FLTH_MASK = 0x7 << 16;

        const LFSDET = 1 << 6;
        const AFSDET = 1 << 5;
        const CNRDY = 1 << 4;
        const FREQ = 1 << 3;
        const WCKCFG = 1 << 2;
        const MUTEDET = 1 << 1;
        const OVRUDR = 1 << 0;
    }
}

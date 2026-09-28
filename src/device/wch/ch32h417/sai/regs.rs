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
        const MODE = 0x00000003;
        const MODE_0 = 0x00000001;
        const MODE_1 = 0x00000002;

        const PRTCFG = 0x0000000C;
        const PRTCFG_0 = 0x00000004;
        const PRTCFG_1 = 0x00000008;

        const DS = 0x000000E0;
        const DS_0 = 0x00000020;
        const DS_1 = 0x00000040;
        const DS_2 = 0x00000080;

        const LSBFIRST = 0x00000100;
        const CKSTR = 0x00000200;

        const SYNCEN = 0x00000C00;
        const SYNCEN_0 = 0x00000400;
        const SYNCEN_1 = 0x00000800;
        const MONO = 0x00001000;
        const EN = 0x00010000;
        const DMAEN = 0x00020000;
        const NODIV = 0x00080000;
        const MCKDIV = 0x03F00000;
        const OSR = 0x04000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR2: u32 {
        const FTH = 0x00000007;
        const FTH_0 = 0x00000001;
        const FTH_1 = 0x00000002;
        const FTH_2 = 0x00000004;

        const FFLUSH = 0x00000008;
        const TRIS = 0x00000010;
        const MUTE = 0x00000020;
        const MUTEVAL = 0x00000040;

        const MUTECNT = 0x00001F80;

        const CPL = 0x00002000;
        const COMP = 0x0000C000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FRCR: u32 {
        const FRL = 0x000000FF;

        const FSALL = 0x00007F00;
        const FSDEF = 0x00010000;
        const FSPOL = 0x00020000;
        const FSOFF = 0x00040000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SLOTR: u32 {
        const FBOFF = 0x0000001F;
        const SLOTSZ = 0x000000C0;
        const NBSLOT = 0x00000F00;
        const SLOTEN = 0xFFFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTENR: u32 {
        const OVRUDRIE = 0x00000001;
        const MUTEDETIE = 0x00000002;
        const WCKCFGIE = 0x00000004;
        const FREQIE = 0x00000008;
        const CNRDYIE = 0x00000010;
        const AFSDETIE = 0x00000020;
        const LFSDETIE = 0x00000040;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const OVRUDR = 0x00000001;
        const MUTEDET = 0x00000002;
        const WCKCFG = 0x00000004;
        const FREQ = 0x00000008;
        const CNRDY = 0x00000010;
        const AFSDET = 0x00000020;
        const LFSDET = 0x00000040;

        const FLTH = 0x00070000;
        const FLTH_0 = 0x00010000;
        const FLTH_1 = 0x00020000;
        const FLTH_2 = 0x00030000;
    }
}

use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// PCA0 register block.
#[derive(Debug)]
#[repr(C)]
pub struct PCA0 {
    pub CR: Reg<CR>,
    pub CFGR: Reg<CFGR>,
    pub SR: Reg<SR>,
    pub FORCE: Reg<FORCE>,
    pub CNT: Reg<CNT>,
    pub PSC: Reg<PSC>,
    pub PR: Reg<PR>,
    pub CCMR0: Reg<CCMR0>,
    pub CCMR1: Reg<CCMR1>,
    pub CCMR2: Reg<CCMR2>,
    pub CCR0: Reg<CCR0>,
    pub CCR1: Reg<CCR1>,
    pub CCR2: Reg<CCR2>,
    pub LCKR: Reg<LCKR>,
    pub ADTR: Reg32,
    pub FLTWPR: Reg<FLTWPR>,
    pub FLTCR: Reg<FLTCR>,
}

bitflags! {
    /// `CR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const PEN = 1 << 0;
    }
}

bitflags! {
    /// `CFGR` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const SYN_MASK = 0x3 << 18;

        const ECF_MASK = 0x3 << 16;

        const PUE = 1 << 15;

        const CC2UE = 1 << 14;

        const CC1UE = 1 << 13;

        const CC0UE = 1 << 12;

        const ADTSEL_MASK = 0x3 << 9;

        const PIE = 1 << 8;

        const CIE = 1 << 7;

        const SDEN = 1 << 6;

        const CPS_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `SR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const CIFC = 1 << 23;

        const PIFC = 1 << 20;

        const FLTIFC = 1 << 19;

        const CC2IFC = 1 << 18;

        const CC1IFC = 1 << 17;

        const CC0IFC = 1 << 16;

        const CIF = 1 << 7;

        const PIF = 1 << 4;

        const FLTIF = 1 << 3;

        const CC2IF = 1 << 2;

        const CC1IF = 1 << 1;

        const CC0IF = 1 << 0;
    }
}

bitflags! {
    /// `FORCE` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FORCE: u32 {
        const FCO2 = 1 << 2;

        const FCO1 = 1 << 1;

        const FCO0 = 1 << 0;
    }
}

bitflags! {
    /// `CNT` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNT: u32 {
        const CNTH_MASK = 0xffff << 16;

        const CNTL_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PSC` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PSC: u32 {
        const PSC_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PR` (0x18).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PR: u32 {
        const PRH_MASK = 0xffff << 16;

        const PRL_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `CCMR0` (0x1c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCMR0: u32 {
        const CSSEL = 1 << 12;

        const ICF_MASK = 0x7 << 9;

        const CC = 1 << 8;

        const SM_MASK = 0x3 << 6;

        const FS_MASK = 0x3 << 4;

        const CEN = 1 << 3;

        const TCP = 1 << 2;

        const CCIE = 1 << 0;
    }
}

bitflags! {
    /// `CCMR1` (0x20).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCMR1: u32 {
        const CSSEL = 1 << 12;

        const ICF_MASK = 0x7 << 9;

        const CC = 1 << 8;

        const SM_MASK = 0x3 << 6;

        const FS_MASK = 0x3 << 4;

        const CEN = 1 << 3;

        const TCP = 1 << 2;

        const CCIE = 1 << 0;
    }
}

bitflags! {
    /// `CCMR2` (0x24).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCMR2: u32 {
        const CSSEL = 1 << 12;

        const ICF_MASK = 0x7 << 9;

        const CC = 1 << 8;

        const SM_MASK = 0x3 << 6;

        const FS_MASK = 0x3 << 4;

        const CEN = 1 << 3;

        const TCP = 1 << 2;

        const CCIE = 1 << 0;
    }
}

bitflags! {
    /// `CCR0` (0x28).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCR0: u32 {
        const CCR0H_MASK = 0xffff << 16;

        const CCR0L_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `CCR1` (0x2c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCR1: u32 {
        const CCR1H_MASK = 0xffff << 16;

        const CCR1L_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `CCR2` (0x30).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCR2: u32 {
        const CCR2H_MASK = 0xffff << 16;

        const CCR2L_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `LCKR` (0x34).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LCKR: u32 {
        const KEY_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `FLTWPR` (0x3c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FLTWPR: u32 {
        const KEY_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `FLTCR` (0x40).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FLTCR: u32 {
        const FLTIE = 1 << 14;

        const FOUT_MASK = 0x3 << 12;

        const FLT1EN = 1 << 10;

        const FLT1SEL_MASK = 0x3 << 8;

        const FLT2DEB_MASK = 0xf << 4;

        const FLT2EN = 1 << 3;

        const FLT2S = 1 << 2;

        const FLTM = 1 << 1;

        const FLTSTAT = 1 << 0;
    }
}

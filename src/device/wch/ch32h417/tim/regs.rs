use bitflags::bitflags;

use crate::register::{Reg, Reg16, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct TIM {
    pub CTLR1: Reg<CTLR1>,
    RESERVED0: Reserved<u16>,
    pub CTLR2: Reg<CTLR2>,
    RESERVED1: Reserved<u16>,
    pub SMCFGR: Reg<SMCFGR>,
    RESERVED2: Reserved<u16>,
    pub DMAINTENR: Reg<DMAINTENR>,
    RESERVED3: Reserved<u16>,
    pub INTFR: Reg<INTFR>,
    RESERVED4: Reserved<u16>,
    pub SWEVGR: Reg<SWEVGR>,
    RESERVED5: Reserved<u16>,
    pub CHCTLR1: Reg<CHCTLR1>,
    RESERVED6: Reserved<u16>,
    pub CHCTLR2: Reg<CHCTLR2>,
    RESERVED7: Reserved<u16>,
    pub CCER: Reg<CCER>,
    RESERVED8: Reserved<u16>,
    pub CNT: Reg32,
    pub PSC: Reg16,
    RESERVED10: Reserved<u16>,
    pub ATRLR: Reg32,
    pub RPTCR: Reg16,
    RESERVED12: Reserved<u16>,
    pub CH1CVR: Reg32,
    pub CH2CVR: Reg32,
    pub CH3CVR: Reg32,
    pub CH4CVR: Reg32,
    pub BDTR: Reg<BDTR>,
    RESERVED17: Reserved<u16>,
    pub DMACFGR: Reg<DMACFGR>,
    RESERVED18: Reserved<u16>,
    pub DMAADR: Reg16,
    RESERVED19: Reserved<u16>,
    pub AUX: Reg<AUX>,
    RESERVED20: Reserved<u16>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u16 {
        const CAPLVL = 1 << 15;
        const CAPOV = 1 << 14;
        const ARPE = 1 << 7;

        const CKD_1 = 1 << 9;
        const CKD_0 = 1 << 8;
        const CKD_MASK = 0x3 << 8;

        const CMS_1 = 1 << 6;
        const CMS_0 = 1 << 5;
        const CMS_MASK = 0x3 << 5;

        const DIR = 1 << 4;
        const OPM = 1 << 3;
        const URS = 1 << 2;
        const UDIS = 1 << 1;
        const CEN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u16 {
        const OIS4 = 1 << 14;
        const OIS3N = 1 << 13;
        const OIS3 = 1 << 12;
        const OIS2N = 1 << 11;
        const OIS2 = 1 << 10;
        const OIS1N = 1 << 9;
        const OIS1 = 1 << 8;
        const TI1S = 1 << 7;

        const MMS_2 = 1 << 6;
        const MMS_1 = 1 << 5;
        const MMS_0 = 1 << 4;
        const MMS_MASK = 0x7 << 4;

        const CCDS = 1 << 3;
        const CCUS = 1 << 2;
        const CCPC = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SMCFGR: u16 {
        const ETP = 1 << 15;
        const ECE = 1 << 14;

        const ETPS_1 = 1 << 13;
        const ETPS_0 = 1 << 12;
        const ETPS_MASK = 0x3 << 12;

        const ETF_3 = 1 << 11;
        const ETF_2 = 1 << 10;
        const ETF_1 = 1 << 9;
        const ETF_0 = 1 << 8;
        const ETF_MASK = 0xF << 8;

        const MSM = 1 << 7;

        const TS_2 = 1 << 6;
        const TS_1 = 1 << 5;
        const TS_0 = 1 << 4;
        const TS_MASK = 0x7 << 4;

        const SMS_2 = 1 << 2;
        const SMS_1 = 1 << 1;
        const SMS_0 = 1 << 0;
        const SMS_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DMAINTENR: u16 {
        const TDE = 1 << 14;
        const COMDE = 1 << 13;
        const CC4DE = 1 << 12;
        const CC3DE = 1 << 11;
        const CC2DE = 1 << 10;
        const CC1DE = 1 << 9;
        const UDE = 1 << 8;
        const BIE = 1 << 7;
        const TIE = 1 << 6;
        const COMIE = 1 << 5;
        const CC4IE = 1 << 4;
        const CC3IE = 1 << 3;
        const CC2IE = 1 << 2;
        const CC1IE = 1 << 1;
        const UIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFR: u16 {
        const CC4OF = 1 << 12;
        const CC3OF = 1 << 11;
        const CC2OF = 1 << 10;
        const CC1OF = 1 << 9;
        const BIF = 1 << 7;
        const TIF = 1 << 6;
        const COMIF = 1 << 5;
        const CC4IF = 1 << 4;
        const CC3IF = 1 << 3;
        const CC2IF = 1 << 2;
        const CC1IF = 1 << 1;
        const UIF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SWEVGR: u16 {
        const BG = 1 << 7;
        const TG = 1 << 6;
        const COMG = 1 << 5;
        const CC4G = 1 << 4;
        const CC3G = 1 << 3;
        const CC2G = 1 << 2;
        const CC1G = 1 << 1;
        const UG = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CHCTLR1: u16 {
        const OC2CE = 1 << 15;

        const OC2M_2 = 1 << 14;
        const OC2M_1 = 1 << 13;
        const OC2M_0 = 1 << 12;
        const OC2M_MASK = 0x7 << 12;

        const OC2PE = 1 << 11;
        const OC2FE = 1 << 10;

        const CC2S_1 = 1 << 9;
        const CC2S_0 = 1 << 8;
        const CC2S_MASK = 0x3 << 8;

        const OC1CE = 1 << 7;

        const OC1M_2 = 1 << 6;
        const OC1M_1 = 1 << 5;
        const OC1M_0 = 1 << 4;
        const OC1M_MASK = 0x7 << 4;

        const OC1PE = 1 << 3;
        const OC1FE = 1 << 2;

        const CC1S_1 = 1 << 1;
        const CC1S_0 = 1 << 0;
        const CC1S_MASK = 0x3 << 0;

        const IC2F_3 = 1 << 15;
        const IC2F_2 = 1 << 14;
        const IC2F_1 = 1 << 13;
        const IC2F_0 = 1 << 12;
        const IC2F_MASK = 0xF << 12;

        const IC2PSC_1 = 1 << 11;
        const IC2PSC_0 = 1 << 10;
        const IC2PSC_MASK = 0x3 << 10;

        const IC1F_3 = 1 << 7;
        const IC1F_2 = 1 << 6;
        const IC1F_1 = 1 << 5;
        const IC1F_0 = 1 << 4;
        const IC1F_MASK = 0xF << 4;

        const IC1PSC_1 = 1 << 3;
        const IC1PSC_0 = 1 << 2;
        const IC1PSC_MASK = 0x3 << 2;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CHCTLR2: u16 {
        const OC4CE = 1 << 15;

        const OC4M_2 = 1 << 14;
        const OC4M_1 = 1 << 13;
        const OC4M_0 = 1 << 12;
        const OC4M_MASK = 0x7 << 12;

        const OC4PE = 1 << 11;
        const OC4FE = 1 << 10;

        const CC4S_1 = 1 << 9;
        const CC4S_0 = 1 << 8;
        const CC4S_MASK = 0x3 << 8;

        const OC3CE = 1 << 7;

        const OC3M_2 = 1 << 6;
        const OC3M_1 = 1 << 5;
        const OC3M_0 = 1 << 4;
        const OC3M_MASK = 0x7 << 4;

        const OC3PE = 1 << 3;
        const OC3FE = 1 << 2;

        const CC3S_1 = 1 << 1;
        const CC3S_0 = 1 << 0;
        const CC3S_MASK = 0x3 << 0;

        const IC4F_3 = 1 << 15;
        const IC4F_2 = 1 << 14;
        const IC4F_1 = 1 << 13;
        const IC4F_0 = 1 << 12;
        const IC4F_MASK = 0xF << 12;

        const IC4PSC_1 = 1 << 11;
        const IC4PSC_0 = 1 << 10;
        const IC4PSC_MASK = 0x3 << 10;

        const IC3F_3 = 1 << 7;
        const IC3F_2 = 1 << 6;
        const IC3F_1 = 1 << 5;
        const IC3F_0 = 1 << 4;
        const IC3F_MASK = 0xF << 4;

        const IC3PSC_1 = 1 << 3;
        const IC3PSC_0 = 1 << 2;
        const IC3PSC_MASK = 0x3 << 2;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCER: u16 {
        const CC4P = 1 << 13;
        const CC4E = 1 << 12;
        const CC3NP = 1 << 11;
        const CC3NE = 1 << 10;
        const CC3P = 1 << 9;
        const CC3E = 1 << 8;
        const CC2NP = 1 << 7;
        const CC2NE = 1 << 6;
        const CC2P = 1 << 5;
        const CC2E = 1 << 4;
        const CC1NP = 1 << 3;
        const CC1NE = 1 << 2;
        const CC1P = 1 << 1;
        const CC1E = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BDTR: u16 {
        const MOE = 1 << 15;
        const AOE = 1 << 14;
        const BKP = 1 << 13;
        const BKE = 1 << 12;
        const OSSR = 1 << 11;
        const OSSI = 1 << 10;

        const LOCK_1 = 1 << 9;
        const LOCK_0 = 1 << 8;
        const LOCK_MASK = 0x3 << 8;

        const DTG_7 = 1 << 7;
        const DTG_6 = 1 << 6;
        const DTG_5 = 1 << 5;
        const DTG_4 = 1 << 4;
        const DTG_3 = 1 << 3;
        const DTG_2 = 1 << 2;
        const DTG_1 = 1 << 1;
        const DTG_0 = 1 << 0;
        const DTG_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DMACFGR: u16 {
        const DBL_4 = 1 << 12;
        const DBL_3 = 1 << 11;
        const DBL_2 = 1 << 10;
        const DBL_1 = 1 << 9;
        const DBL_0 = 1 << 8;
        const DBL_MASK = 0x1F << 8;

        const DBA_4 = 1 << 4;
        const DBA_3 = 1 << 3;
        const DBA_2 = 1 << 2;
        const DBA_1 = 1 << 1;
        const DBA_0 = 1 << 0;
        const DBA_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AUX: u16 {
        const DT_VLU2_MASK = 0xFF << 8;
        const DTN_MODE = 1 << 7;
        const DT_MODE = 1 << 6;

        const BK_SEL_2 = 1 << 5;
        const BK_SEL_1 = 1 << 4;
        const BK_SEL_0 = 1 << 3;
        const BK_SEL_MASK = 0x7 << 3;

        const CAPCH4_ED = 1 << 2;
        const CAPCH3_ED = 1 << 1;
        const CAPCH2_ED = 1 << 0;
    }
}

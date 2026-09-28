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
        const CAPLVL = 0x8000;
        const CAPOV = 0x4000;
        const CKD = 0x0300;
        const CKD_0 = 0x0100;
        const CKD_1 = 0x0200;
        const ARPE = 0x0080;
        const CMS = 0x0060;
        const CMS_0 = 0x0020;
        const CMS_1 = 0x0040;
        const DIR = 0x0010;
        const OPM = 0x0008;
        const URS = 0x0004;
        const UDIS = 0x0002;
        const CEN = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u16 {
        const OIS4 = 0x4000;
        const OIS3N = 0x2000;
        const OIS3 = 0x1000;
        const OIS2N = 0x0800;
        const OIS2 = 0x0400;
        const OIS1N = 0x0200;
        const OIS1 = 0x0100;
        const TI1S = 0x0080;
        const MMS = 0x0070;
        const MMS_0 = 0x0010;
        const MMS_1 = 0x0020;
        const MMS_2 = 0x0040;
        const CCDS = 0x0008;
        const CCUS = 0x0004;
        const CCPC = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SMCFGR: u16 {
        const ETP = 0x8000;
        const ECE = 0x4000;
        const ETPS = 0x3000;
        const ETPS_0 = 0x1000;
        const ETPS_1 = 0x2000;
        const ETF = 0x0F00;
        const ETF_0 = 0x0100;
        const ETF_1 = 0x0200;
        const ETF_2 = 0x0400;
        const ETF_3 = 0x0800;
        const MSM = 0x0080;
        const TS = 0x0070;
        const TS_0 = 0x0010;
        const TS_1 = 0x0020;
        const TS_2 = 0x0040;
        const SMS = 0x0007;
        const SMS_0 = 0x0001;
        const SMS_1 = 0x0002;
        const SMS_2 = 0x0004;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DMAINTENR: u16 {
        const TDE = 0x4000;
        const COMDE = 0x2000;
        const CC4DE = 0x1000;
        const CC3DE = 0x0800;
        const CC2DE = 0x0400;
        const CC1DE = 0x0200;
        const UDE = 0x0100;
        const BIE = 0x0080;
        const TIE = 0x0040;
        const COMIE = 0x0020;
        const CC4IE = 0x0010;
        const CC3IE = 0x0008;
        const CC2IE = 0x0004;
        const CC1IE = 0x0002;
        const UIE = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFR: u16 {
        const CC4OF = 0x1000;
        const CC3OF = 0x0800;
        const CC2OF = 0x0400;
        const CC1OF = 0x0200;
        const BIF = 0x0080;
        const TIF = 0x0040;
        const COMIF = 0x0020;
        const CC4IF = 0x0010;
        const CC3IF = 0x0008;
        const CC2IF = 0x0004;
        const CC1IF = 0x0002;
        const UIF = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SWEVGR: u16 {
        const BG = 0x80;
        const TG = 0x40;
        const COMG = 0x20;
        const CC4G = 0x10;
        const CC3G = 0x08;
        const CC2G = 0x04;
        const CC1G = 0x02;
        const UG = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CHCTLR1: u16 {
        const IC2F = 0xF000;
        const IC2F_0 = 0x1000;
        const IC2F_1 = 0x2000;
        const IC2F_2 = 0x4000;
        const IC2F_3 = 0x8000;
        const IC2PSC = 0x0C00;
        const IC2PSC_0 = 0x0400;
        const IC2PSC_1 = 0x0800;
        const OC2CE = 0x8000;
        const OC2M = 0x7000;
        const OC2M_0 = 0x1000;
        const OC2M_1 = 0x2000;
        const OC2M_2 = 0x4000;
        const OC2PE = 0x0800;
        const OC2FE = 0x0400;
        const CC2S = 0x0300;
        const CC2S_0 = 0x0100;
        const CC2S_1 = 0x0200;
        const IC1F = 0x00F0;
        const IC1F_0 = 0x0010;
        const IC1F_1 = 0x0020;
        const IC1F_2 = 0x0040;
        const IC1F_3 = 0x0080;
        const IC1PSC = 0x000C;
        const IC1PSC_0 = 0x0004;
        const IC1PSC_1 = 0x0008;
        const OC1CE = 0x0080;
        const OC1M = 0x0070;
        const OC1M_0 = 0x0010;
        const OC1M_1 = 0x0020;
        const OC1M_2 = 0x0040;
        const OC1PE = 0x0008;
        const OC1FE = 0x0004;
        const CC1S = 0x0003;
        const CC1S_0 = 0x0001;
        const CC1S_1 = 0x0002;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CHCTLR2: u16 {
        const IC4F = 0xF000;
        const IC4F_0 = 0x1000;
        const IC4F_1 = 0x2000;
        const IC4F_2 = 0x4000;
        const IC4F_3 = 0x8000;
        const IC4PSC = 0x0C00;
        const IC4PSC_0 = 0x0400;
        const IC4PSC_1 = 0x0800;
        const OC4CE = 0x8000;
        const OC4M = 0x7000;
        const OC4M_0 = 0x1000;
        const OC4M_1 = 0x2000;
        const OC4M_2 = 0x4000;
        const OC4PE = 0x0800;
        const OC4FE = 0x0400;
        const CC4S = 0x0300;
        const CC4S_0 = 0x0100;
        const CC4S_1 = 0x0200;
        const IC3F = 0x00F0;
        const IC3F_0 = 0x0010;
        const IC3F_1 = 0x0020;
        const IC3F_2 = 0x0040;
        const IC3F_3 = 0x0080;
        const IC3PSC = 0x000C;
        const IC3PSC_0 = 0x0004;
        const IC3PSC_1 = 0x0008;
        const OC3CE = 0x0080;
        const OC3M = 0x0070;
        const OC3M_0 = 0x0010;
        const OC3M_1 = 0x0020;
        const OC3M_2 = 0x0040;
        const OC3PE = 0x0008;
        const OC3FE = 0x0004;
        const CC3S = 0x0003;
        const CC3S_0 = 0x0001;
        const CC3S_1 = 0x0002;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CCER: u16 {
        const CC4P = 0x2000;
        const CC4E = 0x1000;
        const CC3NP = 0x0800;
        const CC3NE = 0x0400;
        const CC3P = 0x0200;
        const CC3E = 0x0100;
        const CC2NP = 0x0080;
        const CC2NE = 0x0040;
        const CC2P = 0x0020;
        const CC2E = 0x0010;
        const CC1NP = 0x0008;
        const CC1NE = 0x0004;
        const CC1P = 0x0002;
        const CC1E = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BDTR: u16 {
        const MOE = 0x8000;
        const AOE = 0x4000;
        const BKP = 0x2000;
        const BKE = 0x1000;
        const OSSR = 0x0800;
        const OSSI = 0x0400;
        const LOCK = 0x0300;
        const LOCK_0 = 0x0100;
        const LOCK_1 = 0x0200;
        const DTG = 0x00FF;
        const DTG_0 = 0x0001;
        const DTG_1 = 0x0002;
        const DTG_2 = 0x0004;
        const DTG_3 = 0x0008;
        const DTG_4 = 0x0010;
        const DTG_5 = 0x0020;
        const DTG_6 = 0x0040;
        const DTG_7 = 0x0080;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DMACFGR: u16 {
        const DBL = 0x1F00;
        const DBL_0 = 0x0100;
        const DBL_1 = 0x0200;
        const DBL_2 = 0x0400;
        const DBL_3 = 0x0800;
        const DBL_4 = 0x1000;
        const DBA = 0x001F;
        const DBA_0 = 0x0001;
        const DBA_1 = 0x0002;
        const DBA_2 = 0x0004;
        const DBA_3 = 0x0008;
        const DBA_4 = 0x0010;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AUX: u16 {
        const DT_VLU2 = 0xFF00;
        const DTN_MODE = 0x0080;
        const DT_MODE = 0x0040;
        const BK_SEL = 0x0038;
        const BK_SEL_0 = 0x0008;
        const BK_SEL_1 = 0x0010;
        const BK_SEL_2 = 0x0020;
        const CAPCH4_ED = 0x0004;
        const CAPCH3_ED = 0x0002;
        const CAPCH2_ED = 0x0001;
    }
}

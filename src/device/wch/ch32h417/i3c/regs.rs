use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct I3C {
    pub CTLR: Reg<CTLR>,
    pub CFGR: Reg<CFGR>,
    RESERVED0: Reserved<[u32; 2]>,
    pub RDBR: Reg<RDBR>,
    pub RDWR: Reg<RDWR>,
    pub TDBR: Reg<TDBR>,
    pub TDWR: Reg<TDWR>,
    pub IBIDR: Reg<IBIDR>,
    pub TGTTDR: Reg<TGTTDR>,
    RESERVED1: Reserved<u32>,
    pub RESET: Reg<RESET>,
    pub STATR: Reg<STATR>,
    pub STATER: Reg<STATER>,
    RESERVED2: Reserved<[u32; 2]>,
    pub RMR: Reg<RMR>,
    RESERVED3: Reserved<[u32; 3]>,
    pub EVR: Reg<EVR>,
    pub INTENR: Reg<INTENR>,
    pub CEVR: Reg<CEVR>,
    RESERVED4: Reserved<u32>,
    pub DEVR0: Reg<DEVR0>,
    pub DEVR1: Reg<DEVR1>,
    pub DEVR2: Reg<DEVR2>,
    pub DEVR3: Reg<DEVR3>,
    pub DEVR4: Reg<DEVR4>,
    RESERVED5: Reserved<[u32; 7]>,
    pub MAXRLR: Reg<MAXRLR>,
    pub MAXWLR: Reg<MAXWLR>,
    RESERVED6: Reserved<[u32; 2]>,
    pub TIMINGR0: Reg<TIMINGR0>,
    pub TIMINGR1: Reg<TIMINGR1>,
    pub TIMINGR2: Reg<TIMINGR2>,
    RESERVED7: Reserved<[u32; 5]>,
    pub BCR: Reg32,
    pub DCR: Reg<DCR>,
    pub GETCAPR: Reg<GETCAPR>,
    pub CRCAPR: Reg<CRCAPR>,
    pub GETMDSR: Reg<GETMDSR>,
    pub EPIDR: Reg<EPIDR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const MEND = 1 << 31;

        const MTYPE_MASK = 0xF << 27;

        const ADD_MASK = 0x7F << 17;

        const CCC_MASK = 0xFF << 16;

        const RNW = 1 << 16;

        const DCNT_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const TSFSET = 1 << 30;

        const CFLUSH = 1 << 21;
        const CDMAEN = 1 << 20;
        const TMODE = 1 << 19;
        const SMODE = 1 << 18;
        const SFLUSH = 1 << 17;
        const SDMAEN = 1 << 16;

        const TXTHRES = 1 << 14;
        const TXFLUSH = 1 << 13;
        const TXMAEN = 1 << 12;

        const RXTHRES = 1 << 10;
        const RXFLUSH = 1 << 9;
        const RXDMAEN = 1 << 8;
        const HJACK = 1 << 7;

        const EXITPTRN = 1 << 4;
        const RSTPTRN = 1 << 3;
        const NOARBH = 1 << 2;
        const CRINIT = 1 << 1;
        const EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDBR: u32 {
        const RDB0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDWR: u32 {
        const RDB3_MASK = 0xFF << 24;

        const RDB2_MASK = 0xFF << 16;

        const RDB1_MASK = 0xFF << 8;

        const RDB0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TDBR: u32 {
        const TDB0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TDWR: u32 {
        const TDB3_MASK = 0xFF << 24;

        const TDB2_MASK = 0xFF << 16;

        const TDB1_MASK = 0xFF << 8;

        const TDB0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IBIDR: u32 {
        const IBIDB3_MASK = 0xFF << 24;

        const IBIDB2_MASK = 0xFF << 16;

        const IBIDB1_MASK = 0xFF << 8;

        const IBIDB0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TGTTDR: u32 {
        const PRELOAD = 1 << 16;

        const TGTTDCNT_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RESET: u32 {
        const TGT_SIE_RST = 1 << 25;
        const HST_SIE_RST = 1 << 24;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const MID_MASK = 0xFF << 24;

        const DIR = 1 << 18;
        const ABT = 1 << 17;

        const XDCNT_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATER: u32 {
        const DERR = 1 << 10;
        const DNACK = 1 << 9;
        const ANACK = 1 << 8;
        const COVR = 1 << 7;
        const DOVR = 1 << 6;
        const STALL = 1 << 5;
        const PERR = 1 << 4;

        const CODERR_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RMR: u32 {
        const RADD_MASK = 0x7F << 17;

        const RCODE_MASK = 0xFF << 8;

        const IBIRDCNT_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EVR: u32 {
        const GRPF = 1 << 31;
        const DEFF = 1 << 30;
        const INTUPDF = 1 << 29;
        const ASUPDF = 1 << 28;
        const RSTF = 1 << 27;
        const MRLUPDF = 1 << 26;
        const MWLUPDF = 1 << 25;
        const DAUPDF = 1 << 24;
        const STAF = 1 << 23;
        const GETF = 1 << 22;
        const WKPF = 1 << 21;
        const HJF = 1 << 19;
        const CRUPDF = 1 << 18;
        const CRF = 1 << 17;
        const IBIENDF = 1 << 16;
        const IBIF = 1 << 15;
        const ERRF = 1 << 11;
        const RXTGTENDF = 1 << 10;
        const FCF = 1 << 9;
        const RXLASTF = 1 << 7;
        const TXLASTF = 1 << 6;
        const RXFNEF = 1 << 5;
        const TXFNFF = 1 << 4;
        const SFNEF = 1 << 3;
        const CFNFF = 1 << 2;
        const TXFEF = 1 << 1;
        const CFEF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTENR: u32 {
        const GRPIE = 1 << 31;
        const DEFIE = 1 << 30;
        const INTUPDIE = 1 << 29;
        const ASUPDIE = 1 << 28;
        const RSTIE = 1 << 27;
        const MRLUPDIE = 1 << 26;
        const MWLUPDIE = 1 << 25;
        const DAUPDIE = 1 << 24;
        const STAIE = 1 << 23;
        const GETIE = 1 << 22;
        const WKPIE = 1 << 21;
        const HJIE = 1 << 19;
        const CRUPDIE = 1 << 18;
        const CRIE = 1 << 17;
        const IBIENDIE = 1 << 16;
        const IBIIE = 1 << 15;
        const ERRIE = 1 << 11;
        const RXTGTENDIE = 1 << 10;
        const FCIE = 1 << 9;
        const RXFNEIE = 1 << 5;
        const TXFNEIE = 1 << 4;
        const SFNEIE = 1 << 3;
        const CFNFIE = 1 << 2;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CEVR: u32 {
        const CGRPF = 1 << 31;
        const CDEFF = 1 << 30;
        const CINTUPDF = 1 << 29;
        const CASUPDF = 1 << 28;
        const CRSTF = 1 << 27;
        const CMRLUPDF = 1 << 26;
        const CMWLUPDF = 1 << 25;
        const CDAUPDF = 1 << 24;
        const CSTAF = 1 << 23;
        const CGETF = 1 << 22;
        const CWKPF = 1 << 21;
        const CHJF = 1 << 19;
        const CCRUPDF = 1 << 18;
        const CCRF = 1 << 17;
        const CIBIENDF = 1 << 16;
        const CIBIF = 1 << 15;
        const CERRF = 1 << 11;
        const CRXTGTENDF = 1 << 10;
        const CFCR = 1 << 9;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR0: u32 {
        const RSTVAL = 1 << 24;

        const RSTACT_MASK = 0x3 << 22;

        const AS_MASK = 0x3 << 20;

        const HJEN = 1 << 19;
        const CREN = 1 << 17;
        const IBIEN = 1 << 16;

        const DA_MASK = 0x7F << 1;

        const DAVAL = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR1: u32 {
        const DIS = 1 << 31;

        const SUSP = 1 << 19;
        const IBIDEN = 1 << 18;
        const CRACK = 1 << 17;
        const IBIACK = 1 << 16;

        const DA_MASK = 0x7F << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR2: u32 {
        const DIS = 1 << 31;

        const SUSP = 1 << 19;
        const IBIDEN = 1 << 18;
        const CRACK = 1 << 17;
        const IBIACK = 1 << 16;

        const DA_MASK = 0x7F << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR3: u32 {
        const DIS = 1 << 31;

        const SUSP = 1 << 19;
        const IBIDEN = 1 << 18;
        const CRACK = 1 << 17;
        const IBIACK = 1 << 16;

        const DA_MASK = 0x7F << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR4: u32 {
        const DIS = 1 << 31;

        const SUSP = 1 << 19;
        const IBIDEN = 1 << 18;
        const CRACK = 1 << 17;
        const IBIACK = 1 << 16;

        const DA_MASK = 0x7F << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MAXRLR: u32 {
        const IBIP_MASK = 0x7 << 16;

        const MRL_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MAXWLR: u32 {
        const MWL_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMINGR0: u32 {
        const SCLH_I2C_MASK = 0xFF << 24;

        const SCLL_OD_MASK = 0xFF << 16;

        const SCLH_I3C_MASK = 0xFF << 8;

        const SCLL_PP_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMINGR1: u32 {
        const SDA_HD = 1 << 28;

        const FREE_MASK = 0x7F << 16;

        const ASNCR_MASK = 0x3 << 8;

        const AVAL_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMINGR2: u32 {
        const STALL_MASK = 0xFF << 8;

        const STALLA = 1 << 3;
        const STALLC = 1 << 2;
        const STALLD = 1 << 1;
        const STALLT = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DCR: u32 {
        const DCR_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GETCAPR: u32 {
        const CAPPEND = 1 << 14;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CRCAPR: u32 {
        const CAPGRP = 1 << 9;
        const CAPDHOFF = 1 << 3;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GETMDSR: u32 {
        const TSCO = 1 << 24;

        const RDTURN_MASK = 0xFF << 16;

        const FMT_MASK = 0x3 << 8;

        const HOFFAS_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EPIDR: u32 {
        const MIPIMID_MASK = 0x7FFF << 17;

        const IDTSEL = 1 << 16;

        const MIPIID_MASK = 0xF << 12;
    }
}

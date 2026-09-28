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
        const DCNT = 0x0000FFFF;
        const RNW = 0x00010000;
        const ADD = 0x00FE0000;

        const CCC = 0x00FF0000;

        const MTYPE = 0x78000000;
        const MTYPE_0 = 0x08000000;
        const MTYPE_1 = 0x10000000;
        const MTYPE_2 = 0x20000000;
        const MTYPE_3 = 0x40000000;

        const MEND = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const EN = 0x00000001;
        const CRINIT = 0x00000002;
        const NOARBH = 0x00000004;
        const RSTPTRN = 0x00000008;
        const EXITPTRN = 0x00000010;

        const HJACK = 0x00000080;
        const RXDMAEN = 0x00000100;
        const RXFLUSH = 0x00000200;
        const RXTHRES = 0x00000400;

        const TXMAEN = 0x00001000;
        const TXFLUSH = 0x00002000;
        const TXTHRES = 0x00004000;

        const SDMAEN = 0x00010000;
        const SFLUSH = 0x00020000;
        const SMODE = 0x00040000;
        const TMODE = 0x00080000;
        const CDMAEN = 0x00100000;
        const CFLUSH = 0x00200000;

        const TSFSET = 0x40000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDBR: u32 {
        const RDB0 = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDWR: u32 {
        const RDB0 = 0x000000FF;
        const RDB1 = 0x0000FF00;
        const RDB2 = 0x00FF0000;
        const RDB3 = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TDBR: u32 {
        const TDB0 = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TDWR: u32 {
        const TDB0 = 0x000000FF;
        const TDB1 = 0x0000FF00;
        const TDB2 = 0x00FF0000;
        const TDB3 = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IBIDR: u32 {
        const IBIDB0 = 0x000000FF;
        const IBIDB1 = 0x0000FF00;
        const IBIDB2 = 0x00FF0000;
        const IBIDB3 = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TGTTDR: u32 {
        const TGTTDCNT = 0x0000FFFF;
        const PRELOAD = 0x00010000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RESET: u32 {
        const HST_SIE_RST = 0x01000000;
        const TGT_SIE_RST = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const XDCNT = 0x0000FFFF;
        const ABT = 0x00020000;
        const DIR = 0x00040000;

        const MID = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATER: u32 {
        const CODERR = 0x0000000F;
        const PERR = 0x00000010;
        const STALL = 0x00000020;
        const DOVR = 0x00000040;
        const COVR = 0x00000080;
        const ANACK = 0x00000100;
        const DNACK = 0x00000200;
        const DERR = 0x00000400;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RMR: u32 {
        const IBIRDCNT = 0x00000007;
        const RCODE = 0x0000FF00;
        const RADD = 0x00FE0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EVR: u32 {
        const CFEF = 0x00000001;
        const TXFEF = 0x00000002;
        const CFNFF = 0x00000004;
        const SFNEF = 0x00000008;
        const TXFNFF = 0x00000010;
        const RXFNEF = 0x00000020;
        const TXLASTF = 0x00000040;
        const RXLASTF = 0x00000080;

        const FCF = 0x00000200;
        const RXTGTENDF = 0x00000400;
        const ERRF = 0x00000800;

        const IBIF = 0x00008000;
        const IBIENDF = 0x00010000;
        const CRF = 0x00020000;
        const CRUPDF = 0x00040000;
        const HJF = 0x00080000;
        const WKPF = 0x00200000;
        const GETF = 0x00400000;
        const STAF = 0x00800000;
        const DAUPDF = 0x01000000;
        const MWLUPDF = 0x02000000;
        const MRLUPDF = 0x04000000;
        const RSTF = 0x08000000;
        const ASUPDF = 0x10000000;
        const INTUPDF = 0x20000000;
        const DEFF = 0x40000000;
        const GRPF = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTENR: u32 {
        const CFNFIE = 0x00000004;
        const SFNEIE = 0x00000008;
        const TXFNEIE = 0x00000010;
        const RXFNEIE = 0x00000020;

        const FCIE = 0x00000200;
        const RXTGTENDIE = 0x00000400;
        const ERRIE = 0x00000800;

        const IBIIE = 0x00008000;
        const IBIENDIE = 0x00010000;
        const CRIE = 0x00020000;
        const CRUPDIE = 0x00040000;
        const HJIE = 0x00080000;

        const WKPIE = 0x00200000;
        const GETIE = 0x00400000;
        const STAIE = 0x00800000;
        const DAUPDIE = 0x01000000;
        const MWLUPDIE = 0x02000000;
        const MRLUPDIE = 0x04000000;
        const RSTIE = 0x08000000;
        const ASUPDIE = 0x10000000;
        const INTUPDIE = 0x20000000;
        const DEFIE = 0x40000000;
        const GRPIE = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CEVR: u32 {
        const CFCR = 0x00000200;
        const CRXTGTENDF = 0x00000400;
        const CERRF = 0x00000800;

        const CIBIF = 0x00008000;
        const CIBIENDF = 0x00010000;
        const CCRF = 0x00020000;
        const CCRUPDF = 0x00040000;
        const CHJF = 0x00080000;

        const CWKPF = 0x00200000;
        const CGETF = 0x00400000;
        const CSTAF = 0x00800000;
        const CDAUPDF = 0x01000000;
        const CMWLUPDF = 0x02000000;
        const CMRLUPDF = 0x04000000;
        const CRSTF = 0x08000000;
        const CASUPDF = 0x10000000;
        const CINTUPDF = 0x20000000;
        const CDEFF = 0x40000000;
        const CGRPF = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR0: u32 {
        const DAVAL = 0x00000001;
        const DA = 0x000000FE;

        const IBIEN = 0x00010000;
        const CREN = 0x00020000;

        const HJEN = 0x00080000;

        const AS = 0x00300000;
        const AS_0 = 0x00100000;
        const AS_1 = 0x00200000;

        const RSTACT = 0x00C00000;
        const RSTACT_0 = 0x00400000;
        const RSTACT_1 = 0x00800000;

        const RSTVAL = 0x01000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR1: u32 {
        const DA = 0x000000FE;

        const IBIACK = 0x00010000;
        const CRACK = 0x00020000;
        const IBIDEN = 0x00040000;
        const SUSP = 0x00080000;

        const DIS = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR2: u32 {
        const DA = 0x000000FE;

        const IBIACK = 0x00010000;
        const CRACK = 0x00020000;
        const IBIDEN = 0x00040000;
        const SUSP = 0x00080000;

        const DIS = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR3: u32 {
        const DA = 0x000000FE;

        const IBIACK = 0x00010000;
        const CRACK = 0x00020000;
        const IBIDEN = 0x00040000;
        const SUSP = 0x00080000;

        const DIS = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEVR4: u32 {
        const DA = 0x000000FE;

        const IBIACK = 0x00010000;
        const CRACK = 0x00020000;
        const IBIDEN = 0x00040000;
        const SUSP = 0x00080000;

        const DIS = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MAXRLR: u32 {
        const MRL = 0x0000FFFF;

        const IBIP = 0x00070000;
        const IBIP_0 = 0x00010000;
        const IBIP_1 = 0x00020000;
        const IBIP_2 = 0x00040000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MAXWLR: u32 {
        const MWL = 0x0000FFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMINGR0: u32 {
        const SCLL_PP = 0x000000FF;
        const SCLH_I3C = 0x0000FF00;
        const SCLL_OD = 0x00FF0000;
        const SCLH_I2C = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMINGR1: u32 {
        const AVAL = 0x000000FF;
        const ASNCR = 0x00000300;

        const FREE = 0x007F0000;
        const SDA_HD = 0x10000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMINGR2: u32 {
        const STALLT = 0x00000001;
        const STALLD = 0x00000002;
        const STALLC = 0x00000004;
        const STALLA = 0x00000008;

        const STALL = 0x0000FF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DCR: u32 {
        const DCR = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GETCAPR: u32 {
        const CAPPEND = 0x00004000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CRCAPR: u32 {
        const CAPDHOFF = 0x00000008;
        const CAPGRP = 0x00000200;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GETMDSR: u32 {
        const HOFFAS = 0x00000003;
        const HOFFAS_0 = 0x00000001;
        const HOFFAS_1 = 0x00000002;

        const FMT = 0x00000300;
        const FMT_0 = 0x00000100;
        const FMT_1 = 0x00000200;

        const RDTURN = 0x00FF0000;
        const TSCO = 0x01000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EPIDR: u32 {
        const MIPIID = 0x0000F000;
        const IDTSEL = 0x00010000;
        const MIPIMID = 0xFFFE0000;
    }
}

use bitflags::bitflags;

use crate::register::{Reg, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct SPI {
    pub CTLR1: Reg<CTLR1>,
    RESERVED0: Reserved<u16>,
    pub CTLR2: Reg<CTLR2>,
    RESERVED1: Reserved<u16>,
    pub STATR: Reg<STATR>,
    RESERVED2: Reserved<u16>,
    pub DATAR: Reg<DATAR>,
    RESERVED3: Reserved<u16>,
    pub CRCR: Reg<CRCR>,
    RESERVED4: Reserved<u16>,
    pub RCRCR: Reg<RCRCR>,
    RESERVED5: Reserved<u16>,
    pub TCRCR: Reg<TCRCR>,
    RESERVED6: Reserved<u16>,
    pub I2SCFGR: Reg<I2SCFGR>,
    RESERVED7: Reserved<u16>,
    pub I2SPR: Reg<I2SPR>,
    RESERVED8: Reserved<u16>,
    pub HSCR: Reg<HSCR>,
    RESERVED9: Reserved<u16>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u16 {
        const BIDIMODE = 1 << 15;
        const BIDIOE = 1 << 14;
        const CRCEN = 1 << 13;
        const CRCNEXT = 1 << 12;
        const DFF = 1 << 11;
        const RXONLY = 1 << 10;
        const SSM = 1 << 9;
        const SSI = 1 << 8;
        const LSBFIRST = 1 << 7;
        const SPE = 1 << 6;

        const BR_MASK = 0x7 << 3;

        const MSTR = 1 << 2;
        const CPOL = 1 << 1;
        const CPHA = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u16 {
        const TXEIE = 1 << 7;
        const RXNEIE = 1 << 6;
        const ERRIE = 1 << 5;
        const SSOE = 1 << 2;
        const TXDMAEN = 1 << 1;
        const RXDMAEN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u16 {
        const BSY = 1 << 7;
        const OVR = 1 << 6;
        const MODF = 1 << 5;
        const CRCERR = 1 << 4;
        const UDR = 1 << 3;
        const CHSIDE = 1 << 2;
        const TXE = 1 << 1;
        const RXNE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATAR: u16 {
        const DR_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CRCR: u16 {
        const CRCPOLY_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RCRCR: u16 {
        const RXCRC_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TCRCR: u16 {
        const TXCRC_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct I2SCFGR: u16 {
        const I2SMOD = 1 << 11;
        const I2SE = 1 << 10;

        const I2SCFG_MASK = 0x3 << 8;

        const PCMSYNC = 1 << 7;

        const I2SSTD_MASK = 0x3 << 4;

        const CKPOL = 1 << 3;

        const DATLEN_MASK = 0x3 << 1;

        const CHLEN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct I2SPR: u16 {
        const MCKOE = 1 << 9;
        const ODD = 1 << 8;

        const I2SDIV_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HSCR: u16 {
        const HSRXEN2 = 1 << 2;
        const HSRXEN = 1 << 0;
    }
}

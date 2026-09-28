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
        const CPHA = 0x0001;
        const CPOL = 0x0002;
        const MSTR = 0x0004;

        const BR = 0x0038;
        const BR_0 = 0x0008;
        const BR_1 = 0x0010;
        const BR_2 = 0x0020;

        const SPE = 0x0040;
        const LSBFIRST = 0x0080;
        const SSI = 0x0100;
        const SSM = 0x0200;
        const RXONLY = 0x0400;
        const DFF = 0x0800;
        const CRCNEXT = 0x1000;
        const CRCEN = 0x2000;
        const BIDIOE = 0x4000;
        const BIDIMODE = 0x8000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u16 {
        const RXDMAEN = 0x01;
        const TXDMAEN = 0x02;
        const SSOE = 0x04;
        const ERRIE = 0x20;
        const RXNEIE = 0x40;
        const TXEIE = 0x80;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u16 {
        const RXNE = 0x01;
        const TXE = 0x02;
        const CHSIDE = 0x04;
        const UDR = 0x08;
        const CRCERR = 0x10;
        const MODF = 0x20;
        const OVR = 0x40;
        const BSY = 0x80;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATAR: u16 {
        const DR = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CRCR: u16 {
        const CRCPOLY = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RCRCR: u16 {
        const RXCRC = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TCRCR: u16 {
        const TXCRC = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct I2SCFGR: u16 {
        const CHLEN = 0x0001;

        const DATLEN = 0x0006;
        const DATLEN_0 = 0x0002;
        const DATLEN_1 = 0x0004;

        const CKPOL = 0x0008;

        const I2SSTD = 0x0030;
        const I2SSTD_0 = 0x0010;
        const I2SSTD_1 = 0x0020;

        const PCMSYNC = 0x0080;

        const I2SCFG = 0x0300;
        const I2SCFG_0 = 0x0100;
        const I2SCFG_1 = 0x0200;

        const I2SE = 0x0400;
        const I2SMOD = 0x0800;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct I2SPR: u16 {
        const I2SDIV = 0x00FF;
        const ODD = 0x0100;
        const MCKOE = 0x0200;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HSCR: u16 {
        const HSRXEN = 0x0001;
        const HSRXEN2 = 0x0004;
    }
}

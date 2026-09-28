use bitflags::bitflags;

use crate::register::{Reg, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct I2C {
    pub CTLR1: Reg<CTLR1>,
    RESERVED0: Reserved<u16>,
    pub CTLR2: Reg<CTLR2>,
    RESERVED1: Reserved<u16>,
    pub OADDR1: Reg<OADDR1>,
    RESERVED2: Reserved<u16>,
    pub OADDR2: Reg<OADDR2>,
    RESERVED3: Reserved<u16>,
    pub DATAR: Reg<DATAR>,
    RESERVED4: Reserved<u16>,
    pub STAR1: Reg<STAR1>,
    RESERVED5: Reserved<u16>,
    pub STAR2: Reg<STAR2>,
    RESERVED6: Reserved<u16>,
    pub CKCFGR: Reg<CKCFGR>,
    RESERVED7: Reserved<u16>,
    pub RTR: Reg<RTR>,
    RESERVED8: Reserved<u16>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u16 {
        const SWRST = 1 << 15;
        const ALERT = 1 << 13;
        const PEC = 1 << 12;
        const POS = 1 << 11;
        const ACK = 1 << 10;
        const STOP = 1 << 9;
        const START = 1 << 8;
        const NOSTRETCH = 1 << 7;
        const ENGC = 1 << 6;
        const ENPEC = 1 << 5;
        const ENARP = 1 << 4;
        const SMBTYPE = 1 << 3;
        const SMBUS = 1 << 1;
        const PE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u16 {
        const LAST = 1 << 12;
        const DMAEN = 1 << 11;
        const ITBUFEN = 1 << 10;
        const ITEVTEN = 1 << 9;
        const ITERREN = 1 << 8;

        const FREQ_MASK = 0x3F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OADDR1: u16 {
        const ADDMODE = 1 << 15;

        const ADD9 = 1 << 9;
        const ADD8 = 1 << 8;
        const ADD8_9_MASK = 0x3 << 8;

        const ADD7 = 1 << 7;
        const ADD6 = 1 << 6;
        const ADD5 = 1 << 5;
        const ADD4 = 1 << 4;
        const ADD3 = 1 << 3;
        const ADD2 = 1 << 2;
        const ADD1 = 1 << 1;
        const ADD1_7_MASK = 0x7F << 1;

        const ADD0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OADDR2: u16 {
        const ADD2_MASK = 0x7F << 1;

        const ENDUAL = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATAR: u16 {
        const DR_DATAR_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STAR1: u16 {
        const SMBALERT = 1 << 15;
        const TIMEOUT = 1 << 14;
        const PECERR = 1 << 12;
        const OVR = 1 << 11;
        const AF = 1 << 10;
        const ARLO = 1 << 9;
        const BERR = 1 << 8;
        const TXE = 1 << 7;
        const RXNE = 1 << 6;
        const STOPF = 1 << 4;
        const ADD10 = 1 << 3;
        const BTF = 1 << 2;
        const ADDR = 1 << 1;
        const SB = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STAR2: u16 {
        const PEC_MASK = 0xFF << 8;

        const DUALF = 1 << 7;
        const SMBHOST = 1 << 6;
        const SMBDEFAULT = 1 << 5;
        const GENCALL = 1 << 4;
        const TRA = 1 << 2;
        const BUSY = 1 << 1;
        const MSL = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CKCFGR: u16 {
        const FS = 1 << 15;
        const DUTY = 1 << 14;

        const CCR_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTR: u16 {
        const TRISE_MASK = 0x3F << 0;
    }
}

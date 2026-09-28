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
        const PE = 0x0001;
        const SMBUS = 0x0002;
        const SMBTYPE = 0x0008;
        const ENARP = 0x0010;
        const ENPEC = 0x0020;
        const ENGC = 0x0040;
        const NOSTRETCH = 0x0080;
        const START = 0x0100;
        const STOP = 0x0200;
        const ACK = 0x0400;
        const POS = 0x0800;
        const PEC = 0x1000;
        const ALERT = 0x2000;
        const SWRST = 0x8000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u16 {
        const FREQ = 0x003F;
        const FREQ_0 = 0x0001;
        const FREQ_1 = 0x0002;
        const FREQ_2 = 0x0004;
        const FREQ_3 = 0x0008;
        const FREQ_4 = 0x0010;
        const FREQ_5 = 0x0020;

        const ITERREN = 0x0100;
        const ITEVTEN = 0x0200;
        const ITBUFEN = 0x0400;
        const DMAEN = 0x0800;
        const LAST = 0x1000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OADDR1: u16 {
        const ADD0 = 0x0001;
        const ADD1_7 = 0x00FE;
        const ADD8_9 = 0x0300;

        const ADD1 = 0x0002;
        const ADD2 = 0x0004;
        const ADD3 = 0x0008;
        const ADD4 = 0x0010;
        const ADD5 = 0x0020;
        const ADD6 = 0x0040;
        const ADD7 = 0x0080;
        const ADD8 = 0x0100;
        const ADD9 = 0x0200;

        const ADDMODE = 0x8000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OADDR2: u16 {
        const ENDUAL = 0x01;
        const ADD2 = 0xFE;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATAR: u16 {
        const DATAR = 0xFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STAR1: u16 {
        const SB = 0x0001;
        const ADDR = 0x0002;
        const BTF = 0x0004;
        const ADD10 = 0x0008;
        const STOPF = 0x0010;
        const RXNE = 0x0040;
        const TXE = 0x0080;
        const BERR = 0x0100;
        const ARLO = 0x0200;
        const AF = 0x0400;
        const OVR = 0x0800;
        const PECERR = 0x1000;
        const TIMEOUT = 0x4000;
        const SMBALERT = 0x8000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STAR2: u16 {
        const MSL = 0x0001;
        const BUSY = 0x0002;
        const TRA = 0x0004;
        const GENCALL = 0x0010;
        const SMBDEFAULT = 0x0020;
        const SMBHOST = 0x0040;
        const DUALF = 0x0080;
        const PEC = 0xFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CKCFGR: u16 {
        const CCR = 0x0FFF;
        const DUTY = 0x4000;
        const FS = 0x8000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTR: u16 {
        const TRISE = 0x3F;
    }
}

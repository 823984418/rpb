use bitflags::bitflags;

use crate::register::{Reg, Reg32};

#[derive(Debug)]
#[repr(C)]
pub struct FLASH {
    pub ACTLR: Reg<ACTLR>,
    pub KEYR: Reg<KEYR>,
    pub OBKEYR: Reg<OBKEYR>,
    pub STATR: Reg<STATR>,
    pub CTLR: Reg<CTLR>,
    pub ADDR: Reg32,
    RESERVED0: Reg32,
    pub OBR: Reg<OBR>,
    pub WPR: Reg32,
    pub MODEKEYR: Reg<MODEKEYR>,
    pub BOOT_MODEKEYR: Reg<BOOT_MODEKEYR>,
    pub CFGR0: Reg<CFGR0>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ACTLR: u32 {
        const FLASH_ST = 1 << 15;
        const FLASH_READY = 1 << 14;
        const FLASH_RD_MD = 1 << 11;
        const FLASH_LP = 1 << 8;
        const EHMOD = 1 << 7;
        const ENHANCE_STATUSOD = 1 << 6;

        const SCK_CFG_4 = 0b10 << 0;
        const SCK_CFG_2 = 0b01 << 0;
        const SCK_CFG_1 = 0b00 << 0;
        const SCK_CFG = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEYR: u32 {
        const RDPRT = 0x000000A5;
        const KEY1 = 0x45670123;
        const KEY2 = 0xCDEF89AB;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OBKEYR: u32 {
        const KEY1 = 0x45670123;
        const KEY2 = 0xCDEF89AB;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const BOOT_LOCK = 1 << 15;
        const BOOT_MODE = 1 << 14;
        const BOOT_STATUS = 1 << 13;
        const BOOT_AVA = 1 << 12;
        const EOP = 1 << 5;
        const WRPRTERR = 1 << 4;
        const WRBSY = 1 << 1;
        const BSY = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const RSENACT = 1 << 22;
        const PGSTRT = 1 << 21;
        const BER = 1 << 18;
        const FTPG = 1 << 16;
        const FLOCK = 1 << 15;
        const EOPIE = 1 << 12;
        const ERRIE = 1 << 10;
        const OBWRE = 1 << 9;
        const LOCK = 1 << 7;
        const STRT = 1 << 6;
        const OBER = 1 << 5;
        const OBPG = 1 << 4;
        const PER = 1 << 1;
        const PG = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OBR: u32 {

    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MODEKEYR: u32 {
        const KEY1 = 0x45670123;
        const KEY2 = 0xCDEF89AB;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BOOT_MODEKEYR: u32 {
        const KEY1 = 0x45670123;
        const KEY2 = 0xCDEF89AB;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR0: u32 {
        const DBMODE = 1 << 28;
    }
}

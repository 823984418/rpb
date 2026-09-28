use bitflags::bitflags;

use crate::register::{Reg, Reg16, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct FLASH {
    pub ACTLR: Reg<ACTLR>,
    pub KEYR: Reg<KEYR>,
    pub OBKEYR: Reg<OBKEYR>,
    pub STATR: Reg<STATR>,
    pub CTLR: Reg<CTLR>,
    pub ADDR: Reg<ADDR>,
    RESERVED: Reserved<u32>,
    pub OBR: Reg<OBR>,
    pub WPR: Reg<WPR>,
    pub MODEKEYR: Reg<MODEKEYR>,
    pub BOOT_MODEKEYR: Reg<BOOT_MODEKEYR>,
}

#[derive(Debug)]
#[repr(C)]
pub struct OB {
    pub RDPR: Reg16,
    pub USER: Reg16,
    pub Data0: Reg16,
    pub Data1: Reg16,
    pub WRPR0: Reg16,
    pub WRPR1: Reg16,
    pub WRPR2: Reg16,
    pub WRPR3: Reg16,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ACTLR: u32 {
        const ST = 1 << 15;
        const FLASH_ST = 1 << 15;
        const READY = 1 << 14;
        const FLASH_READY = 1 << 14;
        const RD_MD = 1 << 11;
        const FLASH_RD_MD = 1 << 11;
        const LP = 1 << 8;
        const FLASH_LP = 1 << 8;
        const EHMOD = 1 << 7;
        const ENHANCE_STATUS = 1 << 6;
        const ENHANCE_STATUSOD = 1 << 6;

        const LATENCY_HCLK_DIV4 = 0b10 << 0;
        const LATENCY_HCLK_DIV2 = 0b01 << 0;
        const LATENCY_HCLK_DIV1 = 0b00 << 0;
        const SCK_CFG_4 = 0b10 << 0;
        const SCK_CFG_2 = 0b01 << 0;
        const SCK_CFG_1 = 0b00 << 0;
        const SCK_CFG_0 = 0b01 << 0;
        const SCK_CFG = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEYR: u32 {
        const FKEYR = 0xFFFFFFFF;
        const RDPRT = 0x000000A5;
        const KEY1 = 0x45670123;
        const KEY2 = 0xCDEF89AB;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OBKEYR: u32 {
        const OBKEYR = 0xFFFFFFFF;
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
        const PG_STRT = 1 << 21;
        const PGSTRT = 1 << 21;
        const PAGE_BER32 = 1 << 18;
        const BER = 1 << 18;
        const PAGE_PG = 1 << 16;
        const FTPG = 1 << 16;
        const FAST_LOCK = 1 << 15;
        const FLOCK = 1 << 15;
        const EOPIE = 1 << 12;
        const ERRIE = 1 << 10;
        const OPTWRE = 1 << 9;
        const OBWRE = 1 << 9;
        const LOCK = 1 << 7;
        const STRT = 1 << 6;
        const OPTER = 1 << 5;
        const OBER = 1 << 5;
        const OPTPG = 1 << 4;
        const OBPG = 1 << 4;
        const PER = 1 << 1;
        const PG = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR: u32 {
        const FAR = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OBR: u32 {
        const DATA1 = 0x03FC0000;
        const DATA0 = 0x0003FC00;
        const USER = 0x00000304;
        const FIX_11 = 0x00000300;
        const WDG_SW = 0x00000004;
        const RDPRT = 0x00000002;
        const OPTERR = 0x00000001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WPR: u32 {
        const WRP = 0xFFFFFFFF;
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

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDPR: u32 {
        const nRDPR = 0x0000FF00;
        const RDPR = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct USER: u32 {
        const nUSER = 0xFF000000;
        const USER = 0x00FF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Data0: u32 {
        const nData0 = 0x0000FF00;
        const Data0 = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Data1: u32 {
        const nData1 = 0xFF000000;
        const Data1 = 0x00FF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WRPR0: u32 {
        const nWRPR0 = 0x0000FF00;
        const WRPR0 = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WRPR1: u32 {
        const nWRPR1 = 0xFF000000;
        const WRPR1 = 0x00FF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WRPR2: u32 {
        const nWRPR2 = 0x0000FF00;
        const WRPR2 = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WRPR3: u32 {
        const nWRPR3 = 0xFF000000;
        const WRPR3 = 0x00FF0000;
    }
}

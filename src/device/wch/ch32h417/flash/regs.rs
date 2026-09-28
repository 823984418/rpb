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
        const READY = 1 << 14;
        const RD_MD = 1 << 11;
        const LP = 1 << 8;
        const EHMOD = 1 << 7;
        const ENHANCE_STATUS = 1 << 6;

        const SCK_CFG_1 = 1 << 1;
        const SCK_CFG_0 = 1 << 0;
        const SCK_CFG_MASK = 0x3 << 0;

        const LATENCY_HCLK_DIV4 = 0b10 << 0;
        const LATENCY_HCLK_DIV2 = 0b01 << 0;
        const LATENCY_HCLK_DIV1 = 0b00 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEYR: u32 {
        const FKEYR_MASK = 0xFFFFFFFF << 0;

        const KEY1 = 0x45670123;
        const KEY2 = 0xCDEF89AB;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OBKEYR: u32 {
        const OBKEYR_MASK = 0xFFFFFFFF << 0;
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
        const PAGE_BER32 = 1 << 18;
        const PAGE_PG = 1 << 16;
        const FAST_LOCK = 1 << 15;
        const EOPIE = 1 << 12;
        const ERRIE = 1 << 10;
        const OPTWRE = 1 << 9;
        const LOCK = 1 << 7;
        const STRT = 1 << 6;
        const OPTER = 1 << 5;
        const OPTPG = 1 << 4;
        const PER = 1 << 1;
        const PG = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR: u32 {
        const FAR_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OBR: u32 {
        const DATA1_MASK = 0x3FC << 18;

        const DATA0_MASK = 0x3FC << 10;

        const USER_MASK = 0x304 << 0;

        const FIX_11_MASK = 0x3 << 8;

        const WDG_SW = 1 << 2;

        const RDPRT = 1 << 1;
        const OPTERR = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WPR: u32 {
        const WRP_MASK = 0xFFFFFFFF << 0;
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
        const nRDPR_MASK = 0xFF << 8;
        const RDPR_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct USER: u32 {
        const nUSER_MASK = 0xFF << 24;
        const USER_MASK = 0xFF << 16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Data0: u32 {
        const nData0_MASK = 0xFF << 8;
        const Data0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Data1: u32 {
        const nData1_MASK = 0xFF << 24;
        const Data1_MASK = 0xFF << 16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WRPR0: u32 {
        const nWRPR0_MASK = 0xFF << 8;
        const WRPR0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WRPR1: u32 {
        const nWRPR1_MASK = 0xFF << 24;
        const WRPR1_MASK = 0xFF << 16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WRPR2: u32 {
        const nWRPR2_MASK = 0xFF << 8;
        const WRPR2_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WRPR3: u32 {
        const nWRPR3_MASK = 0xFF << 24;
        const WRPR3_MASK = 0xFF << 16;
    }
}

use bitflags::bitflags;

use crate::register::{Reg, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct ECDC {
    pub CTRL: Reg<CTRL>,
    pub INT_FG: Reg<INT_FG>,
    pub KEY_255T224: Reg<KEY_255T224>,
    pub KEY_223T192: Reg<KEY_223T192>,
    pub KEY_191T160: Reg<KEY_191T160>,
    pub KEY_159T128: Reg<KEY_159T128>,
    pub KEY_127T96: Reg<KEY_127T96>,
    pub KEY_95T64: Reg<KEY_95T64>,
    pub KEY_63T32: Reg<KEY_63T32>,
    pub KEY_31T0: Reg<KEY_31T0>,
    pub IV_127T96: Reg<IV_127T96>,
    pub IV_95T64: Reg<IV_95T64>,
    pub IV_63T32: Reg<IV_63T32>,
    pub IV_31T0: Reg<IV_31T0>,
    RESERVED3: Reserved<[u32; 2]>,
    pub SGSD_127T96: Reg<SGSD_127T96>,
    pub SGSD_95T64: Reg<SGSD_95T64>,
    pub SGSD_63T32: Reg<SGSD_63T32>,
    pub SGSD_31T0: Reg<SGSD_31T0>,
    pub SGRT_127T96: Reg<SGRT_127T96>,
    pub SGRT_95T64: Reg<SGRT_95T64>,
    pub SGRT_63T32: Reg<SGRT_63T32>,
    pub SGRT_31T0: Reg<SGRT_31T0>,
    pub SRC_ADDR: Reg<SRC_ADDR>,
    pub DST_ADDR: Reg<DST_ADDR>,
    pub SRAM_LEN: Reg<SRAM_LEN>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL: u32 {
        const ECDC_SM4_CLOCK_EN = 1 << 25;
        const CLOCK_SELECT = 1 << 24;
        const IE_WRSRAM = 1 << 18;
        const IE_SINGLE = 1 << 17;

        const DAT_MOD = 1 << 13;

        const KLEN_MASK = 0x3 << 10;

        const CIPHER_MOD = 1 << 9;
        const ALGRM_MOD = 1 << 8;
        const WRSRAM_EN = 1 << 7;

        const CLKDIV_MASK = 0x7 << 4;

        const MODE_SEL = 1 << 3;
        const NORMAL_EN = 1 << 1;
        const KEYEX_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FG: u32 {
        const IF_WRSRAM = 1 << 18;
        const IF_SINGLE = 1 << 17;
        const IF_EKDONE = 1 << 16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_255T224: u32 {
        const KEY_255T224_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_223T192: u32 {
        const KEY_223T192_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_191T160: u32 {
        const KEY_191T160_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_159T128: u32 {
        const KEY_159T128_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_127T96: u32 {
        const KEY_127T96_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_95T64: u32 {
        const KEY_95T64_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_63T32: u32 {
        const KEY_63T32_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_31T0: u32 {
        const KEY_31T0_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IV_127T96: u32 {
        const IV_127T96_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IV_95T64: u32 {
        const IV_95T64_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IV_63T32: u32 {
        const IV_63T32_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IV_31T0: u32 {
        const IV_31T0_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGSD_127T96: u32 {
        const SGSD_127T96_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGSD_95T64: u32 {
        const SGSD_95T64_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGSD_63T32: u32 {
        const SGSD_63T32_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGSD_31T0: u32 {
        const SGSD_31T0_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGRT_127T96: u32 {
        const SGRT_127T96_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGRT_95T64: u32 {
        const SGRT_95T64_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGRT_63T32: u32 {
        const SGRT_63T32_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGRT_31T0: u32 {
        const SGRT_31T0_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SRC_ADDR: u32 {
        const SRAM_SRC_ADDR_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DST_ADDR: u32 {
        const SRAM_DST_ADDR_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SRAM_LEN: u32 {
        const SRAM_LEN_MASK = 0xFFFFFFFF << 0;
    }
}

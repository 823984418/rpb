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
        const KEYEX_EN = 0x00000001;
        const NORMAL_EN = 0x00000002;
        const MODE_SEL = 0x00000008;

        const CLKDIV_MASK = 0x00000070;

        const WRSRAM_EN = 0x00000080;
        const ALGRM_MOD = 0x00000100;
        const CIPHER_MOD = 0x00000200;

        const KLEN_MASK = 0x00000C00;

        const DAT_MOD = 0x00002000;

        const IE_SINGLE = 0x00020000;
        const IE_WRSRAM = 0x00040000;

        const CLOCK_SELECT = 0x01000000;
        const ECDC_SM4_CLOCK_EN = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FG: u32 {
        const IF_EKDONE = 0x00010000;
        const IF_SINGLE = 0x00020000;
        const IF_WRSRAM = 0x00040000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_255T224: u32 {
        const KEY_255T224 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_223T192: u32 {
        const KEY_223T192 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_191T160: u32 {
        const KEY_191T160 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_159T128: u32 {
        const KEY_159T128 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_127T96: u32 {
        const KEY_127T96 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_95T64: u32 {
        const KEY_95T64 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_63T32: u32 {
        const KEY_63T32 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY_31T0: u32 {
        const KEY_31T0 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IV_127T96: u32 {
        const IV_127T96 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IV_95T64: u32 {
        const IV_95T64 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IV_63T32: u32 {
        const IV_63T32 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IV_31T0: u32 {
        const IV_31T0 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGSD_127T96: u32 {
        const SGSD_127T96 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGSD_95T64: u32 {
        const SGSD_95T64 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGSD_63T32: u32 {
        const SGSD_63T32 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGSD_31T0: u32 {
        const SGSD_31T0 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGRT_127T96: u32 {
        const SGRT_127T96 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGRT_95T64: u32 {
        const SGRT_95T64 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGRT_63T32: u32 {
        const SGRT_63T32 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SGRT_31T0: u32 {
        const SGRT_31T0 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SRC_ADDR: u32 {
        const SRAM_SRC_ADDR = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DST_ADDR: u32 {
        const SRAM_DST_ADDR = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SRAM_LEN: u32 {
        const SRAM_LEN = 0xFFFFFFFF;
    }
}

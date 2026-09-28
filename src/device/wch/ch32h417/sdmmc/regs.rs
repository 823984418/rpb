use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct SDMMC {
    pub ARGUMENT: Reg32,
    pub CMD_SET: Reg<CMD_SET>,
    RESERVED0: Reserved<u16>,
    pub RESPONSE0: Reg32,
    pub RESPONSE1: Reg32,
    pub RESPONSE2: Reg32,
    // union { RESPONSE3; WRITE_CONT; }
    pub RESPONSE3: Reg32,
    pub CONTROL: Reg<CONTROL>,
    RESERVED1: Reserved<u16>,
    pub TIMEOUT: Reg<TIMEOUT>,
    RESERVED3: Reserved<[u8; 3]>,
    pub STATUS: Reg<STATUS>,
    pub INT_FG: Reg<INT_FG>,
    RESERVED4: Reserved<u16>,
    pub INT_EN: Reg<INT_EN>,
    RESERVED5: Reserved<u16>,
    pub DMA_BEG1: Reg32,
    pub BLOCK_CFG: Reg<BLOCK_CFG>,
    pub TRAN_MODE: Reg<TRAN_MODE>,
    pub CLK_DIV: Reg<CLK_DIV>,
    RESERVED6: Reserved<u16>,
    pub DMA_BEG2: Reg32,
    pub TUNE_DATO: Reg<TUNE_DATO>,
    pub TUNE_DATI: Reg<TUNE_DATI>,
    pub TUNE_CLK_CMD: Reg<TUNE_CLK_CMD>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMD_SET: u16 {
        const CKIDX = 1 << 11;
        const CKCRC = 1 << 10;

        const RPTY_MASK = 0x3 << 8;

        const CMDIDX_MASK = 0x3F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CONTROL: u16 {
        const SLV_FORCE_ERR = 1 << 9;
        const SLV_MODE = 1 << 8;
        const NEGSMP = 1 << 5;
        const RST_LGC = 1 << 4;
        const DMAEN = 1 << 3;
        const ALL_CLR = 1 << 2;

        const LW_MASK_1 = 1 << 1;
        const LW_MASK_0 = 1 << 0;
        const LW_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMEOUT: u8 {
        const TOCNT_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATUS: u32 {
        const DAT0STA = 1 << 17;
        const CMDSTA = 1 << 16;

        const MASK_BLOCK_NUM = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FG: u16 {
        const SIF_SLV_BUF_RELEAS = 1 << 10;
        const IF_SDIOINT = 1 << 9;
        const IF_FIFO_OV = 1 << 8;
        const IF_BKGAP = 1 << 7;
        const IF_TRANDONE = 1 << 6;
        const IF_TRANERR = 1 << 5;
        const IF_DATTMO = 1 << 4;
        const IF_CMDDONE = 1 << 3;
        const IF_REIDX_ER = 1 << 2;
        const IF_RECRC_WR = 1 << 1;
        const IF_RE_TMOUT = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_EN: u16 {
        const IE_SDIOINT = 1 << 8;
        const IE_FIFO_OV = 1 << 7;
        const IE_TRANDONE = 1 << 6;
        const IE_TRANERR = 1 << 5;
        const IE_DATTMO = 1 << 4;
        const IE_CMDDONE = 1 << 3;
        const IE_REIDX_ER = 1 << 2;
        const IE_RECRC_WR = 1 << 1;
        const IE_RE_TMOUT = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BLOCK_CFG: u32 {
        const BKSIZE_MASK = 0xFFF << 16;
        const BKNUM_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TRAN_MODE: u32 {
        const SW_1 = 1 << 20;
        const SW_0 = 1 << 19;
        const SW_MASK = 0x3 << 19;

        const CARE_NEG = 1 << 18;
        const DDR_MODE = 1 << 17;
        const DULEDMA_EN = 1 << 16;

        const DMATN_CNT_MASK = 0x7F << 8;

        const AUTOGAPSTOP = 1 << 4;
        const MODE_BOOT = 1 << 2;
        const GAP_STOP = 1 << 1;
        const DMA_DIR = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CLK_DIV: u16 {
        const PHASEINV = 1 << 10;
        const CLKMode = 1 << 9;
        const CLKOE = 1 << 8;

        const DIV_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TUNE_DATO: u32 {
        const TUNNE_DAT7_O_MASK = 0xF << 28;
        const TUNNE_DAT6_O_MASK = 0xF << 24;
        const TUNNE_DAT5_O_MASK = 0xF << 20;
        const TUNNE_DAT4_O_MASK = 0xF << 16;
        const TUNNE_DAT3_O_MASK = 0xF << 12;
        const TUNNE_DAT2_O_MASK = 0xF << 8;
        const TUNNE_DAT1_O_MASK = 0xF << 4;
        const TUNNE_DAT0_O_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TUNE_DATI: u32 {
        const TUNNE_DAT7_I_MASK = 0xF << 28;
        const TUNNE_DAT6_I_MASK = 0xF << 24;
        const TUNNE_DAT5_I_MASK = 0xF << 20;
        const TUNNE_DAT4_I_MASK = 0xF << 16;
        const TUNNE_DAT3_I_MASK = 0xF << 12;
        const TUNNE_DAT2_I_MASK = 0xF << 8;
        const TUNNE_DAT1_I_MASK = 0xF << 4;
        const TUNNE_DAT0_I_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TUNE_CLK_CMD: u32 {
        const TUNNE_CMD_I_MASK = 0xF << 20;
        const TUNNE_CMD_O_MASK = 0xF << 16;
        const TUNNE_CLK_I_MASK = 0xF << 4;
        const TUNNE_CLK_O_MASK = 0xF << 0;
    }
}

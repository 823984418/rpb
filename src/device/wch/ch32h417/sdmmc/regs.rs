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
        const CMDIDX_MASK = 0x003F;
        const RPTY_MASK = 0x0300;
        const CKCRC = 0x0400;
        const CKIDX = 0x0800;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CONTROL: u16 {
        const LW_MASK = 0x0003;
        const LW_MASK_0 = 0x0001;
        const LW_MASK_1 = 0x0002;

        const ALL_CLR = 0x0004;
        const DMAEN = 0x0008;
        const RST_LGC = 0x0010;
        const NEGSMP = 0x0020;
        const SLV_MODE = 0x0100;
        const SLV_FORCE_ERR = 0x0200;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMEOUT: u8 {
        const TOCNT_MASK = 0x0F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATUS: u32 {
        const MASK_BLOCK_NUM = 0x0000FFFF;
        const CMDSTA = 0x00010000;
        const DAT0STA = 0x00020000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FG: u16 {
        const IF_RE_TMOUT = 0x0001;
        const IF_RECRC_WR = 0x0002;
        const IF_REIDX_ER = 0x0004;
        const IF_CMDDONE = 0x0008;
        const IF_DATTMO = 0x0010;
        const IF_TRANERR = 0x0020;
        const IF_TRANDONE = 0x0040;
        const IF_BKGAP = 0x0080;
        const IF_FIFO_OV = 0x0100;
        const IF_SDIOINT = 0x0200;
        const SIF_SLV_BUF_RELEAS = 0x0400;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_EN: u16 {
        const IE_RE_TMOUT = 0x0001;
        const IE_RECRC_WR = 0x0002;
        const IE_REIDX_ER = 0x0004;
        const IE_CMDDONE = 0x0008;
        const IE_DATTMO = 0x0010;
        const IE_TRANERR = 0x0020;
        const IE_TRANDONE = 0x0040;
        const IE_FIFO_OV = 0x0080;
        const IE_SDIOINT = 0x0100;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BLOCK_CFG: u32 {
        const BKNUM_MASK = 0x0000FFFF;
        const BKSIZE_MASK = 0x0FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TRAN_MODE: u32 {
        const DMA_DIR = 0x00000001;
        const GAP_STOP = 0x00000002;
        const MODE_BOOT = 0x00000004;
        const AUTOGAPSTOP = 0x00000010;

        const DMATN_CNT = 0x00007F00;

        const DULEDMA_EN = 0x00010000;
        const DDR_MODE = 0x00020000;
        const CARE_NEG = 0x00040000;

        const SW = 0x00180000;
        const SW_0 = 0x00080000;
        const SW_1 = 0x00100000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CLK_DIV: u16 {
        const DIV_MASK = 0x001F;
        const CLKOE = 0x0100;
        const CLKMode = 0x0200;
        const PHASEINV = 0x0400;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TUNE_DATO: u32 {
        const TUNNE_DAT0_O = 0x0000000F;
        const TUNNE_DAT1_O = 0x000000F0;
        const TUNNE_DAT2_O = 0x00000F00;
        const TUNNE_DAT3_O = 0x0000F000;
        const TUNNE_DAT4_O = 0x000F0000;
        const TUNNE_DAT5_O = 0x00F00000;
        const TUNNE_DAT6_O = 0x0F000000;
        const TUNNE_DAT7_O = 0xF0000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TUNE_DATI: u32 {
        const TUNNE_DAT0_I = 0x0000000F;
        const TUNNE_DAT1_I = 0x000000F0;
        const TUNNE_DAT2_I = 0x00000F00;
        const TUNNE_DAT3_I = 0x0000F000;
        const TUNNE_DAT4_I = 0x000F0000;
        const TUNNE_DAT5_I = 0x00F00000;
        const TUNNE_DAT6_I = 0x0F000000;
        const TUNNE_DAT7_I = 0xF0000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TUNE_CLK_CMD: u32 {
        const TUNNE_CLK_O = 0x0000000F;
        const TUNNE_CLK_I = 0x000000F0;
        const TUNNE_CMD_O = 0x000F0000;
        const TUNNE_CMD_I = 0x00F00000;
    }
}

use bitflags::bitflags;

use crate::register::{Reg, Reg16, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct DVP {
    pub CR0: Reg<CR0>,
    pub CR1: Reg<CR1>,
    pub IER: Reg<IER>,
    Reserved0: Reserved<u8>,
    pub ROW_NUM: Reg16,
    pub COL_NUM: Reg16,
    pub DMA_BUF0: Reg32,
    pub DMA_BUF1: Reg32,
    pub IFR: Reg<IFR>,
    pub STATUS: Reg<STATUS>,
    Reserved1: Reserved<u16>,
    pub ROW_CNT: Reg16,
    Reserved2: Reserved<u16>,
    pub HOFFCNT: Reg16,
    pub VST: Reg16,
    pub CAPCNT: Reg16,
    pub VLINE: Reg16,
    pub DR: Reg32,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR0: u8 {
        const JPEG = 1 << 6;

        const D12_MOD = 0b10 << 4;
        const D10_MOD = 0b01 << 4;
        const D8_MOD = 0b00 << 4;
        const MSK_DAT_MOD_MASK = 0x3 << 4;

        const P_POLAR = 1 << 3;
        const H_POLAR = 1 << 2;
        const V_POLAR = 1 << 1;
        const ENABLE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR1: u8 {
        const CROP = 1 << 5;
        const CM = 1 << 4;
        const BUF_TOG = 1 << 3;
        const RCV_CLR = 1 << 2;
        const ALL_CLR = 1 << 1;
        const DMA_EN = 1 << 0;

        const RATE_25P = 0b10 << 6;
        const RATE_50P = 0b01 << 6;
        const RATE_100P = 0b00 << 6;
        const FCRC_MASK = 0x3 << 6;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u8 {
        const IE_STP_FRM = 1 << 4;
        const IE_FIFO_OV = 1 << 3;
        const IE_FRM_DONE = 1 << 2;
        const IE_ROW_DONE = 1 << 1;
        const IE_STR_FRM = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IFR: u8 {
        const IF_STP_FRM = 1 << 4;
        const IF_FIFO_OV = 1 << 3;
        const IF_FRM_DONE = 1 << 2;
        const IF_ROW_DONE = 1 << 1;
        const IF_STR_FRM = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATUS: u8 {
        const MSK_FIFO_CNT_MASK = 0x7 << 4;

        const FIFO_OV = 1 << 2;
        const FIFO_FULL = 1 << 1;
        const FIFO_RDY = 1 << 0;
    }
}

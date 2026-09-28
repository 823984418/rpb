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
        const ENABLE = 0x01;
        const V_POLAR = 0x02;
        const H_POLAR = 0x04;
        const P_POLAR = 0x08;
        const MSK_DAT_MOD = 0x30;
        const D8_MOD = 0x00;
        const D10_MOD = 0x10;
        const D12_MOD = 0x20;
        const JPEG = 0x40;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR1: u8 {
        const DMA_EN = 0x01;
        const ALL_CLR = 0x02;
        const RCV_CLR = 0x04;
        const BUF_TOG = 0x08;
        const CM = 0x10;
        const CROP = 0x20;
        const FCRC = 0xC0;
        const RATE_100P = 0x00;
        const RATE_50P = 0x40;
        const RATE_25P = 0x80;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u8 {
        const IE_STR_FRM = 0x01;
        const IE_ROW_DONE = 0x02;
        const IE_FRM_DONE = 0x04;
        const IE_FIFO_OV = 0x08;
        const IE_STP_FRM = 0x10;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IFR: u8 {
        const IF_STR_FRM = 0x01;
        const IF_ROW_DONE = 0x02;
        const IF_FRM_DONE = 0x04;
        const IF_FIFO_OV = 0x08;
        const IF_STP_FRM = 0x10;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATUS: u8 {
        const FIFO_RDY = 0x01;
        const FIFO_FULL = 0x02;
        const FIFO_OV = 0x04;
        const MSK_FIFO_CNT = 0x70;
    }
}

use bitflags::bitflags;

use crate::register::{Reg, Reg32};

#[derive(Debug)]
#[repr(C)]
pub struct SDS {
    pub CTRL: Reg<CTRL>,
    pub INT_EN: Reg<INT_EN>,
    pub STATUS: Reg<STATUS>,
    pub RTX_CTRL: Reg<RTX_CTRL>,
    pub RX_LEN0: Reg<RX_LEN0>,
    pub DATA0: Reg32,
    pub DMA_0: Reg32,
    pub RX_LEN1: Reg<RX_LEN1>,
    pub DATA1: Reg32,
    pub DMA_1: Reg32,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL: u32 {
        const ALIGN_EN = 1 << 18;
        const CONT_EN = 1 << 17;
        const PHY_PWR_UP = 1 << 16;
        const TX_PWR_UP = 1 << 15;
        const RX_PWR_UP = 1 << 14;
        const PLL_PWR_UP = 1 << 13;

        const PLL_FACTOR_MASK = 0x1F << 8;

        const DMA_EN = 1 << 7;
        const TX_EN = 1 << 6;
        const RX_EN = 1 << 5;
        const RX_POLARITY = 1 << 4;
        const INT_BUSY_EN = 1 << 3;
        const RESET_PHY = 1 << 2;
        const RESET_LINK = 1 << 1;
        const CLR_ALL = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_EN: u32 {
        const COMINIT_IE = 1 << 5;
        const FIFO_OV_IE = 1 << 3;
        const RECV_DONE_IE = 1 << 2;
        const TRAN_DONE_IE = 1 << 1;
        const RECV_ERR_IE = 1 << 1;
        const PHYRDY_IE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATUS: u32 {
        const TX_SEQ_NUM_MASK = 0xF << 28;

        const RX_SEQ_NUM_MASK = 0xF << 24;

        const R_FIFO_RDY = 1 << 21;
        const LINK_FREE = 1 << 20;
        const PLL_LOCK = 1 << 19;
        const RECV_CRC_OK = 1 << 18;
        const RX_SEQ_MATCH = 1 << 17;
        const PHYRDY = 1 << 16;

        const COMINIT_IF = 1 << 5;
        const FIFO_OV_IF = 1 << 3;
        const RECV_DONE_IF = 1 << 2;
        const TRAN_DONE_IF = 1 << 1;
        const RECV_ERR_IF = 1 << 1;
        const PHYRDY_IF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTX_CTRL: u32 {
        const BUF_MODE = 1 << 18;
        const TX_VLD = 1 << 17;
        const LINK_INIT = 1 << 16;

        const SERDES_TX_LEN_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RX_LEN0: u32 {
        const SERDES_RX_LEN0_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RX_LEN1: u32 {
        const SERDES_RX_LEN1_MASK = 0xFFFF << 0;
    }
}

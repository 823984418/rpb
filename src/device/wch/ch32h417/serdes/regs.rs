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
        const CLR_ALL = 0x00000001;
        const RESET_LINK = 0x00000002;
        const RESET_PHY = 0x00000004;
        const INT_BUSY_EN = 0x00000008;
        const RX_POLARITY = 0x00000010;
        const RX_EN = 0x00000020;
        const TX_EN = 0x00000040;
        const DMA_EN = 0x00000080;
        const PLL_FACTOR = 0x00001F00;
        const PLL_PWR_UP = 0x00002000;
        const RX_PWR_UP = 0x00004000;
        const TX_PWR_UP = 0x00008000;
        const PHY_PWR_UP = 0x00010000;
        const CONT_EN = 0x00020000;
        const ALIGN_EN = 0x00040000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_EN: u32 {
        const PHYRDY_IE = 0x00000001;
        const RECV_ERR_IE = 0x00000002;
        const TRAN_DONE_IE = 0x00000002;
        const RECV_DONE_IE = 0x00000004;
        const FIFO_OV_IE = 0x00000008;
        const COMINIT_IE = 0x00000020;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATUS: u32 {
        const PHYRDY_IF = 0x00000001;
        const RECV_ERR_IF = 0x00000002;
        const TRAN_DONE_IF = 0x00000002;
        const RECV_DONE_IF = 0x00000004;
        const FIFO_OV_IF = 0x00000008;
        const COMINIT_IF = 0x00000020;
        const PHYRDY = 0x00010000;
        const RX_SEQ_MATCH = 0x00020000;
        const RECV_CRC_OK = 0x00040000;
        const PLL_LOCK = 0x00080000;
        const LINK_FREE = 0x00100000;
        const R_FIFO_RDY = 0x00200000;
        const RX_SEQ_NUM = 0x0F000000;
        const TX_SEQ_NUM = 0xF0000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTX_CTRL: u32 {
        const SERDES_TX_LEN = 0x0000FFFF;
        const LINK_INIT = 0x00010000;
        const TX_VLD = 0x00020000;
        const BUF_MODE = 0x00040000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RX_LEN0: u32 {
        const SERDES_RX_LEN0 = 0x0000FFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RX_LEN1: u32 {
        const SERDES_RX_LEN1 = 0x0000FFFF;
    }
}

use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct HSADC {
    pub CFGR: Reg<CFGR>,
    pub CTLR1: Reg<CTLR1>,
    pub CTLR2: Reg<CTLR2>,
    pub STATR: Reg<STATR>,
    pub DATAR: Reg<DATAR>,
    pub ADDR0: Reg<ADDR0>,
    pub ADDR1: Reg<ADDR1>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const DMA_LEN_MASK = 0xFFFF << 16;

        const BURST_EN = 1 << 15;
        const PPMODE = 1 << 14;

        const CLKDIV_5 = 1 << 13;
        const CLKDIV_4 = 1 << 12;
        const CLKDIV_3 = 1 << 11;
        const CLKDIV_2 = 1 << 10;
        const CLKDIV_1 = 1 << 9;
        const CLKDIV_0 = 1 << 8;
        const CLKDIV_MASK = 0x3F << 8;

        const WIDTH = 1 << 7;

        const SETUP_1 = 1 << 6;
        const SETUP_0 = 1 << 5;
        const SETUP_MASK = 0x3 << 5;

        const CHSEL_2 = 1 << 4;
        const CHSEL_1 = 1 << 3;
        const CHSEL_0 = 1 << 2;
        const CHSEL_MASK = 0x7 << 2;

        const DMAEN = 1 << 1;
        const EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u32 {
        const BURSTIE = 1 << 10;
        const DMAIE = 1 << 9;
        const EOCIE = 1 << 8;
        const BURSTEND = 1 << 1;
        const START = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u32 {
        const BURST_DMA_LEN_MASK = 0xFFFF << 16;

        const BURST_LEN_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const FIFO_CNT_MASK = 0x7 << 11;

        const FIFO_OV = 1 << 10;
        const FIFO_FULL = 1 << 9;
        const FIFO_RDY = 1 << 8;
        const PP_ADDR = 1 << 4;
        const RXNE = 1 << 3;
        const BURSTIF = 1 << 2;
        const DMAIF = 1 << 1;
        const EOCIF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATAR: u32 {
        const DR_MASK = 0x3FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR0: u32 {
        const DMA_ADDR0_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR1: u32 {
        const DMA_ADDR1_MASK = 0xFFFFFFFF << 0;
    }
}

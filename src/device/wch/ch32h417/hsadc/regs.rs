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
        const EN = 0x00000001;
        const DMAEN = 0x00000002;

        const CHSEL = 0x0000001C;
        const CHSEL_0 = 0x00000004;
        const CHSEL_1 = 0x00000008;
        const CHSEL_2 = 0x00000010;

        const SETUP = 0x00000060;
        const SETUP_0 = 0x00000020;
        const SETUP_1 = 0x00000040;

        const WIDTH = 0x00000080;

        const CLKDIV = 0x00003F00;
        const CLKDIV_0 = 0x00000100;
        const CLKDIV_1 = 0x00000200;
        const CLKDIV_2 = 0x00000400;
        const CLKDIV_3 = 0x00000800;
        const CLKDIV_4 = 0x00001000;
        const CLKDIV_5 = 0x00002000;

        const PPMODE = 0x00004000;
        const BURST_EN = 0x00008000;

        const DMA_LEN = 0xFFFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u32 {
        const START = 0x00000001;
        const BURSTEND = 0x00000002;
        const EOCIE = 0x00000100;
        const DMAIE = 0x00000200;
        const BURSTIE = 0x00000400;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u32 {
        const BURST_LEN = 0x0000FFFF;
        const BURST_DMA_LEN = 0xFFFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const EOCIF = 0x00000001;
        const DMAIF = 0x00000002;
        const BURSTIF = 0x00000004;
        const RXNE = 0x00000008;
        const PP_ADDR = 0x00000010;

        const FIFO_RDY = 0x00000100;
        const FIFO_FULL = 0x00000200;
        const FIFO_OV = 0x00000400;

        const FIFO_CNT = 0x00003800;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATAR: u32 {
        const DR = 0x000003FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR0: u32 {
        const DMA_ADDR0 = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR1: u32 {
        const DMA_ADDR1 = 0xFFFFFFFF;
    }
}

use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct DMA_Channel {
    pub CFGR: Reg<CFGR>,
    pub CNTR: Reg<CNTR>,
    pub PADDR: Reg<PADDR>,
    pub MADDR: Reg<MADDR>,
    pub M1ADDR: Reg<M1ADDR>,
}

#[derive(Debug)]
#[repr(C)]
pub struct DMA {
    pub INTFR: Reg<INTFR>,
    pub INTFCR: Reg<INTFCR>,
    pub channel: [DMA_Channel; 8],
}

#[derive(Debug)]
#[repr(C)]
pub struct DMAMUX {
    pub CFGR0_3: Reg<CFGR0_3>,
    pub CFGR4_7: Reg<CFGR4_7>,
    pub CFGR8_11: Reg<CFGR8_11>,
    pub CFGR12_15: Reg<CFGR12_15>,
}

impl DMAMUX {
    pub fn CFGR(&self) -> &[Reg<DMAMUX_CFGR>; 16] {
        unsafe { &*(self as *const Self as *const [Reg<DMAMUX_CFGR>; 16]) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFR: u32 {
        const TEIF8 = 1 << 31;
        const HTIF8 = 1 << 30;
        const TCIF8 = 1 << 29;
        const GIF8 = 1 << 28;
        const TEIF7 = 1 << 27;
        const HTIF7 = 1 << 26;
        const TCIF7 = 1 << 25;
        const GIF7 = 1 << 24;
        const TEIF6 = 1 << 23;
        const HTIF6 = 1 << 22;
        const TCIF6 = 1 << 21;
        const GIF6 = 1 << 20;
        const TEIF5 = 1 << 19;
        const HTIF5 = 1 << 18;
        const TCIF5 = 1 << 17;
        const GIF5 = 1 << 16;
        const TEIF4 = 1 << 15;
        const HTIF4 = 1 << 14;
        const TCIF4 = 1 << 13;
        const GIF4 = 1 << 12;
        const TEIF3 = 1 << 11;
        const HTIF3 = 1 << 10;
        const TCIF3 = 1 << 9;
        const GIF3 = 1 << 8;
        const TEIF2 = 1 << 7;
        const HTIF2 = 1 << 6;
        const TCIF2 = 1 << 5;
        const GIF2 = 1 << 4;
        const TEIF1 = 1 << 3;
        const HTIF1 = 1 << 2;
        const TCIF1 = 1 << 1;
        const GIF1 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFCR: u32 {
        const CTEIF8 = 1 << 31;
        const CHTIF8 = 1 << 30;
        const CTCIF8 = 1 << 29;
        const CGIF8 = 1 << 28;
        const CTEIF7 = 1 << 27;
        const CHTIF7 = 1 << 26;
        const CTCIF7 = 1 << 25;
        const CGIF7 = 1 << 24;
        const CTEIF6 = 1 << 23;
        const CHTIF6 = 1 << 22;
        const CTCIF6 = 1 << 21;
        const CGIF6 = 1 << 20;
        const CTEIF5 = 1 << 19;
        const CHTIF5 = 1 << 18;
        const CTCIF5 = 1 << 17;
        const CGIF5 = 1 << 16;
        const CTEIF4 = 1 << 15;
        const CHTIF4 = 1 << 14;
        const CTCIF4 = 1 << 13;
        const CGIF4 = 1 << 12;
        const CTEIF3 = 1 << 11;
        const CHTIF3 = 1 << 10;
        const CTCIF3 = 1 << 9;
        const CGIF3 = 1 << 8;
        const CTEIF2 = 1 << 7;
        const CHTIF2 = 1 << 6;
        const CTCIF2 = 1 << 5;
        const CGIF2 = 1 << 4;
        const CTEIF1 = 1 << 3;
        const CHTIF1 = 1 << 2;
        const CTCIF1 = 1 << 1;
        const CGIF1 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const FLAG_CURMEM = 1 << 16;
        const DOUBLE_MODE = 1 << 15;
        const MEM2MEM = 1 << 14;

        const PL_MASK = 0x3 << 12;

        const MSIZE_MASK = 0x3 << 10;

        const PSIZE_MASK = 0x3 << 8;

        const MINC = 1 << 7;
        const PINC = 1 << 6;
        const CIRC = 1 << 5;
        const DIR = 1 << 4;
        const TEIE = 1 << 3;
        const HTIE = 1 << 2;
        const TCIE = 1 << 1;
        const EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTR: u32 {
        const NDT_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PADDR: u32 {
        const PA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MADDR: u32 {
        const MA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct M1ADDR: u32 {
        const M1A_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DMAMUX_CFGR: u8 {
        const TIM7_UP = 123;
        const TIM6_UP = 122;
        const ADC2 = 121;
        const ADC1 = 120;

        const SAI_B_RX = 115;
        const SAI_B_TX = 114;
        const SAI_A_RX = 113;
        const SAI_A_TX = 112;
        const SDIO = 111;

        const DFSDM_DMA1 = 108;
        const DFSDM_DMA0 = 107;

        const DAC2 = 104;
        const DAC1 = 103;
        const SWPMI1_RX = 102;
        const SWPMI1_TX = 101;
        const USART8_RX = 100;
        const USART8_TX = 99;
        const USART7_RX = 98;
        const USART7_TX = 97;
        const USART6_RX = 96;
        const USART6_TX = 95;
        const USART5_RX = 94;
        const USART5_TX = 93;
        const USART4_RX = 92;
        const USART4_TX = 91;
        const USART3_RX = 90;
        const USART3_TX = 89;
        const USART2_RX = 88;
        const USART2_TX = 87;
        const USART1_RX = 86;
        const USART1_TX = 85;
        const I3C_RX = 84;
        const I3C_TX = 83;
        const I3C_TC = 82;
        const I3C_RS = 81;
        const I2C4_RX = 80;
        const I2C4_TX = 79;
        const I2C3_RX = 78;
        const I2C3_TX = 77;
        const I2C2_RX = 76;
        const I2C2_TX = 75;
        const I2C1_RX = 74;
        const I2C1_TX = 73;
        const QSPI2_DMA = 72;
        const QSPI1_DMA = 71;
        const SPI4_RX = 70;
        const SPI4_TX = 69;
        const SPI3_RX = 68;
        const SPI3_TX = 67;
        const SPI2_RX = 66;
        const SPI2_TX = 65;
        const SPI1_RX = 64;
        const SPI1_TX = 63;
        const TIM12_TRIG = 62;
        const TIM12_UP = 61;
        const TIM12_CH4 = 60;
        const TIM12_CH3 = 59;
        const TIM12_CH2 = 58;
        const TIM12_CH1 = 57;
        const TIM11_TRIG = 56;
        const TIM11_UP = 55;
        const TIM11_CH4 = 54;
        const TIM11_CH3 = 53;
        const TIM11_CH2 = 52;
        const TIM11_CH1 = 51;
        const TIM10_TRIG = 50;
        const TIM10_UP = 49;
        const TIM10_CH4 = 48;
        const TIM10_CH3 = 47;
        const TIM10_CH2 = 46;
        const TIM10_CH1 = 45;
        const TIM9_TRIG = 44;
        const TIM9_UP = 43;
        const TIM9_CH4 = 42;
        const TIM9_CH3 = 41;
        const TIM9_CH2 = 40;
        const TIM9_CH1 = 39;
        const TIM8_TRIG = 38;
        const TIM8_COM = 37;
        const TIM8_UP = 36;
        const TIM8_CH4 = 35;
        const TIM8_CH3 = 34;
        const TIM8_CH2 = 33;
        const TIM8_CH1 = 32;
        const TIM5_TRIG = 31;
        const TIM5_UP = 30;
        const TIM5_CH4 = 29;
        const TIM5_CH3 = 28;
        const TIM5_CH2 = 27;
        const TIM5_CH1 = 26;
        const TIM4_TRIG = 25;
        const TIM4_UP = 24;
        const TIM4_CH4 = 23;
        const TIM4_CH3 = 22;
        const TIM4_CH2 = 21;
        const TIM4_CH1 = 20;
        const TIM3_TRIG = 19;
        const TIM3_UP = 18;
        const TIM3_CH4 = 17;
        const TIM3_CH3 = 16;
        const TIM3_CH2 = 15;
        const TIM3_CH1 = 14;
        const TIM2_TRIG = 13;
        const TIM2_UP = 12;
        const TIM2_CH4 = 11;
        const TIM2_CH3 = 10;
        const TIM2_CH2 = 9;
        const TIM2_CH1 = 8;
        const TIM1_TRIG = 7;
        const TIM1_COM = 6;
        const TIM1_UP = 5;
        const TIM1_CH4 = 4;
        const TIM1_CH3 = 3;
        const TIM1_CH2 = 2;
        const TIM1_CH1 = 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR0_3: u32 {
        const CH3_MASK = 0x7F << 24;

        const CH2_MASK = 0x7F << 16;

        const CH1_MASK = 0x7F << 8;

        const CH0_MASK = 0x7F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR4_7: u32 {
        const CH7_MASK = 0x7F << 24;

        const CH6_MASK = 0x7F << 16;

        const CH5_MASK = 0x7F << 8;

        const CH4_MASK = 0x7F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR8_11: u32 {
        const CH11_MASK = 0x7F << 24;

        const CH10_MASK = 0x7F << 16;

        const CH9_MASK = 0x7F << 8;

        const CH8_MASK = 0x7F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR12_15: u32 {
        const CH15_MASK = 0x7F << 24;

        const CH14_MASK = 0x7F << 16;

        const CH13_MASK = 0x7F << 8;

        const CH12_MASK = 0x7F << 0;
    }
}

use bitflags::bitflags;

use crate::register::{Reg, Reg8};

#[derive(Debug)]
#[repr(C)]
pub struct RCC {
    pub CTLR: Reg<CTLR>,
    pub CFGR0: Reg<CFGR0>,
    pub PLLCFGR: Reg<PLLCFGR>,
    pub INTR: Reg<INTR>,
    pub HB2PRSTR: Reg<HB2PRSTR>,
    pub HB1PRSTR: Reg<HB1PRSTR>,
    pub HBPCENR: Reg<HBPCENR>,
    pub HB2PCENR: Reg<HB2PCENR>,
    pub HB1PCENR: Reg<HB1PCENR>,
    pub BDCTLR: Reg<BDCTLR>,
    pub RSTSCKR: Reg<RSTSCKR>,
    pub HBRSTR: Reg<HBRSTR>,
    pub CFGR2: Reg<CFGR2>,
    pub PLLCFGR2: Reg<PLLCFGR2>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const CSS_HSE_DIS = 1 << 31;
        const SERDES_PLLRDY = 1 << 29;
        const SERDES_PLLON = 1 << 28;
        const ETH_PLLRDY = 1 << 27;
        const ETH_PLLON = 1 << 26;
        const PLLRDY = 1 << 25;
        const PLLON = 1 << 24;
        const USBSS_PLLRDY = 1 << 23;
        const USBSS_PLLON = 1 << 22;
        const USBHS_PLLRDY = 1 << 21;
        const USBHS_PLLON = 1 << 20;
        const CSSON = 1 << 19;
        const HSEBYP = 1 << 18;
        const HSERDY = 1 << 17;
        const HSEON = 1 << 16;

        const HSICAL_MASK = 0xFF << 8;

        const HSITRIM_MASK = 0x1F << 3;

        const HSIRDY = 1 << 1;
        const HSION = 1 << 0;
    }
}

impl CTLR {
    pub fn HSITRIM(n: u32) -> Self {
        Self::from_bits_retain(n << 3) & Self::HSITRIM_MASK
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR0: u32 {
        const ADCSRC = 1 << 31;
        const ADC_DUTY_SEL = 1 << 30;

        const MCO_SERDES_DIV16 = 0b1011 << 24;
        const MCO_ETH_PLL_DIV8 = 0b1010 << 24;
        const MCO_USBSS_PLL_DIV2 = 0b1001 << 24;
        const MCO_UTMI = 0b1000 << 24;
        const MCO_PLL_DIV2 = 0b0111 << 24;
        const MCO_HSE = 0b0110 << 24;
        const MCO_HSI = 0b0101 << 24;
        const MCO_SYSCLK = 0b0100 << 24;
        const MCO_NOCLOCK = 0b0000 << 24;
        const MCO_MASK = 0xF << 24;

        const UTMION = 1 << 23;
        const PIPEON = 1 << 22;
        const RGMIION = 1 << 21;

        const FPRE_DIV4 = 0b10 << 16;
        const FPRE_DIV2 = 0b01 << 16;
        const FPRE_DIV1 = 0b00 << 16;
        const FPRE_MASK = 0x3 << 16;

        const ADCPRE_DIV36 = 0b11111 << 11;
        const ADCPRE_DIV35 = 0b11110 << 11;
        const ADCPRE_DIV34 = 0b11101 << 11;
        const ADCPRE_DIV33 = 0b11100 << 11;
        const ADCPRE_DIV32 = 0b11011 << 11;
        const ADCPRE_DIV31 = 0b11010 << 11;
        const ADCPRE_DIV30 = 0b11001 << 11;
        const ADCPRE_DIV29 = 0b11000 << 11;
        const ADCPRE_DIV28 = 0b10111 << 11;
        const ADCPRE_DIV27 = 0b10110 << 11;
        const ADCPRE_DIV26 = 0b10101 << 11;
        const ADCPRE_DIV25 = 0b10100 << 11;
        const ADCPRE_DIV24 = 0b10011 << 11;
        const ADCPRE_DIV23 = 0b10010 << 11;
        const ADCPRE_DIV22 = 0b10001 << 11;
        const ADCPRE_DIV21 = 0b10000 << 11;
        const ADCPRE_DIV20 = 0b01111 << 11;
        const ADCPRE_DIV19 = 0b01110 << 11;
        const ADCPRE_DIV18 = 0b01101 << 11;
        const ADCPRE_DIV17 = 0b01100 << 11;
        const ADCPRE_DIV16 = 0b01011 << 11;
        const ADCPRE_DIV15 = 0b01010 << 11;
        const ADCPRE_DIV14 = 0b01001 << 11;
        const ADCPRE_DIV13 = 0b01000 << 11;
        const ADCPRE_DIV12 = 0b00111 << 11;
        const ADCPRE_ADCH_DIV8 = 0b11 << 12;
        const ADCPRE_DIV11 = 0b00110 << 11;
        const ADCPRE_DIV10 = 0b00101 << 11;
        const ADCPRE_ADCH_DIV6 = 0b10 << 12;
        const ADCPRE_DIV9 = 0b00100 << 11;
        const ADCPRE_DIV8 = 0b00011 << 11;
        const ADCPRE_ADCH_DIV4 = 0b01 << 12;
        const ADCPRE_DIV7 = 0b00010 << 11;
        const ADCPRE_DIV6 = 0b00001 << 11;
        const ADCPRE_ADCH_DIV2 = 0b00 << 12;
        const ADCPRE_DIV5 = 0b00000 << 11;
        const ADCPRE_MASK = 0x3 << 14;

        const PPRE2_ADCL_DIV16 = 0b111 << 11;
        const PPRE2_ADCL_DIV8 = 0b110 << 11;
        const PPRE2_ADCL_DIV4 = 0b101 << 11;
        const PPRE2_ADCL_DIV2 = 0b100 << 11;
        const PPRE2_ADCL_DIV1 = 0b000 << 11;
        const PPRE2_MASK = 0x7 << 11;

        const HPRE_DIV512 = 0b1111 << 4;
        const HPRE_DIV256 = 0b1110 << 4;
        const HPRE_DIV128 = 0b1101 << 4;
        const HPRE_DIV64 = 0b1100 << 4;
        const HPRE_DIV16 = 0b1011 << 4;
        const HPRE_DIV8 = 0b1010 << 4;
        const HPRE_DIV4 = 0b1001 << 4;
        const HPRE_DIV2 = 0b1000 << 4;
        const HPRE_DIV1 = 0b0000 << 4;
        const HPRE_MASK = 0xF << 4;

        const SWS_PLL = 0b10 << 2;
        const SWS_HSE = 0b01 << 2;
        const SWS_HSI = 0b00 << 2;
        const SWS_MASK = 0x3 << 2;

        const SW_PLL = 0b10 << 0;
        const SW_HSE = 0b01 << 0;
        const SW_HSI = 0b00 << 0;
        const SW_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PLLCFGR: u32 {
        const SYSPLL_GATE = 1 << 31;

        const SYSPLL_SEL_USBSS = 0b111 << 28;
        const SYSPLL_SEL_SERDES = 0b110 << 28;
        const SYSPLL_SEL_ETH = 0b101 << 28;
        const SYSPLL_SEL_USBHS = 0b100 << 28;
        const SYSPLL_SEL_PLL = 0b000 << 28;
        const SYSPLL_SEL_MASK = 0x7 << 28;

        const PLL_SRC_DIV64 = 0b111111 << 8;
        const PLL_SRC_DIV63 = 0b111110 << 8;
        const PLL_SRC_DIV62 = 0b111101 << 8;
        const PLL_SRC_DIV61 = 0b111100 << 8;
        const PLL_SRC_DIV60 = 0b111011 << 8;
        const PLL_SRC_DIV59 = 0b111010 << 8;
        const PLL_SRC_DIV58 = 0b111001 << 8;
        const PLL_SRC_DIV57 = 0b111000 << 8;
        const PLL_SRC_DIV56 = 0b110111 << 8;
        const PLL_SRC_DIV55 = 0b110110 << 8;
        const PLL_SRC_DIV54 = 0b110101 << 8;
        const PLL_SRC_DIV53 = 0b110100 << 8;
        const PLL_SRC_DIV52 = 0b110011 << 8;
        const PLL_SRC_DIV51 = 0b110010 << 8;
        const PLL_SRC_DIV50 = 0b110001 << 8;
        const PLL_SRC_DIV49 = 0b110000 << 8;
        const PLL_SRC_DIV48 = 0b101111 << 8;
        const PLL_SRC_DIV47 = 0b101110 << 8;
        const PLL_SRC_DIV46 = 0b101101 << 8;
        const PLL_SRC_DIV45 = 0b101100 << 8;
        const PLL_SRC_DIV44 = 0b101011 << 8;
        const PLL_SRC_DIV43 = 0b101010 << 8;
        const PLL_SRC_DIV42 = 0b101001 << 8;
        const PLL_SRC_DIV41 = 0b101000 << 8;
        const PLL_SRC_DIV40 = 0b100111 << 8;
        const PLL_SRC_DIV39 = 0b100110 << 8;
        const PLL_SRC_DIV38 = 0b100101 << 8;
        const PLL_SRC_DIV37 = 0b100100 << 8;
        const PLL_SRC_DIV36 = 0b100011 << 8;
        const PLL_SRC_DIV35 = 0b100010 << 8;
        const PLL_SRC_DIV34 = 0b100001 << 8;
        const PLL_SRC_DIV33 = 0b100000 << 8;
        const PLL_SRC_DIV32 = 0b011111 << 8;
        const PLL_SRC_DIV31 = 0b011110 << 8;
        const PLL_SRC_DIV30 = 0b011101 << 8;
        const PLL_SRC_DIV29 = 0b011100 << 8;
        const PLL_SRC_DIV28 = 0b011011 << 8;
        const PLL_SRC_DIV27 = 0b011010 << 8;
        const PLL_SRC_DIV26 = 0b011001 << 8;
        const PLL_SRC_DIV25 = 0b011000 << 8;
        const PLL_SRC_DIV24 = 0b010111 << 8;
        const PLL_SRC_DIV23 = 0b010110 << 8;
        const PLL_SRC_DIV22 = 0b010101 << 8;
        const PLL_SRC_DIV21 = 0b010100 << 8;
        const PLL_SRC_DIV20 = 0b010011 << 8;
        const PLL_SRC_DIV19 = 0b010010 << 8;
        const PLL_SRC_DIV18 = 0b010001 << 8;
        const PLL_SRC_DIV17 = 0b010000 << 8;
        const PLL_SRC_DIV16 = 0b001111 << 8;
        const PLL_SRC_DIV15 = 0b001110 << 8;
        const PLL_SRC_DIV14 = 0b001101 << 8;
        const PLL_SRC_DIV13 = 0b001100 << 8;
        const PLL_SRC_DIV12 = 0b001011 << 8;
        const PLL_SRC_DIV11 = 0b001010 << 8;
        const PLL_SRC_DIV10 = 0b001001 << 8;
        const PLL_SRC_DIV9 = 0b001000 << 8;
        const PLL_SRC_DIV8 = 0b000111 << 8;
        const PLL_SRC_DIV7 = 0b000110 << 8;
        const PLL_SRC_DIV6 = 0b000101 << 8;
        const PLL_SRC_DIV5 = 0b000100 << 8;
        const PLL_SRC_DIV4 = 0b000011 << 8;
        const PLL_SRC_DIV3 = 0b000010 << 8;
        const PLL_SRC_DIV2 = 0b000001 << 8;
        const PLL_SRC_DIV1 = 0b000000 << 8;
        const PLL_SRC_DIV_MASK = 0x3F << 8;

        const PLLSRC_SERDES_PLL = 0b111 << 5;
        const PLLSRC_USBSS_PLL = 0b110 << 5;
        const PLLSRC_ETH_PLL = 0b101 << 5;
        const PLLSRC_USBHS_PLL = 0b100 << 5;
        const PLLSRC_HSE = 0b001 << 5;
        const PLLSRC_HSI = 0b000 << 5;
        const PLLSRC_MASK = 0x7 << 5;

        const PLLMUL59 = 0b11111 << 0;
        const PLLMUL40 = 0b11110 << 0;
        const PLLMUL38 = 0b11101 << 0;
        const PLLMUL36 = 0b11100 << 0;
        const PLLMUL34 = 0b11011 << 0;
        const PLLMUL32 = 0b11010 << 0;
        const PLLMUL30 = 0b11001 << 0;
        const PLLMUL28 = 0b11000 << 0;
        const PLLMUL26 = 0b10111 << 0;
        const PLLMUL24 = 0b10110 << 0;
        const PLLMUL22 = 0b10101 << 0;
        const PLLMUL20 = 0b10100 << 0;
        const PLLMUL19 = 0b10011 << 0;
        const PLLMUL18 = 0b10010 << 0;
        const PLLMUL17 = 0b10001 << 0;
        const PLLMUL16 = 0b10000 << 0;
        const PLLMUL15 = 0b01111 << 0;
        const PLLMUL14 = 0b01110 << 0;
        const PLLMUL13 = 0b01101 << 0;
        const PLLMUL12_5 = 0b01100 << 0;
        const PLLMUL12 = 0b01011 << 0;
        const PLLMUL11_5 = 0b01010 << 0;
        const PLLMUL11 = 0b01001 << 0;
        const PLLMUL10_5 = 0b01000 << 0;
        const PLLMUL10 = 0b00111 << 0;
        const PLLMUL9_5 = 0b00110 << 0;
        const PLLMUL9 = 0b00101 << 0;
        const PLLMUL8_5 = 0b00100 << 0;
        const PLLMUL8 = 0b00011 << 0;
        const PLLMUL7 = 0b00010 << 0;
        const PLLMUL6 = 0b00001 << 0;
        const PLLMUL4 = 0b00000 << 0;
        const PLLMUL_MASK = 0x1F << 0;
    }
}

impl PLLCFGR {
    pub fn PLL_SRC_DIV(n: u32) -> Self {
        Self::from_bits_retain(n << 28) & Self::PLL_SRC_DIV_MASK
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTR: u32 {
        const CSSC = 1 << 23;
        const SERDESPLLRDYC = 1 << 22;
        const ETHPLLRDYC = 1 << 21;
        const PLLRDYC = 1 << 20;
        const HSERDYC = 1 << 19;
        const HSIRDYC = 1 << 18;
        const LSERDYC = 1 << 17;
        const LSIRDYC = 1 << 16;
        const SERDESPLLRDYIE = 1 << 14;
        const ETHPLLRDYIE = 1 << 13;
        const PLLRDYIE = 1 << 12;
        const HSERDYIE = 1 << 11;
        const HSIRDYIE = 1 << 10;
        const LSERDYIE = 1 << 9;
        const LSIRDYIE = 1 << 8;
        const CSSF = 1 << 7;
        const SERDESPLLRDYF = 1 << 6;
        const ETHPLLRDYF = 1 << 5;
        const PLLRDYF = 1 << 4;
        const HSERDYF = 1 << 3;
        const HSIRDYF = 1 << 2;
        const LSERDYF = 1 << 1;
        const LSIRDYF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HB2PRSTR: u32 {
        const I3CRST = 1 << 31;
        const LTDCRST = 1 << 30;
        const GPHARST = 1 << 27;
        const ECDCRST = 1 << 26;
        const DFSDMRST = 1 << 25;
        const OPCMRST = 1 << 23;
        const TIM12RST = 1 << 22;
        const TIM11RST = 1 << 21;
        const TIM10RST = 1 << 20;
        const TIM9RST = 1 << 19;
        const SDIORST = 1 << 18;
        const SAIRST = 1 << 16;
        const I2C4RST = 1 << 15;
        const USART1RST = 1 << 14;
        const TIM8RST = 1 << 13;
        const SPI1RST = 1 << 12;
        const TIM1RST = 1 << 11;
        const ADC2RST = 1 << 10;
        const ADC1RST = 1 << 9;
        const IOPFRST = 1 << 7;
        const IOPERST = 1 << 6;
        const IOPDRST = 1 << 5;
        const IOPCRST = 1 << 4;
        const IOPBRST = 1 << 3;
        const IOPARST = 1 << 2;
        const HSADCRST = 1 << 1;
        const AFIORST = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HB1PRSTR: u32 {
        const SWPMIRST = 1 << 31;
        const I2C3RST = 1 << 30;
        const DACRST = 1 << 29;
        const PWRRST = 1 << 28;
        const BKPRST = 1 << 27;
        const CAN2RST = 1 << 26;
        const CAN1RST = 1 << 25;
        const CAN3RST = 1 << 24;
        const I2C2RST = 1 << 22;
        const I2C1RST = 1 << 21;
        const USART5RST = 1 << 20;
        const USART4RST = 1 << 19;
        const USART3RST = 1 << 18;
        const USART2RST = 1 << 17;
        const SPI4RST = 1 << 16;
        const SPI3RST = 1 << 15;
        const SPI2RST = 1 << 14;
        const QSPI2RST = 1 << 13;
        const QSPI1RST = 1 << 12;
        const WWDGRST = 1 << 11;
        const LPTIM2RST = 1 << 10;
        const LPTIM1RST = 1 << 9;
        const USART8RST = 1 << 8;
        const USART7RST = 1 << 7;
        const USART6RST = 1 << 6;
        const TIM7RST = 1 << 5;
        const TIM6RST = 1 << 4;
        const TIM5RST = 1 << 3;
        const TIM4RST = 1 << 2;
        const TIM3RST = 1 << 1;
        const TIM2RST = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HBPCENR: u32 {
        const PIOCEN = 1 << 22;
        const SERDESEN = 1 << 20;
        const USBPDEN = 1 << 19;
        const UHSIFEN = 1 << 18;
        const USBOTGEN = 1 << 17;
        const ETHEN = 1 << 14;
        const DVPEN = 1 << 13;
        const USBSSEN = 1 << 12;
        const USBHSEN = 1 << 11;
        const SDMMCEN = 1 << 10;
        const RNGEN = 1 << 9;
        const FMCEN = 1 << 8;
        const CRCEN = 1 << 6;
        const DMA2EN = 1 << 1;
        const DMA1EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HB2PCENR: u32 {
        const I3CEN = 1 << 31;
        const LTDCEN = 1 << 30;
        const GPHAEN = 1 << 27;
        const ECDCEN = 1 << 26;
        const DFSDMEN = 1 << 25;
        const OPCMEN = 1 << 23;
        const TIM12EN = 1 << 22;
        const TIM11EN = 1 << 21;
        const TIM10EN = 1 << 20;
        const TIM9EN = 1 << 19;
        const SDIOEN = 1 << 18;
        const SAIEN = 1 << 16;
        const I2C4EN = 1 << 15;
        const USART1EN = 1 << 14;
        const TIM8EN = 1 << 13;
        const SPI1EN = 1 << 12;
        const TIM1EN = 1 << 11;
        const ADC2EN = 1 << 10;
        const ADC1EN = 1 << 9;
        const IOPFEN = 1 << 7;
        const IOPEEN = 1 << 6;
        const IOPDEN = 1 << 5;
        const IOPCEN = 1 << 4;
        const IOPBEN = 1 << 3;
        const IOPAEN = 1 << 2;
        const HSADCEN = 1 << 1;
        const AFIOEN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HB1PCENR: u32 {
        const SWPMIEN = 1 << 31;
        const I2C3EN = 1 << 30;
        const DACEN = 1 << 29;
        const PWREN = 1 << 28;
        const BKPEN = 1 << 27;
        const CAN2EN = 1 << 26;
        const CAN1EN = 1 << 25;
        const CAN3EN = 1 << 24;
        const I2C2EN = 1 << 22;
        const I2C1EN = 1 << 21;
        const USART5EN = 1 << 20;
        const USART4EN = 1 << 19;
        const USART3EN = 1 << 18;
        const USART2EN = 1 << 17;
        const SPI4EN = 1 << 16;
        const SPI3EN = 1 << 15;
        const SPI2EN = 1 << 14;
        const QSPI2EN = 1 << 13;
        const QSPI1EN = 1 << 12;
        const WWDGEN = 1 << 11;
        const LPTIM2EN = 1 << 10;
        const LPTIM1EN = 1 << 9;
        const USART8EN = 1 << 8;
        const USART7EN = 1 << 7;
        const USART6EN = 1 << 6;
        const TIM7EN = 1 << 5;
        const TIM6EN = 1 << 4;
        const TIM5EN = 1 << 3;
        const TIM4EN = 1 << 2;
        const TIM3EN = 1 << 1;
        const TIM2EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BDCTLR: u32 {
        const BDRST = 1 << 16;

        const RTCCAL_MASK = 0x7F << 9;

        const RTCEN = 1 << 8;

        const RTCSEL_HSE = 0b11 << 6;
        const RTCSEL_LSI = 0b10 << 6;
        const RTCSEL_LSE = 0b01 << 6;
        const RTCSEL_NOCLOCK = 0b00 << 6;
        const RTCSEL_MASK = 0x3 << 6;

        const ASOS = 1 << 5;
        const ASOE = 1 << 4;
        const CCO = 1 << 3;
        const LSEBYP = 1 << 2;
        const LSERDY = 1 << 1;
        const LSEON = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSTSCKR: u32 {
        const LOCKUPRSTF = 1 << 31;
        const WWDGRSTF = 1 << 30;
        const IWDGRSTF = 1 << 29;
        const SFTRSTF = 1 << 28;
        const PORRSTF = 1 << 27;
        const PINRSTF = 1 << 26;
        const RMVF = 1 << 24;
        const LSIRDY = 1 << 1;
        const LSION = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HBRSTR: u32 {
        const PIOCRST = 1 << 22;
        const SERDESRST = 1 << 20;
        const USBPDRST = 1 << 19;
        const UHSIFRST = 1 << 18;
        const USBOTGRST = 1 << 17;
        const ETHRST = 1 << 14;
        const DVPRST = 1 << 13;
        const USBSSRST = 1 << 12;
        const USBHSRST = 1 << 11;
        const SDMMCRST = 1 << 10;
        const RNGRST = 1 << 9;
        const FMCRST = 1 << 8;
        const DMA2RST = 1 << 1;
        const DMA1RST = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR2: u32 {
        const ETH1GSRC_SERDESPLL_DIV8 = 0b11 << 30;
        const ETH1GSRC_ETHPLL_DIV4 = 0b10 << 30;
        const ETH1GSRC_USBSSPLL = 0b01 << 30;
        const ETH1GSRC_PLLCLK = 0b00 << 30;
        const ETH1GSRC_MASK = 0x3 << 30;

        const HSADCSRC_ETHPLL = 0b11 << 28;
        const HSADCSRC_USBHSPLL = 0b10 << 28;
        const HSADCSRC_PLLCLK = 0b01 << 28;
        const HSADCSRC_SYSCLK = 0b00 << 28;
        const HSADCSRC_MASK = 0x3 << 28;

        const I2S3SRC = 1 << 25;
        const I2S3SRC_PLLCLK = 1 << 25;
        const I2S3SRC_SYSCLK = 0b0 << 25;

        const I2S2SRC = 1 << 24;
        const I2S2SRC_PLLCLK = 1 << 24;
        const I2S2SRC_SYSCLK = 0b0 << 24;

        const RNGSRC = 1 << 23;
        const RNGSRC_PLLCLK = 1 << 23;
        const RNGSRC_SYSCLK = 0b0 << 23;

        const USBFSSRC = 1 << 20;
        const USBFSSRC_USBHSPLL = 1 << 20;
        const USBFSSRC_PLLCLK = 0b0 << 20;

        const USBFSDIV_DIV9_5 = 0b1111 << 16;
        const USBFSDIV_DIV7_5 = 0b1110 << 16;
        const USBFSDIV_DIV6_5 = 0b1101 << 16;
        const USBFSDIV_DIV5_5 = 0b1100 << 16;
        const USBFSDIV_DIV4_5 = 0b1011 << 16;
        const USBFSDIV_DIV3_5 = 0b1010 << 16;
        const USBFSDIV_DIV2_5 = 0b1001 << 16;
        const USBFSDIV_DIV1_5 = 0b1000 << 16;
        const USBFSDIV_DIV10 = 0b0111 << 16;
        const USBFSDIV_DIV8 = 0b0110 << 16;
        const USBFSDIV_DIV6 = 0b0101 << 16;
        const USBFSDIV_DIV5 = 0b0100 << 16;
        const USBFSDIV_DIV4 = 0b0011 << 16;
        const USBFSDIV_DIV3 = 0b0010 << 16;
        const USBFSDIV_DIV2 = 0b0001 << 16;
        const USBFSDIV_DIV1 = 0b0000 << 16;
        const USBFSDIV_MASK = 0xF << 16;

        const LTDCSRC_USBHSPLL = 0b11 << 14;
        const LTDCSRC_ETHPLL = 0b10 << 14;
        const LTDCSRC_SERDESPLL = 0b01 << 14;
        const LTDCSRC_PLLCLK = 0b00 << 14;
        const LTDCSRC_MASK = 0x3 << 14;

        const LTDCDIV_DIV64 = 0b111111 << 8;
        const LTDCDIV_DIV63 = 0b111110 << 8;
        const LTDCDIV_DIV62 = 0b111101 << 8;
        const LTDCDIV_DIV61 = 0b111100 << 8;
        const LTDCDIV_DIV60 = 0b111011 << 8;
        const LTDCDIV_DIV59 = 0b111010 << 8;
        const LTDCDIV_DIV58 = 0b111001 << 8;
        const LTDCDIV_DIV57 = 0b111000 << 8;
        const LTDCDIV_DIV56 = 0b110111 << 8;
        const LTDCDIV_DIV55 = 0b110110 << 8;
        const LTDCDIV_DIV54 = 0b110101 << 8;
        const LTDCDIV_DIV53 = 0b110100 << 8;
        const LTDCDIV_DIV52 = 0b110011 << 8;
        const LTDCDIV_DIV51 = 0b110010 << 8;
        const LTDCDIV_DIV50 = 0b110001 << 8;
        const LTDCDIV_DIV49 = 0b110000 << 8;
        const LTDCDIV_DIV48 = 0b101111 << 8;
        const LTDCDIV_DIV47 = 0b101110 << 8;
        const LTDCDIV_DIV46 = 0b101101 << 8;
        const LTDCDIV_DIV45 = 0b101100 << 8;
        const LTDCDIV_DIV44 = 0b101011 << 8;
        const LTDCDIV_DIV43 = 0b101010 << 8;
        const LTDCDIV_DIV42 = 0b101001 << 8;
        const LTDCDIV_DIV41 = 0b101000 << 8;
        const LTDCDIV_DIV40 = 0b100111 << 8;
        const LTDCDIV_DIV39 = 0b100110 << 8;
        const LTDCDIV_DIV38 = 0b100101 << 8;
        const LTDCDIV_DIV37 = 0b100100 << 8;
        const LTDCDIV_DIV36 = 0b100011 << 8;
        const LTDCDIV_DIV35 = 0b100010 << 8;
        const LTDCDIV_DIV34 = 0b100001 << 8;
        const LTDCDIV_DIV33 = 0b100000 << 8;
        const LTDCDIV_DIV32 = 0b011111 << 8;
        const LTDCDIV_DIV31 = 0b011110 << 8;
        const LTDCDIV_DIV30 = 0b011101 << 8;
        const LTDCDIV_DIV29 = 0b011100 << 8;
        const LTDCDIV_DIV28 = 0b011011 << 8;
        const LTDCDIV_DIV27 = 0b011010 << 8;
        const LTDCDIV_DIV26 = 0b011001 << 8;
        const LTDCDIV_DIV25 = 0b011000 << 8;
        const LTDCDIV_DIV24 = 0b010111 << 8;
        const LTDCDIV_DIV23 = 0b010110 << 8;
        const LTDCDIV_DIV22 = 0b010101 << 8;
        const LTDCDIV_DIV21 = 0b010100 << 8;
        const LTDCDIV_DIV20 = 0b010011 << 8;
        const LTDCDIV_DIV19 = 0b010010 << 8;
        const LTDCDIV_DIV18 = 0b010001 << 8;
        const LTDCDIV_DIV17 = 0b010000 << 8;
        const LTDCDIV_DIV16 = 0b001111 << 8;
        const LTDCDIV_DIV15 = 0b001110 << 8;
        const LTDCDIV_DIV14 = 0b001101 << 8;
        const LTDCDIV_DIV13 = 0b001100 << 8;
        const LTDCDIV_DIV12 = 0b001011 << 8;
        const LTDCDIV_DIV11 = 0b001010 << 8;
        const LTDCDIV_DIV10 = 0b001001 << 8;
        const LTDCDIV_DIV9 = 0b001000 << 8;
        const LTDCDIV_DIV8 = 0b000111 << 8;
        const LTDCDIV_DIV7 = 0b000110 << 8;
        const LTDCDIV_DIV6 = 0b000101 << 8;
        const LTDCDIV_DIV5 = 0b000100 << 8;
        const LTDCDIV_DIV4 = 0b000011 << 8;
        const LTDCDIV_DIV3 = 0b000010 << 8;
        const LTDCDIV_DIV2 = 0b000001 << 8;
        const LTDCDIV_DIV1 = 0b000000 << 8;
        const LTDCDIV_MASK = 0x3F << 8;

        const UHSIFSRC_ETHPLL = 0b11 << 6;
        const UHSIFSRC_USBHSPLL = 0b10 << 6;
        const UHSIFSRC_PLLCLK = 0b01 << 6;
        const UHSIFSRC_SYSCLK = 0b00 << 6;
        const UHSIFSRC_MASK = 0x3 << 6;

        const UHSIFDIV_DIV64 = 0b111111 << 0;
        const UHSIFDIV_DIV63 = 0b111110 << 0;
        const UHSIFDIV_DIV62 = 0b111101 << 0;
        const UHSIFDIV_DIV61 = 0b111100 << 0;
        const UHSIFDIV_DIV60 = 0b111011 << 0;
        const UHSIFDIV_DIV59 = 0b111010 << 0;
        const UHSIFDIV_DIV58 = 0b111001 << 0;
        const UHSIFDIV_DIV57 = 0b111000 << 0;
        const UHSIFDIV_DIV56 = 0b110111 << 0;
        const UHSIFDIV_DIV55 = 0b110110 << 0;
        const UHSIFDIV_DIV54 = 0b110101 << 0;
        const UHSIFDIV_DIV53 = 0b110100 << 0;
        const UHSIFDIV_DIV52 = 0b110011 << 0;
        const UHSIFDIV_DIV51 = 0b110010 << 0;
        const UHSIFDIV_DIV50 = 0b110001 << 0;
        const UHSIFDIV_DIV49 = 0b110000 << 0;
        const UHSIFDIV_DIV48 = 0b101111 << 0;
        const UHSIFDIV_DIV47 = 0b101110 << 0;
        const UHSIFDIV_DIV46 = 0b101101 << 0;
        const UHSIFDIV_DIV45 = 0b101100 << 0;
        const UHSIFDIV_DIV44 = 0b101011 << 0;
        const UHSIFDIV_DIV43 = 0b101010 << 0;
        const UHSIFDIV_DIV42 = 0b101001 << 0;
        const UHSIFDIV_DIV41 = 0b101000 << 0;
        const UHSIFDIV_DIV40 = 0b100111 << 0;
        const UHSIFDIV_DIV39 = 0b100110 << 0;
        const UHSIFDIV_DIV38 = 0b100101 << 0;
        const UHSIFDIV_DIV37 = 0b100100 << 0;
        const UHSIFDIV_DIV36 = 0b100011 << 0;
        const UHSIFDIV_DIV35 = 0b100010 << 0;
        const UHSIFDIV_DIV34 = 0b100001 << 0;
        const UHSIFDIV_DIV33 = 0b100000 << 0;
        const UHSIFDIV_DIV32 = 0b011111 << 0;
        const UHSIFDIV_DIV31 = 0b011110 << 0;
        const UHSIFDIV_DIV30 = 0b011101 << 0;
        const UHSIFDIV_DIV29 = 0b011100 << 0;
        const UHSIFDIV_DIV28 = 0b011011 << 0;
        const UHSIFDIV_DIV27 = 0b011010 << 0;
        const UHSIFDIV_DIV26 = 0b011001 << 0;
        const UHSIFDIV_DIV25 = 0b011000 << 0;
        const UHSIFDIV_DIV24 = 0b010111 << 0;
        const UHSIFDIV_DIV23 = 0b010110 << 0;
        const UHSIFDIV_DIV22 = 0b010101 << 0;
        const UHSIFDIV_DIV21 = 0b010100 << 0;
        const UHSIFDIV_DIV20 = 0b010011 << 0;
        const UHSIFDIV_DIV19 = 0b010010 << 0;
        const UHSIFDIV_DIV18 = 0b010001 << 0;
        const UHSIFDIV_DIV17 = 0b010000 << 0;
        const UHSIFDIV_DIV16 = 0b001111 << 0;
        const UHSIFDIV_DIV15 = 0b001110 << 0;
        const UHSIFDIV_DIV14 = 0b001101 << 0;
        const UHSIFDIV_DIV13 = 0b001100 << 0;
        const UHSIFDIV_DIV12 = 0b001011 << 0;
        const UHSIFDIV_DIV11 = 0b001010 << 0;
        const UHSIFDIV_DIV10 = 0b001001 << 0;
        const UHSIFDIV_DIV9 = 0b001000 << 0;
        const UHSIFDIV_DIV8 = 0b000111 << 0;
        const UHSIFDIV_DIV7 = 0b000110 << 0;
        const UHSIFDIV_DIV6 = 0b000101 << 0;
        const UHSIFDIV_DIV5 = 0b000100 << 0;
        const UHSIFDIV_DIV4 = 0b000011 << 0;
        const UHSIFDIV_DIV3 = 0b000010 << 0;
        const UHSIFDIV_DIV2 = 0b000001 << 0;
        const UHSIFDIV_DIV1 = 0b000000 << 0;
        const UHSIFDIV_MASK = 0x3F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PLLCFGR2: u32 {
        const SERDESPLL_MUL90 = 0b1111 << 16;
        const SERDESPLL_MUL80 = 0b1110 << 16;
        const SERDESPLL_MUL76 = 0b1101 << 16;
        const SERDESPLL_MUL70 = 0b1100 << 16;
        const SERDESPLL_MUL64 = 0b1011 << 16;
        const SERDESPLL_MUL60 = 0b1010 << 16;
        const SERDESPLL_MUL56 = 0b1001 << 16;
        const SERDESPLL_MUL50 = 0b1000 << 16;
        const SERDESPLL_MUL45 = 0b0111 << 16;
        const SERDESPLL_MUL40 = 0b0110 << 16;
        const SERDESPLL_MUL38 = 0b0101 << 16;
        const SERDESPLL_MUL35 = 0b0100 << 16;
        const SERDESPLL_MUL32 = 0b0011 << 16;
        const SERDESPLL_MUL30 = 0b0010 << 16;
        const SERDESPLL_MUL28 = 0b0001 << 16;
        const SERDESPLL_MUL25 = 0b0000 << 16;
        const SERDESPLL_MUL_MASK = 0xF << 16;

        const USBHSPLL_IN_DIV32 = 0b11111 << 8;
        const USBHSPLL_IN_DIV31 = 0b11110 << 8;
        const USBHSPLL_IN_DIV30 = 0b11101 << 8;
        const USBHSPLL_IN_DIV29 = 0b11100 << 8;
        const USBHSPLL_IN_DIV28 = 0b11011 << 8;
        const USBHSPLL_IN_DIV27 = 0b11010 << 8;
        const USBHSPLL_IN_DIV26 = 0b11001 << 8;
        const USBHSPLL_IN_DIV25 = 0b11000 << 8;
        const USBHSPLL_IN_DIV24 = 0b10111 << 8;
        const USBHSPLL_IN_DIV23 = 0b10110 << 8;
        const USBHSPLL_IN_DIV22 = 0b10101 << 8;
        const USBHSPLL_IN_DIV21 = 0b10100 << 8;
        const USBHSPLL_IN_DIV20 = 0b10011 << 8;
        const USBHSPLL_IN_DIV19 = 0b10010 << 8;
        const USBHSPLL_IN_DIV18 = 0b10001 << 8;
        const USBHSPLL_IN_DIV17 = 0b10000 << 8;
        const USBHSPLL_IN_DIV16 = 0b01111 << 8;
        const USBHSPLL_IN_DIV15 = 0b01110 << 8;
        const USBHSPLL_IN_DIV14 = 0b01101 << 8;
        const USBHSPLL_IN_DIV13 = 0b01100 << 8;
        const USBHSPLL_IN_DIV12 = 0b01011 << 8;
        const USBHSPLL_IN_DIV11 = 0b01010 << 8;
        const USBHSPLL_IN_DIV10 = 0b01001 << 8;
        const USBHSPLL_IN_DIV9 = 0b01000 << 8;
        const USBHSPLL_IN_DIV8 = 0b00111 << 8;
        const USBHSPLL_IN_DIV7 = 0b00110 << 8;
        const USBHSPLL_IN_DIV6 = 0b00101 << 8;
        const USBHSPLL_IN_DIV5 = 0b00100 << 8;
        const USBHSPLL_IN_DIV4 = 0b00011 << 8;
        const USBHSPLL_IN_DIV3 = 0b00010 << 8;
        const USBHSPLL_IN_DIV2 = 0b00001 << 8;
        const USBHSPLL_IN_DIV1 = 0b00000 << 8;
        const USBHSPLL_IN_DIV_MASK = 0x1F << 8;

        const USBSSPLL_REFSEL_80MHZ = 0b111 << 4;
        const USBSSPLL_REFSEL_60MHZ = 0b110 << 4;
        const USBSSPLL_REFSEL_40MHZ = 0b101 << 4;
        const USBSSPLL_REFSEL_32MHZ = 0b100 << 4;
        const USBSSPLL_REFSEL_30MHZ = 0b011 << 4;
        const USBSSPLL_REFSEL_25MHZ = 0b010 << 4;
        const USBSSPLL_REFSEL_24MHZ = 0b001 << 4;
        const USBSSPLL_REFSEL_20MHZ = 0b000 << 4;
        const USBSSPLL_REFSEL_MASK = 0x7 << 4;

        const USBHSPLL_REFSEL_32MHZ = 0b11 << 2;
        const USBHSPLL_REFSEL_24MHZ = 0b10 << 2;
        const USBHSPLL_REFSEL_20MHZ = 0b01 << 2;
        const USBHSPLL_REFSEL_25MHZ = 0b00 << 2;
        const USBHSPLL_REFSEL_MASK = 0x3 << 2;

        const USBHSPLLSRC_PLLCLK = 0b11 << 0;
        const USBHSPLLSRC_ETHCLK_20M = 0b10 << 0;
        const USBHSPLLSRC_HSI = 0b01 << 0;
        const USBHSPLLSRC_HSE = 0b00 << 0;
        const USBHSPLLSRC_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT: u8 {
        const CSS = 1 << 7;
        const SERDESPLLRDY = 1 << 6;
        const ETHPLLRDY = 1 << 5;
        const PLLRDY = 1 << 4;
        const HSERDY = 1 << 3;
        const HSIRDY = 1 << 2;
        const LSERDY = 1 << 1;
        const LSIRDY = 1 << 0;
    }
}

impl RCC {
    pub fn HSICAL(&self) -> &Reg8 {
        unsafe { Reg::from_ptr((self.CTLR.as_ptr() as *mut u8).add(1)) }
    }

    pub fn INTF(&self) -> &Reg<INT> {
        unsafe { Reg::from_ptr(self.INTR.as_ptr() as *mut INT) }
    }

    pub fn INTE(&self) -> &Reg<INT> {
        unsafe { Reg::from_ptr((self.INTR.as_ptr() as *mut INT).add(1)) }
    }

    pub fn INTC(&self) -> &Reg<INT> {
        unsafe { Reg::from_ptr((self.INTR.as_ptr() as *mut INT).add(2)) }
    }
}

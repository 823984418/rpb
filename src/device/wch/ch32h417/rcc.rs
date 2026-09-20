use bitflags::bitflags;

use crate::register::{Reg, Reg32};

#[derive(Debug)]
#[repr(C)]
pub struct RCC {
    pub CTLR: Reg<CTLR>,
    pub CFGR0: Reg<CFGR0>,
    pub PLLCFGR: Reg<PLLCFGR>,
    pub INTR: Reg<INTR>,
    pub HB2PRSTR: Reg<HB2PRSTR>,
    pub HB1PRSTR: Reg<HB1PRSTR>,
    pub HBPCENR: Reg32,
    pub HB2PCENR: Reg<HB2PCENR>,
    pub HB1PCENR: Reg32,
    pub BDCTLR: Reg32,
    pub RSTSCKR: Reg32,
    pub HBRSTR: Reg32,
    pub CFGR2: Reg32,
    pub PLLCFGR2: Reg32,
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct CTLR: u32 {
        const CSS_HSE_DIS = 1 << 31;
        const SERDES_PLLRDY = 1 << 29;
        const SERDES_PLLON = 1 << 28;
        const ETH_PLLRDY = 1 << 27;
        const ETH_PLLON = 1 << 27;
        const PLLRDY = 1 << 26;
        const PLLON = 1 << 25;
        const USBSS_PLLRDY = 1 << 24;
        const USBSS_PLLON = 1 << 23;
        const USBHS_PLLRDY = 1 << 22;
        const USBHS_PLLON = 1 << 21;
        const CSSON = 1 << 20;
        const HSEBYP = 1 << 19;
        const HSERDY = 1 << 18;
        const HSEON = 1 << 17;
        const HSICAL_MASK = 0xFF << 8;
        const HSITRIM_MASK = 0x1F << 3;
        const HSIRDY = 1 << 1;
        const HSION = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct CFGR0: u32 {
        const ADCSRC = 1 << 31;
        const ADC_DUTY_SEL = 1 << 30;
        const MCO_MASK = 0xF << 24;
        const UTMION = 1 << 23;
        const PIPEON = 1 << 22;
        const RGMIION = 1 << 21;
        const FPRE_MASK = 0x3 << 16;
        const ADCPRE_MASK = 0x3 << 14;
        const PPRE2_MASK = 0x7 << 11;
        const HPRE_MASK = 0xF << 4;

        const SWS_MASK = 0x3 << 2;
        const SWS_PLL = 0b10 << 2;
        const SWS_HSE = 0b01 << 2;
        const SWS_HSI = 0b00 << 2;

        const SW_MASK = 0x3 << 0;
        const SW_PLL = 0b10 << 0;
        const SW_HSE = 0b01 << 0;
        const SW_HSI = 0b00 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
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
    #[derive(Debug, Copy, Clone)]
    pub struct PLLCFGR: u32 {
        const SYSPLL_GATE = 1 << 31;

        const SYSPLL_SEL_MASK = 0x7 << 28;
        const SYSPLL_SEL_USBSS_PLL = 0b111 << 28;
        const SYSPLL_SEL_SERDES_PLL_DIV_2 = 0b110 << 28;
        const SYSPLL_SEL_ETH_PLL = 0b101 << 28;
        const SYSPLL_SEL_USBHS_PLL = 0b100 << 28;
        const SYSPLL_SEL_PLL_CLK = 0b000 << 28;

        const PLL_SRC_DIV_MASK = 0x3F << 8;

        const PLLSRC_MASK = 0x7 << 5;
        const PLLSRC_SERDES_PLL_DIV_2 = 0b111 << 5;
        const PLLSRC_USBSS_PLL = 0b110 << 5;
        const PLLSRC_ETH_PLL = 0b101 << 5;
        const PLLSRC_USBHS_PLL = 0b100 << 5;
        const PLLSRC_HSE = 0b001 << 5;
        const PLLSRC_HSI = 0b000 << 5;

        const PLLMUL_MASK = 0x1F << 0;
        const PLLMUL_59 = 0b11111 << 0;
        const PLLMUL_40 = 0b11110 << 0;
        const PLLMUL_38 = 0b11101 << 0;
        const PLLMUL_36 = 0b11100 << 0;
        const PLLMUL_34 = 0b11011 << 0;
        const PLLMUL_32 = 0b11010 << 0;
        const PLLMUL_30 = 0b11001 << 0;
        const PLLMUL_28 = 0b11000 << 0;
        const PLLMUL_26 = 0b10111 << 0;
        const PLLMUL_24 = 0b10110 << 0;
        const PLLMUL_22 = 0b10101 << 0;
        const PLLMUL_20 = 0b10100 << 0;
        const PLLMUL_19 = 0b10011 << 0;
        const PLLMUL_18 = 0b10010 << 0;
        const PLLMUL_17 = 0b10001 << 0;
        const PLLMUL_16 = 0b10000 << 0;
        const PLLMUL_15 = 0b01111 << 0;
        const PLLMUL_14 = 0b01110 << 0;
        const PLLMUL_13 = 0b01101 << 0;
        const PLLMUL_12_5 = 0b01100 << 0;
        const PLLMUL_12 = 0b01011 << 0;
        const PLLMUL_11_5 = 0b01010 << 0;
        const PLLMUL_11 = 0b01001 << 0;
        const PLLMUL_10_5 = 0b01000 << 0;
        const PLLMUL_10 = 0b00111 << 0;
        const PLLMUL_9_5 = 0b00110 << 0;
        const PLLMUL_9 = 0b00101 << 0;
        const PLLMUL_8_5 = 0b00100 << 0;
        const PLLMUL_8 = 0b00011 << 0;
        const PLLMUL_7 = 0b00010 << 0;
        const PLLMUL_6 = 0b00001 << 0;
        const PLLMUL_4 = 0b00000 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
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
    #[derive(Debug, Copy, Clone)]
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
    #[derive(Debug, Copy, Clone)]
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

impl RCC {
    pub fn DeInit(&self) {
        self.CTLR.bit_or(CTLR::HSION);
        self.CFGR0.bit_clear(
            CFGR0::ADCSRC
                | CFGR0::ADC_DUTY_SEL
                | CFGR0::MCO_MASK
                | CFGR0::UTMION
                | CFGR0::RGMIION
                | CFGR0::FPRE_MASK
                | CFGR0::ADCPRE_MASK
                | CFGR0::PPRE2_MASK
                | CFGR0::from_bits_retain(0x7 << 8)
                | CFGR0::HPRE_MASK
                | CFGR0::SWS_MASK
                | CFGR0::SW_MASK,
        );
        // TODO:
    }
}

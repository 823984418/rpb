use bitflags::bitflags;

use crate::register::Reg;

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

        const MCO_MASK = 0xF << 24;

        const UTMION = 1 << 23;
        const PIPEON = 1 << 22;
        const RGMIION = 1 << 21;

        const FPRE_4 = 0b10 << 16;
        const FPRE_2 = 0b01 << 16;
        const FPRE_1 = 0b00 << 16;
        const FPRE_MASK = 0x3 << 16;

        const ADCPRE_MASK = 0x3 << 14;

        const PPRE2_MASK = 0x7 << 11;

        const HPRE_512 = 0b1111 << 4;
        const HPRE_256 = 0b1110 << 4;
        const HPRE_128 = 0b1101 << 4;
        const HPRE_64 = 0b1100 << 4;
        const HPRE_16 = 0b1011 << 4;
        const HPRE_8 = 0b1010 << 4;
        const HPRE_4 = 0b1001 << 4;
        const HPRE_2 = 0b1000 << 4;
        const HPRE_1 = 0b0000 << 4;
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

        const SYSPLL_SEL_USBSS_PLL = 0b111 << 28;
        const SYSPLL_SEL_SERDES_PLL_DIV_2 = 0b110 << 28;
        const SYSPLL_SEL_ETH_PLL = 0b101 << 28;
        const SYSPLL_SEL_USBHS_PLL = 0b100 << 28;
        const SYSPLL_SEL_PLL_CLK = 0b000 << 28;
        const SYSPLL_SEL_MASK = 0x7 << 28;

        const PLL_SRC_DIV_MASK = 0x3F << 8;

        const PLLSRC_SERDES_PLL_DIV_2 = 0b111 << 5;
        const PLLSRC_USBSS_PLL = 0b110 << 5;
        const PLLSRC_ETH_PLL = 0b101 << 5;
        const PLLSRC_USBHS_PLL = 0b100 << 5;
        const PLLSRC_HSE = 0b001 << 5;
        const PLLSRC_HSI = 0b000 << 5;
        const PLLSRC_MASK = 0x7 << 5;

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
        const OTGFSEN = 1 << 17;
        const ETHMACEN = 1 << 14;
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
        const LKUPRSTF = 1 << 31;
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
        const OTGFSRST = 1 << 17;
        const ETHMACRST = 1 << 14;
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
        const ETH1GSRC_MASK = 0x3 << 30;
        const HSADCSRC_MASK = 0x3 << 28;
        const I2S3SRC = 1 << 25;
        const I2S2SRC = 1 << 24;
        const RNGSRC = 1 << 23;
        const USBFSSRC = 1 << 20;
        const USBFSDIV_MASK = 0xF << 16;
        const LTDCSRC_MASK = 0x3 << 14;
        const LTDCDIV_MASK = 0x3F << 8;
        const UHSIFSRC_MASK = 0x3 << 6;
        const UHSIFDIV_MASK = 0x3F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PLLCFGR2: u32 {
        const SERDESPLL_MUL_90 = 0b1111 << 16;
        const SERDESPLL_MUL_80 = 0b1110 << 16;
        const SERDESPLL_MUL_76 = 0b1101 << 16;
        const SERDESPLL_MUL_70 = 0b1100 << 16;
        const SERDESPLL_MUL_64 = 0b1011 << 16;
        const SERDESPLL_MUL_60 = 0b1010 << 16;
        const SERDESPLL_MUL_56 = 0b1001 << 16;
        const SERDESPLL_MUL_50 = 0b1000 << 16;
        const SERDESPLL_MUL_55 = 0b0111 << 16;
        const SERDESPLL_MUL_40 = 0b0110 << 16;
        const SERDESPLL_MUL_38 = 0b0101 << 16;
        const SERDESPLL_MUL_35 = 0b0100 << 16;
        const SERDESPLL_MUL_32 = 0b0011 << 16;
        const SERDESPLL_MUL_30 = 0b0010 << 16;
        const SERDESPLL_MUL_28 = 0b0001 << 16;
        const SERDESPLL_MUL_25 = 0b0000 << 16;
        const SERDESPLL_MUL_MASK = 0xF << 16;

        const USBHSPLL_IN_DIV_MASK = 0x1F << 8;

        const USBSSPLL_REFSEL_MASK = 0x3 << 4;

        const USBHSPLL_REFSEL_MASK = 0x3 << 2;

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
    pub fn INTF(&self) -> &Reg<INT> {
        unsafe { Reg::from_addr(self.INTR.as_ptr() as *mut INT) }
    }

    pub fn INTE(&self) -> &Reg<INT> {
        unsafe { Reg::from_addr((self.INTR.as_ptr() as *mut INT).add(1)) }
    }

    pub fn INTC(&self) -> &Reg<INT> {
        unsafe { Reg::from_addr((self.INTR.as_ptr() as *mut INT).add(2)) }
    }
}

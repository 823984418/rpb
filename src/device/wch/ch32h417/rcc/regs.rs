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
        const HSICAL = 0xFF << 8;

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
        const MCO = 0xF << 24;
        const MCO_3 = 1 << 27;
        const MCO_2 = 1 << 26;
        const MCO_1 = 1 << 25;
        const MCO_0 = 1 << 24;
        const MCO_SERDES_DIV16 = 0x0B << 24;
        const MCO_ETH_PLL_DIV8 = 0x0A << 24;
        const MCO_USBSS_PLL_DIV2 = 0x09 << 24;
        const MCO_UTMI = 0x08 << 24;
        const MCO_PLL_DIV2 = 0x07 << 24;
        const MCO_HSE = 0x06 << 24;
        const MCO_HSI = 0x05 << 24;
        const MCO_SYSCLK = 0x04 << 24;
        const MCO_NOCLOCK = 0x00 << 24;

        const UTMION = 1 << 23;
        const PIPEON = 1 << 22;
        const RGMIION = 1 << 21;

        const FPRE_MASK = 0x3 << 16;
        const FPRE = 0x3 << 16;
        const FPRE_4 = 0b10 << 16;
        const FPRE_2 = 0b01 << 16;
        const FPRE_1 = 0b00 << 16;
        const FPRE_0 = 1 << 16;
        const FPRE_DIV4 = 0b10 << 16;
        const FPRE_DIV2 = 0b01 << 16;
        const FPRE_DIV1 = 0b00 << 16;

        const ADCPRE_MASK = 0x3 << 14;
        const ADCPRE = 0x3 << 14;
        const ADCPRE_1 = 1 << 15;
        const ADCPRE_0 = 1 << 14;
        const ADCPRE_ADCH_DIV8 = 0x3000;
        const ADCPRE_ADCH_DIV6 = 0x2000;
        const ADCPRE_ADCH_DIV4 = 0x1000;
        const ADCPRE_ADCH_DIV2 = 0x0000;
        const ADCPRE_DIV36 = 0xF800;
        const ADCPRE_DIV35 = 0xF000;
        const ADCPRE_DIV34 = 0xE800;
        const ADCPRE_DIV33 = 0xE000;
        const ADCPRE_DIV32 = 0xD800;
        const ADCPRE_DIV31 = 0xD000;
        const ADCPRE_DIV30 = 0xC800;
        const ADCPRE_DIV29 = 0xC000;
        const ADCPRE_DIV28 = 0xB800;
        const ADCPRE_DIV27 = 0xB000;
        const ADCPRE_DIV26 = 0xA800;
        const ADCPRE_DIV25 = 0xA000;
        const ADCPRE_DIV24 = 0x9800;
        const ADCPRE_DIV23 = 0x9000;
        const ADCPRE_DIV22 = 0x8800;
        const ADCPRE_DIV21 = 0x8000;
        const ADCPRE_DIV20 = 0x7800;
        const ADCPRE_DIV19 = 0x7000;
        const ADCPRE_DIV18 = 0x6800;
        const ADCPRE_DIV17 = 0x6000;
        const ADCPRE_DIV16 = 0x5800;
        const ADCPRE_DIV15 = 0x5000;
        const ADCPRE_DIV14 = 0x4800;
        const ADCPRE_DIV13 = 0x4000;
        const ADCPRE_DIV12 = 0x3800;
        const ADCPRE_DIV11 = 0x3000;
        const ADCPRE_DIV10 = 0x2800;
        const ADCPRE_DIV9 = 0x2000;
        const ADCPRE_DIV8 = 0x1800;
        const ADCPRE_DIV7 = 0x1000;
        const ADCPRE_DIV6 = 0x0800;
        const ADCPRE_DIV5 = 0x0000;

        const PPRE2_MASK = 0x7 << 11;
        const PPRE2 = 0x7 << 11;
        const PPRE2_2 = 1 << 13;
        const PPRE2_1 = 1 << 12;
        const PPRE2_0 = 1 << 11;
        const PPRE2_ADCL_DIV16 = 0b111 << 11;
        const PPRE2_ADCL_DIV8 = 0b110 << 11;
        const PPRE2_ADCL_DIV4 = 0b101 << 11;
        const PPRE2_ADCL_DIV2 = 0b100 << 11;
        const PPRE2_ADCL_DIV1 = 0b000 << 11;

        const HPRE_MASK = 0xF << 4;
        const HPRE = 0xF << 4;
        const HPRE_3 = 1 << 7;
        const HPRE_0 = 1 << 4;
        const HPRE_512 = 0b1111 << 4;
        const HPRE_256 = 0b1110 << 4;
        const HPRE_128 = 0b1101 << 4;
        const HPRE_64 = 0b1100 << 4;
        const HPRE_16 = 0b1011 << 4;
        const HPRE_8 = 0b1010 << 4;
        const HPRE_4 = 0b1001 << 4;
        const HPRE_2 = 0b1000 << 4;
        const HPRE_1 = 0b0000 << 4;
        const HPRE_DIV512 = 0b1111 << 4;
        const HPRE_DIV256 = 0b1110 << 4;
        const HPRE_DIV128 = 0b1101 << 4;
        const HPRE_DIV64 = 0b1100 << 4;
        const HPRE_DIV16 = 0b1011 << 4;
        const HPRE_DIV8 = 0b1010 << 4;
        const HPRE_DIV4 = 0b1001 << 4;
        const HPRE_DIV2 = 0b1000 << 4;
        const HPRE_DIV1 = 0b0000 << 4;

        const SWS_MASK = 0x3 << 2;
        const SWS = 0x3 << 2;
        const SWS_1 = 1 << 3;
        const SWS_0 = 1 << 2;
        const SWS_PLL = 0b10 << 2;
        const SWS_HSE = 0b01 << 2;
        const SWS_HSI = 0b00 << 2;

        const SW_MASK = 0x3 << 0;
        const SW = 0x3 << 0;
        const SW_1 = 1 << 1;
        const SW_0 = 1 << 0;
        const SW_PLL = 0b10 << 0;
        const SW_HSE = 0b01 << 0;
        const SW_HSI = 0b00 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PLLCFGR: u32 {
        const SYSPLL_GATE = 1 << 31;

        const SYSPLL_SEL_MASK = 0x7 << 28;
        const SYSPLL_SEL = 0x7 << 28;
        const SYSPLL_SEL_2 = 1 << 30;
        const SYSPLL_SEL_1 = 1 << 29;
        const SYSPLL_SEL_0 = 1 << 28;
        const SYSPLL_SEL_USBSS_PLL = 0b111 << 28;
        const SYSPLL_SEL_SERDES_PLL_DIV_2 = 0b110 << 28;
        const SYSPLL_SEL_ETH_PLL = 0b101 << 28;
        const SYSPLL_SEL_USBHS_PLL = 0b100 << 28;
        const SYSPLL_SEL_PLL_CLK = 0b000 << 28;
        const SYSPLL_USBSS = 0b111 << 28;
        const SYSPLL_SERDES = 0b110 << 28;
        const SYSPLL_ETH = 0b101 << 28;
        const SYSPLL_USBHS = 0b100 << 28;
        const SYSPLL_PLL = 0b000 << 28;

        const PLL_SRC_DIV_MASK = 0x3F << 8;
        const PLL_SRC_DIV_5 = 1 << 13;
        const PLL_SRC_DIV_4 = 1 << 12;
        const PLL_SRC_DIV_3 = 1 << 11;
        const PLL_SRC_DIV_2 = 1 << 10;
        const PLL_SRC_DIV_1 = 1 << 9;
        const PLL_SRC_DIV_0 = 1 << 8;
        const PLL_SRC_DIV64 = 0x3F << 8;
        const PLL_SRC_DIV63 = 0x3E << 8;
        const PLL_SRC_DIV62 = 0x3D << 8;
        const PLL_SRC_DIV61 = 0x3C << 8;
        const PLL_SRC_DIV60 = 0x3B << 8;
        const PLL_SRC_DIV59 = 0x3A << 8;
        const PLL_SRC_DIV58 = 0x39 << 8;
        const PLL_SRC_DIV57 = 0x38 << 8;
        const PLL_SRC_DIV56 = 0x37 << 8;
        const PLL_SRC_DIV55 = 0x36 << 8;
        const PLL_SRC_DIV54 = 0x35 << 8;
        const PLL_SRC_DIV53 = 0x34 << 8;
        const PLL_SRC_DIV52 = 0x33 << 8;
        const PLL_SRC_DIV51 = 0x32 << 8;
        const PLL_SRC_DIV50 = 0x31 << 8;
        const PLL_SRC_DIV49 = 0x30 << 8;
        const PLL_SRC_DIV48 = 0x2F << 8;
        const PLL_SRC_DIV47 = 0x2E << 8;
        const PLL_SRC_DIV46 = 0x2D << 8;
        const PLL_SRC_DIV45 = 0x2C << 8;
        const PLL_SRC_DIV44 = 0x2B << 8;
        const PLL_SRC_DIV43 = 0x2A << 8;
        const PLL_SRC_DIV42 = 0x29 << 8;
        const PLL_SRC_DIV41 = 0x28 << 8;
        const PLL_SRC_DIV40 = 0x27 << 8;
        const PLL_SRC_DIV39 = 0x26 << 8;
        const PLL_SRC_DIV38 = 0x25 << 8;
        const PLL_SRC_DIV37 = 0x24 << 8;
        const PLL_SRC_DIV36 = 0x23 << 8;
        const PLL_SRC_DIV35 = 0x22 << 8;
        const PLL_SRC_DIV34 = 0x21 << 8;
        const PLL_SRC_DIV33 = 0x20 << 8;
        const PLL_SRC_DIV32 = 0x1F << 8;
        const PLL_SRC_DIV31 = 0x1E << 8;
        const PLL_SRC_DIV30 = 0x1D << 8;
        const PLL_SRC_DIV29 = 0x1C << 8;
        const PLL_SRC_DIV28 = 0x1B << 8;
        const PLL_SRC_DIV27 = 0x1A << 8;
        const PLL_SRC_DIV26 = 0x19 << 8;
        const PLL_SRC_DIV25 = 0x18 << 8;
        const PLL_SRC_DIV24 = 0x17 << 8;
        const PLL_SRC_DIV23 = 0x16 << 8;
        const PLL_SRC_DIV22 = 0x15 << 8;
        const PLL_SRC_DIV21 = 0x14 << 8;
        const PLL_SRC_DIV20 = 0x13 << 8;
        const PLL_SRC_DIV19 = 0x12 << 8;
        const PLL_SRC_DIV18 = 0x11 << 8;
        const PLL_SRC_DIV17 = 0x10 << 8;
        const PLL_SRC_DIV16 = 0x0F << 8;
        const PLL_SRC_DIV15 = 0x0E << 8;
        const PLL_SRC_DIV14 = 0x0D << 8;
        const PLL_SRC_DIV13 = 0x0C << 8;
        const PLL_SRC_DIV12 = 0x0B << 8;
        const PLL_SRC_DIV11 = 0x0A << 8;
        const PLL_SRC_DIV10 = 0x09 << 8;
        const PLL_SRC_DIV9 = 0x08 << 8;
        const PLL_SRC_DIV8 = 0x07 << 8;
        const PLL_SRC_DIV7 = 0x06 << 8;
        const PLL_SRC_DIV6 = 0x05 << 8;
        const PLL_SRC_DIV5 = 0x04 << 8;
        const PLL_SRC_DIV4 = 0x03 << 8;
        const PLL_SRC_DIV3 = 0x02 << 8;
        const PLL_SRC_DIV2 = 0x01 << 8;
        const PLL_SRC_DIV1 = 0x00 << 8;

        const PLLSRC_MASK = 0x7 << 5;
        const PLLSRC = 0x7 << 5;
        const PLLSRC_4 = 1 << 7;
        const PLLSRC_2 = 1 << 6;
        const PLLSRC_0 = 1 << 5;
        const PLLSRC_SERDES_PLL = 0b111 << 5;
        const PLLSRC_SERDES_PLL_DIV_2 = 0b111 << 5;
        const PLLSRC_USBSS_PLL = 0b110 << 5;
        const PLLSRC_ETH_PLL = 0b101 << 5;
        const PLLSRC_USBHS_PLL = 0b100 << 5;
        const PLLSRC_HSE = 0b001 << 5;
        const PLLSRC_HSI = 0b000 << 5;

        const PLLMUL_MASK = 0x1F << 0;
        const PLLMUL = 0x1F << 0;
        const PLLMUL_3 = 1 << 3;
        const PLLMUL_2 = 1 << 2;
        const PLLMUL_1 = 1 << 1;
        const PLLMUL_0 = 1 << 0;

        const PLLMUL59 = 0x1F;
        const PLLMUL40 = 0x1E;
        const PLLMUL38 = 0x1D;
        const PLLMUL36 = 0x1C;
        const PLLMUL34 = 0x1B;
        const PLLMUL32 = 0x1A;
        const PLLMUL30 = 0x19;
        const PLLMUL28 = 0x18;
        const PLLMUL26 = 0x17;
        const PLLMUL24 = 0x16;
        const PLLMUL22 = 0x15;
        const PLLMUL20 = 0x14;
        const PLLMUL19 = 0x13;
        const PLLMUL18 = 0x12;
        const PLLMUL17 = 0x11;
        const PLLMUL16 = 0x10;
        const PLLMUL15 = 0x0F;
        const PLLMUL14 = 0x0E;
        const PLLMUL13 = 0x0D;
        const PLLMUL12_5 = 0x0C;
        const PLLMUL12 = 0x0B;
        const PLLMUL11_5 = 0x0A;
        const PLLMUL11 = 0x09;
        const PLLMUL10_5 = 0x08;
        const PLLMUL10 = 0x07;
        const PLLMUL9_5 = 0x06;
        const PLLMUL9 = 0x05;
        const PLLMUL8_5 = 0x04;
        const PLLMUL8 = 0x03;
        const PLLMUL7 = 0x02;
        const PLLMUL6 = 0x01;
        const PLLMUL4 = 0x00;

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
        const USBOTGEN = 1 << 17;
        const ETHMACEN = 1 << 14;
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
        const RTCCAL = 0x7F << 9;

        const RTCEN = 1 << 8;

        const RTCSEL_MASK = 0x3 << 6;
        const RTCSEL = 0x3 << 6;
        const RTCSEL_1 = 1 << 7;
        const RTCSEL_0 = 1 << 6;
        const RTCSEL_HSE = 0b11 << 6;
        const RTCSEL_LSI = 0b10 << 6;
        const RTCSEL_LSE = 0b01 << 6;
        const RTCSEL_NOCLOCK = 0b00 << 6;

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
        const USBOTGRST = 1 << 17;
        const ETHMACRST = 1 << 14;
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
        const ETH1GSRC_MASK = 0x3 << 30;
        const ETH1GSRC = 0x3 << 30;
        const ETH1GSRC_1 = 1 << 31;
        const ETH1GSRC_0 = 1 << 30;
        const ETH1GSRC_SERDESPLL_DIV8 = 0b11 << 30;
        const ETH1GSRC_ETHPLL_DIV4 = 0b10 << 30;
        const ETH1GSRC_USBSSPLL = 0b01 << 30;
        const ETH1GSRC_PLLCLK = 0b00 << 30;

        const HSADCSRC_MASK = 0x3 << 28;
        const HSADCSRC = 0x3 << 28;
        const HSADCSRC_1 = 1 << 29;
        const HSADCSRC_0 = 1 << 28;
        const HSADCSRC_ETHPLL = 0b11 << 28;
        const HSADCSRC_USBHSPLL = 0b10 << 28;
        const HSADCSRC_PLLCLK = 0b01 << 28;
        const HSADCSRC_SYSCLK = 0b00 << 28;

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

        const USBFSDIV_MASK = 0xF << 16;
        const USBFSDIV = 0xF << 16;
        const USBFSDIV_3 = 1 << 19;
        const USBFSDIV_2 = 1 << 18;
        const USBFSDIV_1 = 1 << 17;
        const USBFSDIV_0 = 1 << 16;
        const USBFSDIV_DIV9_5 = 0x0F << 16;
        const USBFSDIV_DIV7_5 = 0x0E << 16;
        const USBFSDIV_DIV6_5 = 0x0D << 16;
        const USBFSDIV_DIV5_5 = 0x0C << 16;
        const USBFSDIV_DIV4_5 = 0x0B << 16;
        const USBFSDIV_DIV3_5 = 0x0A << 16;
        const USBFSDIV_DIV2_5 = 0x09 << 16;
        const USBFSDIV_DIV1_5 = 0x08 << 16;
        const USBFSDIV_DIV10 = 0x07 << 16;
        const USBFSDIV_DIV8 = 0x06 << 16;
        const USBFSDIV_DIV6 = 0x05 << 16;
        const USBFSDIV_DIV5 = 0x04 << 16;
        const USBFSDIV_DIV4 = 0x03 << 16;
        const USBFSDIV_DIV3 = 0x02 << 16;
        const USBFSDIV_DIV2 = 0x01 << 16;
        const USBFSDIV_DIV1 = 0x00 << 16;

        const LTDCSRC_MASK = 0x3 << 14;
        const LTDCSRC = 0x3 << 14;
        const LTDCSRC_1 = 1 << 15;
        const LTDCSRC_0 = 1 << 14;
        const LTDCSRC_USBHSPLL = 0b11 << 14;
        const LTDCSRC_ETHPLL = 0b10 << 14;
        const LTDCSRC_SERDESPLL = 0b01 << 14;
        const LTDCSRC_PLLCLK = 0b00 << 14;

        const LTDCDIV_MASK = 0x3F << 8;
        const LTDCDIV = 0x3F << 8;
        const LTDCDIV_5 = 1 << 13;
        const LTDCDIV_4 = 1 << 12;
        const LTDCDIV_3 = 1 << 11;
        const LTDCDIV_2 = 1 << 10;
        const LTDCDIV_1 = 1 << 9;
        const LTDCDIV_0 = 1 << 8;
        const LTDCDIV_DIV64 = 0x3F << 8;
        const LTDCDIV_DIV63 = 0x3E << 8;
        const LTDCDIV_DIV62 = 0x3D << 8;
        const LTDCDIV_DIV61 = 0x3C << 8;
        const LTDCDIV_DIV60 = 0x3B << 8;
        const LTDCDIV_DIV59 = 0x3A << 8;
        const LTDCDIV_DIV58 = 0x39 << 8;
        const LTDCDIV_DIV57 = 0x38 << 8;
        const LTDCDIV_DIV56 = 0x37 << 8;
        const LTDCDIV_DIV55 = 0x36 << 8;
        const LTDCDIV_DIV54 = 0x35 << 8;
        const LTDCDIV_DIV53 = 0x34 << 8;
        const LTDCDIV_DIV52 = 0x33 << 8;
        const LTDCDIV_DIV51 = 0x32 << 8;
        const LTDCDIV_DIV50 = 0x31 << 8;
        const LTDCDIV_DIV49 = 0x30 << 8;
        const LTDCDIV_DIV48 = 0x2F << 8;
        const LTDCDIV_DIV47 = 0x2E << 8;
        const LTDCDIV_DIV46 = 0x2D << 8;
        const LTDCDIV_DIV45 = 0x2C << 8;
        const LTDCDIV_DIV44 = 0x2B << 8;
        const LTDCDIV_DIV43 = 0x2A << 8;
        const LTDCDIV_DIV42 = 0x29 << 8;
        const LTDCDIV_DIV41 = 0x28 << 8;
        const LTDCDIV_DIV40 = 0x27 << 8;
        const LTDCDIV_DIV39 = 0x26 << 8;
        const LTDCDIV_DIV38 = 0x25 << 8;
        const LTDCDIV_DIV37 = 0x24 << 8;
        const LTDCDIV_DIV36 = 0x23 << 8;
        const LTDCDIV_DIV35 = 0x22 << 8;
        const LTDCDIV_DIV34 = 0x21 << 8;
        const LTDCDIV_DIV33 = 0x20 << 8;
        const LTDCDIV_DIV32 = 0x1F << 8;
        const LTDCDIV_DIV31 = 0x1E << 8;
        const LTDCDIV_DIV30 = 0x1D << 8;
        const LTDCDIV_DIV29 = 0x1C << 8;
        const LTDCDIV_DIV28 = 0x1B << 8;
        const LTDCDIV_DIV27 = 0x1A << 8;
        const LTDCDIV_DIV26 = 0x19 << 8;
        const LTDCDIV_DIV25 = 0x18 << 8;
        const LTDCDIV_DIV24 = 0x17 << 8;
        const LTDCDIV_DIV23 = 0x16 << 8;
        const LTDCDIV_DIV22 = 0x15 << 8;
        const LTDCDIV_DIV21 = 0x14 << 8;
        const LTDCDIV_DIV20 = 0x13 << 8;
        const LTDCDIV_DIV19 = 0x12 << 8;
        const LTDCDIV_DIV18 = 0x11 << 8;
        const LTDCDIV_DIV17 = 0x10 << 8;
        const LTDCDIV_DIV16 = 0x0F << 8;
        const LTDCDIV_DIV15 = 0x0E << 8;
        const LTDCDIV_DIV14 = 0x0D << 8;
        const LTDCDIV_DIV13 = 0x0C << 8;
        const LTDCDIV_DIV12 = 0x0B << 8;
        const LTDCDIV_DIV11 = 0x0A << 8;
        const LTDCDIV_DIV10 = 0x09 << 8;
        const LTDCDIV_DIV9 = 0x08 << 8;
        const LTDCDIV_DIV8 = 0x07 << 8;
        const LTDCDIV_DIV7 = 0x06 << 8;
        const LTDCDIV_DIV6 = 0x05 << 8;
        const LTDCDIV_DIV5 = 0x04 << 8;
        const LTDCDIV_DIV4 = 0x03 << 8;
        const LTDCDIV_DIV3 = 0x02 << 8;
        const LTDCDIV_DIV2 = 0x01 << 8;
        const LTDCDIV_DIV1 = 0x00 << 8;

        const UHSIFSRC_MASK = 0x3 << 6;
        const UHSIFSRC = 0x3 << 6;
        const UHSIFSRC_1 = 1 << 7;
        const UHSIFSRC_0 = 1 << 6;
        const UHSIFSRC_ETHPLL = 0b11 << 6;
        const UHSIFSRC_USBHSPLL = 0b10 << 6;
        const UHSIFSRC_PLLCLK = 0b01 << 6;
        const UHSIFSRC_SYSCLK = 0b00 << 6;

        const UHSIFDIV_MASK = 0x3F << 0;
        const UHSIFDIV = 0x3F << 0;
        const UHSIFDIV_5 = 1 << 5;
        const UHSIFDIV_4 = 1 << 4;
        const UHSIFDIV_3 = 1 << 3;
        const UHSIFDIV_2 = 1 << 2;
        const UHSIFDIV_1 = 1 << 1;
        const UHSIFDIV_0 = 1 << 0;
        const UHSIFDIV_DIV64 = 0x3F;
        const UHSIFDIV_DIV63 = 0x3E;
        const UHSIFDIV_DIV62 = 0x3D;
        const UHSIFDIV_DIV61 = 0x3C;
        const UHSIFDIV_DIV60 = 0x3B;
        const UHSIFDIV_DIV59 = 0x3A;
        const UHSIFDIV_DIV58 = 0x39;
        const UHSIFDIV_DIV57 = 0x38;
        const UHSIFDIV_DIV56 = 0x37;
        const UHSIFDIV_DIV55 = 0x36;
        const UHSIFDIV_DIV54 = 0x35;
        const UHSIFDIV_DIV53 = 0x34;
        const UHSIFDIV_DIV52 = 0x33;
        const UHSIFDIV_DIV51 = 0x32;
        const UHSIFDIV_DIV50 = 0x31;
        const UHSIFDIV_DIV49 = 0x30;
        const UHSIFDIV_DIV48 = 0x2F;
        const UHSIFDIV_DIV47 = 0x2E;
        const UHSIFDIV_DIV46 = 0x2D;
        const UHSIFDIV_DIV45 = 0x2C;
        const UHSIFDIV_DIV44 = 0x2B;
        const UHSIFDIV_DIV43 = 0x2A;
        const UHSIFDIV_DIV42 = 0x29;
        const UHSIFDIV_DIV41 = 0x28;
        const UHSIFDIV_DIV40 = 0x27;
        const UHSIFDIV_DIV39 = 0x26;
        const UHSIFDIV_DIV38 = 0x25;
        const UHSIFDIV_DIV37 = 0x24;
        const UHSIFDIV_DIV36 = 0x23;
        const UHSIFDIV_DIV35 = 0x22;
        const UHSIFDIV_DIV34 = 0x21;
        const UHSIFDIV_DIV33 = 0x20;
        const UHSIFDIV_DIV32 = 0x1F;
        const UHSIFDIV_DIV31 = 0x1E;
        const UHSIFDIV_DIV30 = 0x1D;
        const UHSIFDIV_DIV29 = 0x1C;
        const UHSIFDIV_DIV28 = 0x1B;
        const UHSIFDIV_DIV27 = 0x1A;
        const UHSIFDIV_DIV26 = 0x19;
        const UHSIFDIV_DIV25 = 0x18;
        const UHSIFDIV_DIV24 = 0x17;
        const UHSIFDIV_DIV23 = 0x16;
        const UHSIFDIV_DIV22 = 0x15;
        const UHSIFDIV_DIV21 = 0x14;
        const UHSIFDIV_DIV20 = 0x13;
        const UHSIFDIV_DIV19 = 0x12;
        const UHSIFDIV_DIV18 = 0x11;
        const UHSIFDIV_DIV17 = 0x10;
        const UHSIFDIV_DIV16 = 0x0F;
        const UHSIFDIV_DIV15 = 0x0E;
        const UHSIFDIV_DIV14 = 0x0D;
        const UHSIFDIV_DIV13 = 0x0C;
        const UHSIFDIV_DIV12 = 0x0B;
        const UHSIFDIV_DIV11 = 0x0A;
        const UHSIFDIV_DIV10 = 0x09;
        const UHSIFDIV_DIV9 = 0x08;
        const UHSIFDIV_DIV8 = 0x07;
        const UHSIFDIV_DIV7 = 0x06;
        const UHSIFDIV_DIV6 = 0x05;
        const UHSIFDIV_DIV5 = 0x04;
        const UHSIFDIV_DIV4 = 0x03;
        const UHSIFDIV_DIV3 = 0x02;
        const UHSIFDIV_DIV2 = 0x01;
        const UHSIFDIV_DIV1 = 0x00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PLLCFGR2: u32 {
        const SERDESPLL_MUL_MASK = 0xF << 16;
        const SERDESPLL_MUL = 0xF << 16;
        const SERDESPLL_MUL_3 = 1 << 19;
        const SERDESPLL_MUL_2 = 1 << 18;
        const SERDESPLL_MUL_1 = 1 << 17;
        const SERDESPLL_MUL_0 = 1 << 16;
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

        const USBHSPLL_IN_DIV_MASK = 0x1F << 8;
        const USBHSPLL_IN_DIV = 0x1F << 8;
        const USBHSPLL_IN_DIV_4 = 1 << 12;
        const USBHSPLL_IN_DIV_3 = 1 << 11;
        const USBHSPLL_IN_DIV_2 = 1 << 10;
        const USBHSPLL_IN_DIV_1 = 1 << 9;
        const USBHSPLL_IN_DIV_0 = 1 << 8;
        const USBHSPLL_IN_DIV32 = 0x1F << 8;
        const USBHSPLL_IN_DIV31 = 0x1E << 8;
        const USBHSPLL_IN_DIV30 = 0x1D << 8;
        const USBHSPLL_IN_DIV29 = 0x1C << 8;
        const USBHSPLL_IN_DIV28 = 0x1B << 8;
        const USBHSPLL_IN_DIV27 = 0x1A << 8;
        const USBHSPLL_IN_DIV26 = 0x19 << 8;
        const USBHSPLL_IN_DIV25 = 0x18 << 8;
        const USBHSPLL_IN_DIV24 = 0x17 << 8;
        const USBHSPLL_IN_DIV23 = 0x16 << 8;
        const USBHSPLL_IN_DIV22 = 0x15 << 8;
        const USBHSPLL_IN_DIV21 = 0x14 << 8;
        const USBHSPLL_IN_DIV20 = 0x13 << 8;
        const USBHSPLL_IN_DIV19 = 0x12 << 8;
        const USBHSPLL_IN_DIV18 = 0x11 << 8;
        const USBHSPLL_IN_DIV17 = 0x10 << 8;
        const USBHSPLL_IN_DIV16 = 0x0F << 8;
        const USBHSPLL_IN_DIV15 = 0x0E << 8;
        const USBHSPLL_IN_DIV14 = 0x0D << 8;
        const USBHSPLL_IN_DIV13 = 0x0C << 8;
        const USBHSPLL_IN_DIV12 = 0x0B << 8;
        const USBHSPLL_IN_DIV11 = 0x0A << 8;
        const USBHSPLL_IN_DIV10 = 0x09 << 8;
        const USBHSPLL_IN_DIV9 = 0x08 << 8;
        const USBHSPLL_IN_DIV8 = 0x07 << 8;
        const USBHSPLL_IN_DIV7 = 0x06 << 8;
        const USBHSPLL_IN_DIV6 = 0x05 << 8;
        const USBHSPLL_IN_DIV5 = 0x04 << 8;
        const USBHSPLL_IN_DIV4 = 0x03 << 8;
        const USBHSPLL_IN_DIV3 = 0x02 << 8;
        const USBHSPLL_IN_DIV2 = 0x01 << 8;
        const USBHSPLL_IN_DIV1 = 0x00 << 8;

        const USBSSPLL_REFSEL_MASK = 0x3 << 4;
        const USBSSPLL_REFSEL = 0x7 << 4;
        const USBSSPLL_REFSEL_2 = 0b11 << 4;
        const USBSSPLL_REFSEL_1 = 1 << 5;
        const USBSSPLL_REFSEL_0 = 1 << 4;
        const USBSSPLL_REFSEL_80MHZ = 0b111 << 4;
        const USBSSPLL_REFSEL_60MHZ = 0b110 << 4;
        const USBSSPLL_REFSEL_40MHZ = 0b101 << 4;
        const USBSSPLL_REFSEL_32MHZ = 0b100 << 4;
        const USBSSPLL_REFSEL_30MHZ = 0b011 << 4;
        const USBSSPLL_REFSEL_25MHZ = 0b010 << 4;
        const USBSSPLL_REFSEL_24MHZ = 0b001 << 4;
        const USBSSPLL_REFSEL_20MHZ = 0b000 << 4;

        const USBHSPLL_REFSEL_MASK = 0x3 << 2;
        const USBHSPLL_REFSEL = 0x3 << 2;
        const USBHSPLL_REFSEL_1 = 1 << 3;
        const USBHSPLL_REFSEL_0 = 1 << 2;
        const USBHSPLL_REFSEL_32MHZ = 0b11 << 2;
        const USBHSPLL_REFSEL_24MHZ = 0b10 << 2;
        const USBHSPLL_REFSEL_20MHZ = 0b01 << 2;
        const USBHSPLL_REFSEL_25MHZ = 0b00 << 2;

        const USBHSPLLSRC_MASK = 0x3 << 0;
        const USBHSPLLSRC = 0x3 << 0;
        const USBHSPLLSRC_1 = 1 << 1;
        const USBHSPLLSRC_0 = 1 << 0;
        const USBHSPLLSRC_PLLCLK = 0b11 << 0;
        const USBHSPLLSRC_ETHCLK_20M = 0b10 << 0;
        const USBHSPLLSRC_HSI = 0b01 << 0;
        const USBHSPLLSRC_HSE = 0b00 << 0;
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

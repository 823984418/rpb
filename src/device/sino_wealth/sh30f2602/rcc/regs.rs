use bitflags::bitflags;

use crate::register::{Reg, Reserved};

/// RCC register block.
#[derive(Debug)]
#[repr(C)]
pub struct RCC {
    pub CR: Reg<CR>,
    pub CFGR: Reg<CFGR>,
    pub CIENR: Reg<CIENR>,
    pub CISTR: Reg<CISTR>,
    pub CICLR: Reg<CICLR>,
    pub AHBRSTR: Reg<AHBRSTR>,
    /// Reserved (0x18..=0x1b).
    RESERVED0: Reserved<[u32; 1]>,
    pub APB0RSTR: Reg<APB0RSTR>,
    pub AHBENR: Reg<AHBENR>,
    /// Reserved (0x24..=0x27).
    RESERVED1: Reserved<[u32; 1]>,
    pub APB0ENR: Reg<APB0ENR>,
    pub RSTSTR: Reg<RSTSTR>,
    pub RSTCLR: Reg<RSTCLR>,
    /// Reserved (0x34..=0x37).
    RESERVED2: Reserved<[u32; 1]>,
    pub RCCLOCK: Reg<RCCLOCK>,
}

bitflags! {
    /// `CR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        /// CSM时钟检测功能使能
        const CSMON = 1 << 8;

        /// PLL锁相环就绪标志
        const PLLRDY = 1 << 7;

        /// PLL锁相环使能
        const PLLON = 1 << 6;

        const SWS_PLL_ = 0b11 << 2;
        const SWS_PLL = 0b10 << 2;
        const SWS_HSI_ = 0b01 << 2;
        const SWS_HSI = 0b00 << 2;
        /// 系统时钟切换状态
        const SWS_MASK = 0x3 << 2;

        const SW_PLL_ = 0b11 << 0;
        const SW_PLL = 0b10 << 0;
        const SW_HSI_ = 0b01 << 0;
        const SW_HSI = 0b00 << 0;
        /// 系统时钟切换
        const SW_MASK = 0x3 << 0;
    }
}

bitflags! {
    /// `CFGR` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        /// PLL倍频因子F
        /// 0b000000 => 15
        /// 0b111111 => 78
        const PLLF_MASK = 0x3f << 12;

        const PLLK_8 = 0b111 << 9;
        const PLLK_7 = 0b110 << 9;
        const PLLK_6 = 0b101 << 9;
        const PLLK_5 = 0b100 << 9;
        const PLLK_4 = 0b011 << 9;
        const PLLK_3 = 0b010 << 9;
        const PLLK_2 = 0b001 << 9;
        const PLLK_1 = 0b000 << 9;
        /// PLL分频因子K
        const PLLK_MASK = 0x7 << 9;

        const PPRE_16 = 0b100 << 3;
        const PPRE_8 = 0b011 << 3;
        const PPRE_4 = 0b010 << 3;
        const PPRE_2 = 0b001 << 3;
        const PPRE_1 = 0b000 << 3;
        /// 低速APB预分频
        /// 其他为HCLK16分频
        const PPRE_MASK = 0x7 << 3;

        const HPRE_32 = 0b101 << 0;
        const HPRE_16 = 0b100 << 0;
        const HPRE_8 = 0b011 << 0;
        const HPRE_4 = 0b010 << 0;
        const HPRE_2 = 0b001 << 0;
        const HPRE_1 = 0b000 << 0;
        /// AHB预分频
        const HPRE_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `CIENR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CIENR: u32 {
        /// PLL就绪中断使能
        const PLLRDYIE = 1 << 4;
    }
}

bitflags! {
    /// `CISTR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CISTR: u32 {
        /// PLL异常中断标志
        const CSMPLLF = 1 << 7;

        /// PLL就绪中断标志
        const PLLRDYIF = 1 << 4;
    }
}

bitflags! {
    /// `CICLR` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CICLR: u32 {
        /// 清除CSM中断
        const CSMC = 1 << 7;

        /// 清除PLL就绪中断
        const PLLRDYC = 1 << 4;
    }
}

bitflags! {
    /// `AHBRSTR` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AHBRSTR: u32 {
        const ADCRST = 1 << 5;

        const MACPRST = 1 << 4;

        const SYSCFGRST = 1 << 2;

        const MCMRST = 1 << 1;

        const GPIORST = 1 << 0;
    }
}

bitflags! {
    /// `APB0RSTR` (0x1c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct APB0RSTR: u32 {
        const NTESTRST = 1 << 14;

        const PCA0RST = 1 << 10;

        const AMOCRST = 1 << 9;

        const WWDTRST = 1 << 8;

        const PWM4RST = 1 << 7;

        const SPI0RST = 1 << 6;

        const PWM3RST = 1 << 5;

        const UART1RST = 1 << 4;

        const UART0RST = 1 << 3;

        const TIM8RST = 1 << 1;

        const TIM7RST = 1 << 0;
    }
}

bitflags! {
    /// `AHBENR` (0x20).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AHBENR: u32 {
        const ADCEN = 1 << 5;

        const MACPEN = 1 << 4;

        const SYSCFGEN = 1 << 2;

        const MCMEN = 1 << 1;

        const GPIOEN = 1 << 0;
    }
}

bitflags! {
    /// `APB0ENR` (0x28).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct APB0ENR: u32 {
        const NTESTEN = 1 << 14;

        const PCA0EN = 1 << 10;

        const AMOCEN = 1 << 9;

        const WWDTEN = 1 << 8;

        const PWM4EN = 1 << 7;

        const SPI0EN = 1 << 6;

        const PWM3EN = 1 << 5;

        const UART1EN = 1 << 4;

        const UART0EN = 1 << 3;

        const TIM8EN = 1 << 1;

        const TIM7EN = 1 << 0;
    }
}

bitflags! {
    /// `RSTSTR` (0x2c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSTSTR: u32 {
        const WWDTRSTF = 1 << 5;

        const IWDTRSTF = 1 << 4;

        const SWRSTF = 1 << 3;

        const PORSTF = 1 << 2;

        const LVRSTF = 1 << 1;

        const PINRSTF = 1 << 0;
    }
}

bitflags! {
    /// `RSTCLR` (0x30).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSTCLR: u32 {
        const WWDTRSTFC = 1 << 5;

        const IWDTRSTFC = 1 << 4;

        const SWRSTFC = 1 << 3;

        const PORSTFC = 1 << 2;

        const LVRSTFC = 1 << 1;

        const PINRSTFC = 1 << 0;
    }
}

bitflags! {
    /// `RCCLOCK` (0x38).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RCCLOCK: u32 {
        const LOCK_VALUE = 0x33CC << 0;
        const LOCK_MASK = 0xFFFF << 0;
    }
}

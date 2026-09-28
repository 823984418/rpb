use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct GPIO {
    pub CFGLR: Reg<CFGR>,
    pub CFGHR: Reg<CFGR>,
    pub INDR: Reg32,
    pub OUTDR: Reg32,
    pub BSHR: Reg32,
    pub BCR: Reg32,
    pub LCKR: Reg32,
    pub SPEED: Reg<SPEED>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const CFGLR_CNF_MASK = 0xCCCCCCCC << 0;

        const CFGLR_CNF7_MASK = 0x3 << 30;

        const CFGLR_MODE_MASK = 0x33333333 << 0;

        const CFGLR_MODE7_MASK = 0x3 << 28;

        const CFGLR_CNF6_MASK = 0x3 << 26;

        const CFGLR_MODE6_MASK = 0x3 << 24;

        const CFGLR_CNF5_MASK = 0x3 << 22;

        const CFGLR_MODE5_MASK = 0x3 << 20;

        const CFGLR_CNF4_MASK = 0x3 << 18;

        const CFGLR_MODE4_MASK = 0x3 << 16;

        const CFGLR_CNF3_MASK = 0x3 << 14;

        const CFGLR_MODE3_MASK = 0x3 << 12;

        const CFGLR_CNF2_MASK = 0x3 << 10;

        const CFGLR_MODE2_MASK = 0x3 << 8;

        const CFGLR_CNF1_MASK = 0x3 << 6;

        const CFGLR_MODE1_MASK = 0x3 << 4;

        const CFGLR_CNF0_MASK = 0x3 << 2;

        const CFGLR_MODE0_MASK = 0x3 << 0;

        const CFGHR_CNF_MASK = 0xCCCCCCCC << 0;

        const CFGHR_CNF15_MASK = 0x3 << 30;

        const CFGHR_MODE_MASK = 0x33333333 << 0;

        const CFGHR_MODE15_MASK = 0x3 << 28;

        const CFGHR_CNF14_MASK = 0x3 << 26;

        const CFGHR_MODE14_MASK = 0x3 << 24;

        const CFGHR_CNF13_MASK = 0x3 << 22;

        const CFGHR_MODE13_MASK = 0x3 << 20;

        const CFGHR_CNF12_MASK = 0x3 << 18;

        const CFGHR_MODE12_MASK = 0x3 << 16;

        const CFGHR_CNF11_MASK = 0x3 << 14;

        const CFGHR_MODE11_MASK = 0x3 << 12;

        const CFGHR_CNF10_MASK = 0x3 << 10;

        const CFGHR_MODE10_MASK = 0x3 << 8;

        const CFGHR_CNF9_MASK = 0x3 << 6;

        const CFGHR_MODE9_MASK = 0x3 << 4;

        const CFGHR_CNF8_MASK = 0x3 << 2;

        const CFGHR_MODE8_MASK = 0x3 << 0;
    }
}

impl CFGR {
    pub const fn pin(self, n: u32) -> CFGR_PIN {
        CFGR_PIN::from_bits_retain(self.bits() >> (n * 4))
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR_PIN: u32 {
        const AF_OD_OUT = 0b11_01;
        const AF_PP_OUT = 0b10_01;
        const OD_OUT = 0b01_01;
        const PP_OUT = 0b00_01;

        const RES_IN = 0b10_00;
        const FLOAT_IN = 0b01_00;
        const ANALOG_IN = 0b00_00;

        const MASK = 0b11_00;
    }
}

impl CFGR_PIN {
    pub const fn pin(self, n: u32) -> CFGR {
        CFGR::from_bits_retain(self.bits() << (n * 4))
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SPEED: u32 {}
}

impl SPEED {
    pub const fn pin(self, n: u32) -> SPEED_PIN {
        SPEED_PIN::from_bits_retain(self.bits() >> (n * 2))
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SPEED_PIN: u32 {
        const SPEED_180M = 0b11;
        const SPEED_100M = 0b10;
        const SPEED_50M = 0b01;
        const SPEED_10M = 0b00;

        const MASK = 0b11;
    }
}

impl SPEED_PIN {
    pub const fn pin(self, n: u32) -> SPEED {
        SPEED::from_bits_retain(self.bits() << (n * 2))
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INDR: u32 {
        const IDR15 = 1 << 15;
        const IDR14 = 1 << 14;
        const IDR13 = 1 << 13;
        const IDR12 = 1 << 12;
        const IDR11 = 1 << 11;
        const IDR10 = 1 << 10;
        const IDR9 = 1 << 9;
        const IDR8 = 1 << 8;
        const IDR7 = 1 << 7;
        const IDR6 = 1 << 6;
        const IDR5 = 1 << 5;
        const IDR4 = 1 << 4;
        const IDR3 = 1 << 3;
        const IDR2 = 1 << 2;
        const IDR1 = 1 << 1;
        const IDR0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OUTDR: u32 {
        const ODR15 = 1 << 15;
        const ODR14 = 1 << 14;
        const ODR13 = 1 << 13;
        const ODR12 = 1 << 12;
        const ODR11 = 1 << 11;
        const ODR10 = 1 << 10;
        const ODR9 = 1 << 9;
        const ODR8 = 1 << 8;
        const ODR7 = 1 << 7;
        const ODR6 = 1 << 6;
        const ODR5 = 1 << 5;
        const ODR4 = 1 << 4;
        const ODR3 = 1 << 3;
        const ODR2 = 1 << 2;
        const ODR1 = 1 << 1;
        const ODR0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BSHR: u32 {
        const BR15 = 1 << 31;
        const BR14 = 1 << 30;
        const BR13 = 1 << 29;
        const BR12 = 1 << 28;
        const BR11 = 1 << 27;
        const BR10 = 1 << 26;
        const BR9 = 1 << 25;
        const BR8 = 1 << 24;
        const BR7 = 1 << 23;
        const BR6 = 1 << 22;
        const BR5 = 1 << 21;
        const BR4 = 1 << 20;
        const BR3 = 1 << 19;
        const BR2 = 1 << 18;
        const BR1 = 1 << 17;
        const BR0 = 1 << 16;

        const BS15 = 1 << 15;
        const BS14 = 1 << 14;
        const BS13 = 1 << 13;
        const BS12 = 1 << 12;
        const BS11 = 1 << 11;
        const BS10 = 1 << 10;
        const BS9 = 1 << 9;
        const BS8 = 1 << 8;
        const BS7 = 1 << 7;
        const BS6 = 1 << 6;
        const BS5 = 1 << 5;
        const BS4 = 1 << 4;
        const BS3 = 1 << 3;
        const BS2 = 1 << 2;
        const BS1 = 1 << 1;
        const BS0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BCR: u32 {
        const BR15 = 1 << 15;
        const BR14 = 1 << 14;
        const BR13 = 1 << 13;
        const BR12 = 1 << 12;
        const BR11 = 1 << 11;
        const BR10 = 1 << 10;
        const BR9 = 1 << 9;
        const BR8 = 1 << 8;
        const BR7 = 1 << 7;
        const BR6 = 1 << 6;
        const BR5 = 1 << 5;
        const BR4 = 1 << 4;
        const BR3 = 1 << 3;
        const BR2 = 1 << 2;
        const BR1 = 1 << 1;
        const BR0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LCKR: u32 {
        const LCKK = 1 << 16;
        const LCK15 = 1 << 15;
        const LCK14 = 1 << 14;
        const LCK13 = 1 << 13;
        const LCK12 = 1 << 12;
        const LCK11 = 1 << 11;
        const LCK10 = 1 << 10;
        const LCK9 = 1 << 9;
        const LCK8 = 1 << 8;
        const LCK7 = 1 << 7;
        const LCK6 = 1 << 6;
        const LCK5 = 1 << 5;
        const LCK4 = 1 << 4;
        const LCK3 = 1 << 3;
        const LCK2 = 1 << 2;
        const LCK1 = 1 << 1;
        const LCK0 = 1 << 0;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct AFIO {
    pub PCFR1: Reg32,
    pub GPIOA_AFLR: Reg32,
    pub GPIOA_AFHR: Reg32,
    pub GPIOB_AFLR: Reg32,
    pub GPIOB_AFHR: Reg32,
    pub GPIOC_AFLR: Reg32,
    pub GPIOC_AFHR: Reg32,
    pub GPIOD_AFLR: Reg32,
    pub GPIOD_AFHR: Reg32,
    pub GPIOE_AFLR: Reg32,
    pub GPIOE_AFHR: Reg32,
    pub GPIOF_AFLR: Reg32,
    pub GPIOF_AFHR: Reg32,
    RESERVED0: Reserved<[u32; 2]>,
    pub EXTICR1: Reg32,
    pub EXTICR2: Reg32,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PCFR1: u32 {
        const SWJ_CFG_MASK = 0x7 << 24;

        const USBPD_CC_HVT = 1 << 20;

        const VDD33_IO_HSLV = 1 << 18;
        const VIO33_IO_HSLV = 1 << 17;
        const VIO18_IO_HSLV = 1 << 16;

        const TIM2ITR1_REMAP = 1 << 12;

        const SDMMC_REMAP_MASK = 0x3 << 10;

        const UHSIF_PORT_REMAP_MASK = 0x3 << 8;

        const UHSIF_CLK_REMAP_MASK = 0x3 << 6;

        const ADC2_ETRGREG_REMAP = 1 << 4;
        const ADC2_ETRGINJ_REMAP = 1 << 3;
        const ADC1_ETRGREG_REMAP = 1 << 2;
        const ADC1_ETRGINJ_REMAP = 1 << 1;
        const PD0_1_REMAP = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AFLR: u32 {
        const AFR7_MASK = 0xF << 28;

        const AFR6_MASK = 0xF << 24;

        const AFR5_MASK = 0xF << 20;

        const AFR4_MASK = 0xF << 16;

        const AFR3_MASK = 0xF << 12;

        const AFR2_MASK = 0xF << 8;

        const AFR1_MASK = 0xF << 4;

        const AFR0_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AFHR: u32 {
        const AFR15_MASK = 0xF << 28;

        const AFR14_MASK = 0xF << 24;

        const AFR13_MASK = 0xF << 20;

        const AFR12_MASK = 0xF << 16;

        const AFR11_MASK = 0xF << 12;

        const AFR10_MASK = 0xF << 8;

        const AFR9_MASK = 0xF << 4;

        const AFR8_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EXTICR1: u32 {
        const EXTI7_CMPOUT = 0b0110 << 28;
        const EXTI7_PF = 0b0101 << 28;
        const EXTI7_PE = 0b0100 << 28;
        const EXTI7_PD = 0b0011 << 28;
        const EXTI7_PC = 0b0010 << 28;
        const EXTI7_PB = 0b0001 << 28;
        const EXTI7_PA = 0b0000 << 28;
        const EXTI7_MASK = 0xF << 28;

        const EXTI6_CMPOUT = 0b0110 << 24;
        const EXTI6_PF = 0b0101 << 24;
        const EXTI6_PE = 0b0100 << 24;
        const EXTI6_PD = 0b0011 << 24;
        const EXTI6_PC = 0b0010 << 24;
        const EXTI6_PB = 0b0001 << 24;
        const EXTI6_PA = 0b0000 << 24;
        const EXTI6_MASK = 0xF << 24;

        const EXTI5_CMPOUT = 0b0110 << 20;
        const EXTI5_PF = 0b0101 << 20;
        const EXTI5_PE = 0b0100 << 20;
        const EXTI5_PD = 0b0011 << 20;
        const EXTI5_PC = 0b0010 << 20;
        const EXTI5_PB = 0b0001 << 20;
        const EXTI5_PA = 0b0000 << 20;
        const EXTI5_MASK = 0xF << 20;

        const EXTI4_CMPOUT = 0b0110 << 16;
        const EXTI4_PF = 0b0101 << 16;
        const EXTI4_PE = 0b0100 << 16;
        const EXTI4_PD = 0b0011 << 16;
        const EXTI4_PC = 0b0010 << 16;
        const EXTI4_PB = 0b0001 << 16;
        const EXTI4_PA = 0b0000 << 16;
        const EXTI4_MASK = 0xF << 16;

        const EXTI3_CMPOUT = 0b0110 << 12;
        const EXTI3_PF = 0b0101 << 12;
        const EXTI3_PE = 0b0100 << 12;
        const EXTI3_PD = 0b0011 << 12;
        const EXTI3_PC = 0b0010 << 12;
        const EXTI3_PB = 0b0001 << 12;
        const EXTI3_PA = 0b0000 << 12;
        const EXTI3_MASK = 0xF << 12;

        const EXTI2_CMPOUT = 0b0110 << 8;
        const EXTI2_PF = 0b0101 << 8;
        const EXTI2_PE = 0b0100 << 8;
        const EXTI2_PD = 0b0011 << 8;
        const EXTI2_PC = 0b0010 << 8;
        const EXTI2_PB = 0b0001 << 8;
        const EXTI2_PA = 0b0000 << 8;
        const EXTI2_MASK = 0xF << 8;

        const EXTI1_CMPOUT = 0b0110 << 4;
        const EXTI1_PF = 0b0101 << 4;
        const EXTI1_PE = 0b0100 << 4;
        const EXTI1_PD = 0b0011 << 4;
        const EXTI1_PC = 0b0010 << 4;
        const EXTI1_PB = 0b0001 << 4;
        const EXTI1_PA = 0b0000 << 4;
        const EXTI1_MASK = 0xF << 4;

        const EXTI0_CMPOUT = 0b0110 << 0;
        const EXTI0_PF = 0b0101 << 0;
        const EXTI0_PE = 0b0100 << 0;
        const EXTI0_PD = 0b0011 << 0;
        const EXTI0_PC = 0b0010 << 0;
        const EXTI0_PB = 0b0001 << 0;
        const EXTI0_PA = 0b0000 << 0;
        const EXTI0_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EXTICR2: u32 {
        const EXTI15_CMPOUT = 0b0110 << 28;
        const EXTI15_PF = 0b0101 << 28;
        const EXTI15_PE = 0b0100 << 28;
        const EXTI15_PD = 0b0011 << 28;
        const EXTI15_PC = 0b0010 << 28;
        const EXTI15_PB = 0b0001 << 28;
        const EXTI15_PA = 0b0000 << 28;
        const EXTI15_MASK = 0xF << 28;

        const EXTI14_CMPOUT = 0b0110 << 24;
        const EXTI14_PF = 0b0101 << 24;
        const EXTI14_PE = 0b0100 << 24;
        const EXTI14_PD = 0b0011 << 24;
        const EXTI14_PC = 0b0010 << 24;
        const EXTI14_PB = 0b0001 << 24;
        const EXTI14_PA = 0b0000 << 24;
        const EXTI14_MASK = 0xF << 24;

        const EXTI13_CMPOUT = 0b0110 << 20;
        const EXTI13_PF = 0b0101 << 20;
        const EXTI13_PE = 0b0100 << 20;
        const EXTI13_PD = 0b0011 << 20;
        const EXTI13_PC = 0b0010 << 20;
        const EXTI13_PB = 0b0001 << 20;
        const EXTI13_PA = 0b0000 << 20;
        const EXTI13_MASK = 0xF << 20;

        const EXTI12_CMPOUT = 0b0110 << 16;
        const EXTI12_PF = 0b0101 << 16;
        const EXTI12_PE = 0b0100 << 16;
        const EXTI12_PD = 0b0011 << 16;
        const EXTI12_PC = 0b0010 << 16;
        const EXTI12_PB = 0b0001 << 16;
        const EXTI12_PA = 0b0000 << 16;
        const EXTI12_MASK = 0xF << 16;

        const EXTI11_CMPOUT = 0b0110 << 12;
        const EXTI11_PF = 0b0101 << 12;
        const EXTI11_PE = 0b0100 << 12;
        const EXTI11_PD = 0b0011 << 12;
        const EXTI11_PC = 0b0010 << 12;
        const EXTI11_PB = 0b0001 << 12;
        const EXTI11_PA = 0b0000 << 12;
        const EXTI11_MASK = 0xF << 12;

        const EXTI10_CMPOUT = 0b0110 << 8;
        const EXTI10_PF = 0b0101 << 8;
        const EXTI10_PE = 0b0100 << 8;
        const EXTI10_PD = 0b0011 << 8;
        const EXTI10_PC = 0b0010 << 8;
        const EXTI10_PB = 0b0001 << 8;
        const EXTI10_PA = 0b0000 << 8;
        const EXTI10_MASK = 0xF << 8;

        const EXTI9_CMPOUT = 0b0110 << 4;
        const EXTI9_PF = 0b0101 << 4;
        const EXTI9_PE = 0b0100 << 4;
        const EXTI9_PD = 0b0011 << 4;
        const EXTI9_PC = 0b0010 << 4;
        const EXTI9_PB = 0b0001 << 4;
        const EXTI9_PA = 0b0000 << 4;
        const EXTI9_MASK = 0xF << 4;

        const EXTI8_CMPOUT = 0b0110 << 0;
        const EXTI8_PF = 0b0101 << 0;
        const EXTI8_PE = 0b0100 << 0;
        const EXTI8_PD = 0b0011 << 0;
        const EXTI8_PC = 0b0010 << 0;
        const EXTI8_PB = 0b0001 << 0;
        const EXTI8_PA = 0b0000 << 0;
        const EXTI8_MASK = 0xF << 0;
    }
}

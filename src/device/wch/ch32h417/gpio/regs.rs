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
        const CFGLR_MODE = 0x33333333;

        const CFGLR_MODE0 = 0x00000003;
        const CFGLR_MODE0_0 = 0x00000001;
        const CFGLR_MODE0_1 = 0x00000002;

        const CFGLR_MODE1 = 0x00000030;
        const CFGLR_MODE1_0 = 0x00000010;
        const CFGLR_MODE1_1 = 0x00000020;

        const CFGLR_MODE2 = 0x00000300;
        const CFGLR_MODE2_0 = 0x00000100;
        const CFGLR_MODE2_1 = 0x00000200;

        const CFGLR_MODE3 = 0x00003000;
        const CFGLR_MODE3_0 = 0x00001000;
        const CFGLR_MODE3_1 = 0x00002000;

        const CFGLR_MODE4 = 0x00030000;
        const CFGLR_MODE4_0 = 0x00010000;
        const CFGLR_MODE4_1 = 0x00020000;

        const CFGLR_MODE5 = 0x00300000;
        const CFGLR_MODE5_0 = 0x00100000;
        const CFGLR_MODE5_1 = 0x00200000;

        const CFGLR_MODE6 = 0x03000000;
        const CFGLR_MODE6_0 = 0x01000000;
        const CFGLR_MODE6_1 = 0x02000000;

        const CFGLR_MODE7 = 0x30000000;
        const CFGLR_MODE7_0 = 0x10000000;
        const CFGLR_MODE7_1 = 0x20000000;

        const CFGLR_CNF = 0xCCCCCCCC;

        const CFGLR_CNF0 = 0x0000000C;
        const CFGLR_CNF0_0 = 0x00000004;
        const CFGLR_CNF0_1 = 0x00000008;

        const CFGLR_CNF1 = 0x000000C0;
        const CFGLR_CNF1_0 = 0x00000040;
        const CFGLR_CNF1_1 = 0x00000080;

        const CFGLR_CNF2 = 0x00000C00;
        const CFGLR_CNF2_0 = 0x00000400;
        const CFGLR_CNF2_1 = 0x00000800;

        const CFGLR_CNF3 = 0x0000C000;
        const CFGLR_CNF3_0 = 0x00004000;
        const CFGLR_CNF3_1 = 0x00008000;

        const CFGLR_CNF4 = 0x000C0000;
        const CFGLR_CNF4_0 = 0x00040000;
        const CFGLR_CNF4_1 = 0x00080000;

        const CFGLR_CNF5 = 0x00C00000;
        const CFGLR_CNF5_0 = 0x00400000;
        const CFGLR_CNF5_1 = 0x00800000;

        const CFGLR_CNF6 = 0x0C000000;
        const CFGLR_CNF6_0 = 0x04000000;
        const CFGLR_CNF6_1 = 0x08000000;

        const CFGLR_CNF7 = 0xC0000000;
        const CFGLR_CNF7_0 = 0x40000000;
        const CFGLR_CNF7_1 = 0x80000000;

        const CFGHR_MODE = 0x33333333;

        const CFGHR_MODE8 = 0x00000003;
        const CFGHR_MODE8_0 = 0x00000001;
        const CFGHR_MODE8_1 = 0x00000002;

        const CFGHR_MODE9 = 0x00000030;
        const CFGHR_MODE9_0 = 0x00000010;
        const CFGHR_MODE9_1 = 0x00000020;

        const CFGHR_MODE10 = 0x00000300;
        const CFGHR_MODE10_0 = 0x00000100;
        const CFGHR_MODE10_1 = 0x00000200;

        const CFGHR_MODE11 = 0x00003000;
        const CFGHR_MODE11_0 = 0x00001000;
        const CFGHR_MODE11_1 = 0x00002000;

        const CFGHR_MODE12 = 0x00030000;
        const CFGHR_MODE12_0 = 0x00010000;
        const CFGHR_MODE12_1 = 0x00020000;

        const CFGHR_MODE13 = 0x00300000;
        const CFGHR_MODE13_0 = 0x00100000;
        const CFGHR_MODE13_1 = 0x00200000;

        const CFGHR_MODE14 = 0x03000000;
        const CFGHR_MODE14_0 = 0x01000000;
        const CFGHR_MODE14_1 = 0x02000000;

        const CFGHR_MODE15 = 0x30000000;
        const CFGHR_MODE15_0 = 0x10000000;
        const CFGHR_MODE15_1 = 0x20000000;

        const CFGHR_CNF = 0xCCCCCCCC;

        const CFGHR_CNF8 = 0x0000000C;
        const CFGHR_CNF8_0 = 0x00000004;
        const CFGHR_CNF8_1 = 0x00000008;

        const CFGHR_CNF9 = 0x000000C0;
        const CFGHR_CNF9_0 = 0x00000040;
        const CFGHR_CNF9_1 = 0x00000080;

        const CFGHR_CNF10 = 0x00000C00;
        const CFGHR_CNF10_0 = 0x00000400;
        const CFGHR_CNF10_1 = 0x00000800;

        const CFGHR_CNF11 = 0x0000C000;
        const CFGHR_CNF11_0 = 0x00004000;
        const CFGHR_CNF11_1 = 0x00008000;

        const CFGHR_CNF12 = 0x000C0000;
        const CFGHR_CNF12_0 = 0x00040000;
        const CFGHR_CNF12_1 = 0x00080000;

        const CFGHR_CNF13 = 0x00C00000;
        const CFGHR_CNF13_0 = 0x00400000;
        const CFGHR_CNF13_1 = 0x00800000;

        const CFGHR_CNF14 = 0x0C000000;
        const CFGHR_CNF14_0 = 0x04000000;
        const CFGHR_CNF14_1 = 0x08000000;

        const CFGHR_CNF15 = 0xC0000000;
        const CFGHR_CNF15_0 = 0x40000000;
        const CFGHR_CNF15_1 = 0x80000000;
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
        const IDR0 = 0x0001;
        const IDR1 = 0x0002;
        const IDR2 = 0x0004;
        const IDR3 = 0x0008;
        const IDR4 = 0x0010;
        const IDR5 = 0x0020;
        const IDR6 = 0x0040;
        const IDR7 = 0x0080;
        const IDR8 = 0x0100;
        const IDR9 = 0x0200;
        const IDR10 = 0x0400;
        const IDR11 = 0x0800;
        const IDR12 = 0x1000;
        const IDR13 = 0x2000;
        const IDR14 = 0x4000;
        const IDR15 = 0x8000;

        const INDR_IDR0 = 0x0001;
        const INDR_IDR1 = 0x0002;
        const INDR_IDR2 = 0x0004;
        const INDR_IDR3 = 0x0008;
        const INDR_IDR4 = 0x0010;
        const INDR_IDR5 = 0x0020;
        const INDR_IDR6 = 0x0040;
        const INDR_IDR7 = 0x0080;
        const INDR_IDR8 = 0x0100;
        const INDR_IDR9 = 0x0200;
        const INDR_IDR10 = 0x0400;
        const INDR_IDR11 = 0x0800;
        const INDR_IDR12 = 0x1000;
        const INDR_IDR13 = 0x2000;
        const INDR_IDR14 = 0x4000;
        const INDR_IDR15 = 0x8000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OUTDR: u32 {
        const ODR0 = 0x0001;
        const ODR1 = 0x0002;
        const ODR2 = 0x0004;
        const ODR3 = 0x0008;
        const ODR4 = 0x0010;
        const ODR5 = 0x0020;
        const ODR6 = 0x0040;
        const ODR7 = 0x0080;
        const ODR8 = 0x0100;
        const ODR9 = 0x0200;
        const ODR10 = 0x0400;
        const ODR11 = 0x0800;
        const ODR12 = 0x1000;
        const ODR13 = 0x2000;
        const ODR14 = 0x4000;
        const ODR15 = 0x8000;

        const OUTDR_ODR0 = 0x0001;
        const OUTDR_ODR1 = 0x0002;
        const OUTDR_ODR2 = 0x0004;
        const OUTDR_ODR3 = 0x0008;
        const OUTDR_ODR4 = 0x0010;
        const OUTDR_ODR5 = 0x0020;
        const OUTDR_ODR6 = 0x0040;
        const OUTDR_ODR7 = 0x0080;
        const OUTDR_ODR8 = 0x0100;
        const OUTDR_ODR9 = 0x0200;
        const OUTDR_ODR10 = 0x0400;
        const OUTDR_ODR11 = 0x0800;
        const OUTDR_ODR12 = 0x1000;
        const OUTDR_ODR13 = 0x2000;
        const OUTDR_ODR14 = 0x4000;
        const OUTDR_ODR15 = 0x8000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BSHR: u32 {
        const BS0 = 0x00000001;
        const BS1 = 0x00000002;
        const BS2 = 0x00000004;
        const BS3 = 0x00000008;
        const BS4 = 0x00000010;
        const BS5 = 0x00000020;
        const BS6 = 0x00000040;
        const BS7 = 0x00000080;
        const BS8 = 0x00000100;
        const BS9 = 0x00000200;
        const BS10 = 0x00000400;
        const BS11 = 0x00000800;
        const BS12 = 0x00001000;
        const BS13 = 0x00002000;
        const BS14 = 0x00004000;
        const BS15 = 0x00008000;

        const BR0 = 0x00010000;
        const BR1 = 0x00020000;
        const BR2 = 0x00040000;
        const BR3 = 0x00080000;
        const BR4 = 0x00100000;
        const BR5 = 0x00200000;
        const BR6 = 0x00400000;
        const BR7 = 0x00800000;
        const BR8 = 0x01000000;
        const BR9 = 0x02000000;
        const BR10 = 0x04000000;
        const BR11 = 0x08000000;
        const BR12 = 0x10000000;
        const BR13 = 0x20000000;
        const BR14 = 0x40000000;
        const BR15 = 0x80000000;

        const BSHR_BS0 = 0x00000001;
        const BSHR_BS1 = 0x00000002;
        const BSHR_BS2 = 0x00000004;
        const BSHR_BS3 = 0x00000008;
        const BSHR_BS4 = 0x00000010;
        const BSHR_BS5 = 0x00000020;
        const BSHR_BS6 = 0x00000040;
        const BSHR_BS7 = 0x00000080;
        const BSHR_BS8 = 0x00000100;
        const BSHR_BS9 = 0x00000200;
        const BSHR_BS10 = 0x00000400;
        const BSHR_BS11 = 0x00000800;
        const BSHR_BS12 = 0x00001000;
        const BSHR_BS13 = 0x00002000;
        const BSHR_BS14 = 0x00004000;
        const BSHR_BS15 = 0x00008000;

        const BSHR_BR0 = 0x00010000;
        const BSHR_BR1 = 0x00020000;
        const BSHR_BR2 = 0x00040000;
        const BSHR_BR3 = 0x00080000;
        const BSHR_BR4 = 0x00100000;
        const BSHR_BR5 = 0x00200000;
        const BSHR_BR6 = 0x00400000;
        const BSHR_BR7 = 0x00800000;
        const BSHR_BR8 = 0x01000000;
        const BSHR_BR9 = 0x02000000;
        const BSHR_BR10 = 0x04000000;
        const BSHR_BR11 = 0x08000000;
        const BSHR_BR12 = 0x10000000;
        const BSHR_BR13 = 0x20000000;
        const BSHR_BR14 = 0x40000000;
        const BSHR_BR15 = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BCR: u32 {
        const BR0 = 0x0001;
        const BR1 = 0x0002;
        const BR2 = 0x0004;
        const BR3 = 0x0008;
        const BR4 = 0x0010;
        const BR5 = 0x0020;
        const BR6 = 0x0040;
        const BR7 = 0x0080;
        const BR8 = 0x0100;
        const BR9 = 0x0200;
        const BR10 = 0x0400;
        const BR11 = 0x0800;
        const BR12 = 0x1000;
        const BR13 = 0x2000;
        const BR14 = 0x4000;
        const BR15 = 0x8000;

        const BCR_BR0 = 0x0001;
        const BCR_BR1 = 0x0002;
        const BCR_BR2 = 0x0004;
        const BCR_BR3 = 0x0008;
        const BCR_BR4 = 0x0010;
        const BCR_BR5 = 0x0020;
        const BCR_BR6 = 0x0040;
        const BCR_BR7 = 0x0080;
        const BCR_BR8 = 0x0100;
        const BCR_BR9 = 0x0200;
        const BCR_BR10 = 0x0400;
        const BCR_BR11 = 0x0800;
        const BCR_BR12 = 0x1000;
        const BCR_BR13 = 0x2000;
        const BCR_BR14 = 0x4000;
        const BCR_BR15 = 0x8000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LCKR: u32 {
        const LCK0 = 0x00000001;
        const LCK1 = 0x00000002;
        const LCK2 = 0x00000004;
        const LCK3 = 0x00000008;
        const LCK4 = 0x00000010;
        const LCK5 = 0x00000020;
        const LCK6 = 0x00000040;
        const LCK7 = 0x00000080;
        const LCK8 = 0x00000100;
        const LCK9 = 0x00000200;
        const LCK10 = 0x00000400;
        const LCK11 = 0x00000800;
        const LCK12 = 0x00001000;
        const LCK13 = 0x00002000;
        const LCK14 = 0x00004000;
        const LCK15 = 0x00008000;
        const LCKK = 0x00010000;
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
        const SW_CFG_MASK = 0x7 << 24;
        const USBPD_CC_HVT = 1 << 20;
        const VDD33_IO_HSLV = 1 << 18;
        const VDDIO_IO_HSLV = 1 << 17;
        const VIO18_IO_HSLV = 1 << 16;
        const TIM2ITR1_RM = 1 << 12;
        const SDMMC_RM = 0x3 << 10;
        const UHSIF_PORT_RM = 0x3 << 8;
        const UHSIF_CLK_RM = 0x3 << 6;
        const ADC2_ETRGINJ_RM = 1 << 4;
        const ADC2_ETRGREG_RM = 1 << 3;
        const ADC1_ETRGINJ_RM = 1 << 2;
        const ADC1_ETRGREG_RM = 1 << 1;
        const PD0PD1_RM = 1 << 0;

        const PCFR1_PD0_1_REMAP = 0x00000001;
        const PCFR1_ADC1_ETRGINJ_REMAP = 0x00000002;
        const PCFR1_ADC1_ETRGREG_REMAP = 0x00000004;
        const PCFR1_ADC2_ETRGINJ_REMAP = 0x00000008;
        const PCFR1_ADC2_ETRGREG_REMAP = 0x00000010;

        const PCFR1_UHSIF_CLK_REMAP = 0x000000C0;
        const PCFR1_UHSIF_CLK_REMAP_0 = 0x00000040;
        const PCFR1_UHSIF_CLK_REMAP_1 = 0x00000080;

        const PCFR1_UHSIF_PORT_REMAP = 0x00000300;
        const PCFR1_UHSIF_PORT_REMAP_0 = 0x00000100;
        const PCFR1_UHSIF_PORT_REMAP_1 = 0x00000200;

        const PCFR1_SDMMC_REMAP = 0x00000C00;
        const PCFR1_SDMMC_REMAP_0 = 0x00000400;
        const PCFR1_SDMMC_REMAP_1 = 0x00000800;

        const PCFR1_TIM2ITR1_REMAP = 0x00001000;

        const PCFR1_VIO18_IO_HSLV = 0x00010000;
        const PCFR1_VIO33_IO_HSLV = 0x00020000;
        const PCFR1_VDD33_IO_HSLV = 0x00040000;

        const PCFR1_USBPD_CC_HVT = 0x00100000;

        const PCFR1_SWJ_CFG = 0x07000000;
        const PCFR1_SWJ_CFG_0 = 0x01000000;
        const PCFR1_SWJ_CFG_1 = 0x02000000;
        const PCFR1_SWJ_CFG_2 = 0x04000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AFLR: u32 {
        const AFR0 = 0x0000000F;
        const AFR0_0 = 0x00000001;
        const AFR0_1 = 0x00000002;
        const AFR0_2 = 0x00000004;
        const AFR0_3 = 0x00000008;

        const AFR1 = 0x000000F0;
        const AFR1_0 = 0x00000010;
        const AFR1_1 = 0x00000020;
        const AFR1_2 = 0x00000040;
        const AFR1_3 = 0x00000080;

        const AFR2 = 0x00000F00;
        const AFR2_0 = 0x00000100;
        const AFR2_1 = 0x00000200;
        const AFR2_2 = 0x00000400;
        const AFR2_3 = 0x00000800;

        const AFR3 = 0x0000F000;
        const AFR3_0 = 0x00001000;
        const AFR3_1 = 0x00002000;
        const AFR3_2 = 0x00004000;
        const AFR3_3 = 0x00008000;

        const AFR4 = 0x000F0000;
        const AFR4_0 = 0x00010000;
        const AFR4_1 = 0x00020000;
        const AFR4_2 = 0x00040000;
        const AFR4_3 = 0x00080000;

        const AFR5 = 0x00F00000;
        const AFR5_0 = 0x00100000;
        const AFR5_1 = 0x00200000;
        const AFR5_2 = 0x00400000;
        const AFR5_3 = 0x00800000;

        const AFR6 = 0x0F000000;
        const AFR6_0 = 0x01000000;
        const AFR6_1 = 0x02000000;
        const AFR6_2 = 0x04000000;
        const AFR6_3 = 0x08000000;

        const AFR7 = 0xF0000000;
        const AFR7_0 = 0x10000000;
        const AFR7_1 = 0x20000000;
        const AFR7_2 = 0x40000000;
        const AFR7_3 = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AFHR: u32 {
        const AFR8 = 0x0000000F;
        const AFR8_0 = 0x00000001;
        const AFR8_1 = 0x00000002;
        const AFR8_2 = 0x00000004;
        const AFR8_3 = 0x00000008;

        const AFR9 = 0x000000F0;
        const AFR9_0 = 0x00000010;
        const AFR9_1 = 0x00000020;
        const AFR9_2 = 0x00000040;
        const AFR9_3 = 0x00000080;

        const AFR10 = 0x00000F00;
        const AFR10_0 = 0x00000100;
        const AFR10_1 = 0x00000200;
        const AFR10_2 = 0x00000400;
        const AFR10_3 = 0x00000800;

        const AFR11 = 0x0000F000;
        const AFR11_0 = 0x00001000;
        const AFR11_1 = 0x00002000;
        const AFR11_2 = 0x00004000;
        const AFR11_3 = 0x00008000;

        const AFR12 = 0x000F0000;
        const AFR12_0 = 0x00010000;
        const AFR12_1 = 0x00020000;
        const AFR12_2 = 0x00040000;
        const AFR12_3 = 0x00080000;

        const AFR13 = 0x00F00000;
        const AFR13_0 = 0x00100000;
        const AFR13_1 = 0x00200000;
        const AFR13_2 = 0x00400000;
        const AFR13_3 = 0x00800000;

        const AFR14 = 0x0F000000;
        const AFR14_0 = 0x01000000;
        const AFR14_1 = 0x02000000;
        const AFR14_2 = 0x04000000;
        const AFR14_3 = 0x08000000;

        const AFR15 = 0xF0000000;
        const AFR15_0 = 0x10000000;
        const AFR15_1 = 0x20000000;
        const AFR15_2 = 0x40000000;
        const AFR15_3 = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EXTICR1: u32 {
        const EXTI0 = 0x0000000F;
        const EXTI1 = 0x000000F0;
        const EXTI2 = 0x00000F00;
        const EXTI3 = 0x0000F000;
        const EXTI4 = 0x000F0000;
        const EXTI5 = 0x00F00000;
        const EXTI6 = 0x0F000000;
        const EXTI7 = 0xF0000000;

        const EXTI0_PA = 0x00000000;
        const EXTI0_PB = 0x00000001;
        const EXTI0_PC = 0x00000002;
        const EXTI0_PD = 0x00000003;
        const EXTI0_PE = 0x00000004;
        const EXTI0_PF = 0x00000005;
        const EXTI0_CMPOUT = 0x00000006;

        const EXTI1_PA = 0x00000000;
        const EXTI1_PB = 0x00000010;
        const EXTI1_PC = 0x00000020;
        const EXTI1_PD = 0x00000030;
        const EXTI1_PE = 0x00000040;
        const EXTI1_PF = 0x00000050;
        const EXTI1_CMPOUT = 0x00000060;

        const EXTI2_PA = 0x00000000;
        const EXTI2_PB = 0x00000100;
        const EXTI2_PC = 0x00000200;
        const EXTI2_PD = 0x00000300;
        const EXTI2_PE = 0x00000400;
        const EXTI2_PF = 0x00000500;
        const EXTI2_CMPOUT = 0x00000600;

        const EXTI3_PA = 0x00000000;
        const EXTI3_PB = 0x00001000;
        const EXTI3_PC = 0x00002000;
        const EXTI3_PD = 0x00003000;
        const EXTI3_PE = 0x00004000;
        const EXTI3_PF = 0x00005000;
        const EXTI3_CMPOUT = 0x00006000;

        const EXTI4_PA = 0x00000000;
        const EXTI4_PB = 0x00010000;
        const EXTI4_PC = 0x00020000;
        const EXTI4_PD = 0x00030000;
        const EXTI4_PE = 0x00040000;
        const EXTI4_PF = 0x00050000;
        const EXTI4_CMPOUT = 0x00060000;

        const EXTI5_PA = 0x00000000;
        const EXTI5_PB = 0x00100000;
        const EXTI5_PC = 0x00200000;
        const EXTI5_PD = 0x00300000;
        const EXTI5_PE = 0x00400000;
        const EXTI5_PF = 0x00500000;
        const EXTI5_CMPOUT = 0x00600000;

        const EXTI6_PA = 0x00000000;
        const EXTI6_PB = 0x01000000;
        const EXTI6_PC = 0x02000000;
        const EXTI6_PD = 0x03000000;
        const EXTI6_PE = 0x04000000;
        const EXTI6_PF = 0x05000000;
        const EXTI6_CMPOUT = 0x06000000;

        const EXTI7_PA = 0x00000000;
        const EXTI7_PB = 0x10000000;
        const EXTI7_PC = 0x20000000;
        const EXTI7_PD = 0x30000000;
        const EXTI7_PE = 0x40000000;
        const EXTI7_PF = 0x50000000;
        const EXTI7_CMPOUT = 0x60000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EXTICR2: u32 {
        const EXTI8 = 0x0000000F;
        const EXTI9 = 0x000000F0;
        const EXTI10 = 0x00000F00;
        const EXTI11 = 0x0000F000;
        const EXTI12 = 0x000F0000;
        const EXTI13 = 0x00F00000;
        const EXTI14 = 0x0F000000;
        const EXTI15 = 0xF0000000;

        const EXTI8_PA = 0x00000000;
        const EXTI8_PB = 0x00000001;
        const EXTI8_PC = 0x00000002;
        const EXTI8_PD = 0x00000003;
        const EXTI8_PE = 0x00000004;
        const EXTI8_PF = 0x00000005;
        const EXTI8_CMPOUT = 0x00000006;

        const EXTI9_PA = 0x00000000;
        const EXTI9_PB = 0x00000010;
        const EXTI9_PC = 0x00000020;
        const EXTI9_PD = 0x00000030;
        const EXTI9_PE = 0x00000040;
        const EXTI9_PF = 0x00000050;
        const EXTI9_CMPOUT = 0x00000060;

        const EXTI10_PA = 0x00000000;
        const EXTI10_PB = 0x00000100;
        const EXTI10_PC = 0x00000200;
        const EXTI10_PD = 0x00000300;
        const EXTI10_PE = 0x00000400;
        const EXTI10_PF = 0x00000500;
        const EXTI10_CMPOUT = 0x00000600;

        const EXTI11_PA = 0x00000000;
        const EXTI11_PB = 0x00001000;
        const EXTI11_PC = 0x00002000;
        const EXTI11_PD = 0x00003000;
        const EXTI11_PE = 0x00004000;
        const EXTI11_PF = 0x00005000;
        const EXTI11_CMPOUT = 0x00006000;

        const EXTI12_PA = 0x00000000;
        const EXTI12_PB = 0x00010000;
        const EXTI12_PC = 0x00020000;
        const EXTI12_PD = 0x00030000;
        const EXTI12_PE = 0x00040000;
        const EXTI12_PF = 0x00050000;
        const EXTI12_CMPOUT = 0x00060000;

        const EXTI13_PA = 0x00000000;
        const EXTI13_PB = 0x00100000;
        const EXTI13_PC = 0x00200000;
        const EXTI13_PD = 0x00300000;
        const EXTI13_PE = 0x00400000;
        const EXTI13_PF = 0x00500000;
        const EXTI13_CMPOUT = 0x00600000;

        const EXTI14_PA = 0x00000000;
        const EXTI14_PB = 0x01000000;
        const EXTI14_PC = 0x02000000;
        const EXTI14_PD = 0x03000000;
        const EXTI14_PE = 0x04000000;
        const EXTI14_PF = 0x05000000;
        const EXTI14_CMPOUT = 0x06000000;

        const EXTI15_PA = 0x00000000;
        const EXTI15_PB = 0x10000000;
        const EXTI15_PC = 0x20000000;
        const EXTI15_PD = 0x30000000;
        const EXTI15_PE = 0x40000000;
        const EXTI15_PF = 0x50000000;
        const EXTI15_CMPOUT = 0x60000000;
    }
}

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
    pub struct CFGR: u32 {}
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
    }
}

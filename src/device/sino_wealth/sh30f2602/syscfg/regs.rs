use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// SYSCFG register block.
#[derive(Debug)]
#[repr(C)]
pub struct SYSCFG {
    pub PWRCR: Reg<PWRCR>,
    pub PWRSR: Reg<PWRSR>,
    pub SAFR: Reg<SAFR>,
    /// Reserved (0xc..=0xf).
    RESERVED0: [Reg32; 1],
    pub DBGCR: Reg<DBGCR>,
    pub GPIOBCR: Reg<GPIOBCR>,
    /// Reserved (0x18..=0x7f).
    RESERVED1: [Reg32; 26],
    pub HLDOCR: Reg<HLDOCR>,
}

bitflags! {
    /// `PWRCR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWRCR: u32 {
        const LVREN = 1 << 10;

        const LVRLV_MASK = 0x3 << 8;

        const BODEN = 1 << 7;

        const BODIE = 1 << 6;

        const BODMD_MASK = 0x3 << 4;

        const VBOD_MASK = 0xf << 0;
    }
}

bitflags! {
    /// `PWRSR` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWRSR: u32 {
        const BODF = 1 << 1;

        const BODIF = 1 << 0;
    }
}

bitflags! {
    /// `SAFR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SAFR: u32 {
        const LOCK_MASK = 0xffff << 16;

        const IEN_CSM = 1 << 7;

        const IEN_BOD = 1 << 6;

        const IEN_EXTI0 = 1 << 5;

        const SWJCFG = 1 << 2;
    }
}

bitflags! {
    /// `DBGCR` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DBGCR: u32 {
        const LOCK_MASK = 0xffff << 16;

        const DBG_SPI = 1 << 14;

        const DBG_UART = 1 << 13;

        const DBG_MCM = 1 << 12;

        const DBG_TIM = 1 << 11;

        const DBG_PCA = 1 << 10;

        const DBG_WWDT = 1 << 9;

        const DBG_IWDT = 1 << 8;

        const DBG_PWM = 1 << 7;

        const DBG_STOP = 1 << 1;
    }
}

bitflags! {
    /// `GPIOBCR` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GPIOBCR: u32 {
        const LOCK_MASK = 0xffff << 16;

        const BUS = 1 << 0;
    }
}

bitflags! {
    /// `HLDOCR` (0x80).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HLDOCR: u32 {
        const LOCK_MASK = 0xffff << 16;

        const IDCEN = 1 << 2;

        const IDC_MASK = 0x3 << 0;
    }
}

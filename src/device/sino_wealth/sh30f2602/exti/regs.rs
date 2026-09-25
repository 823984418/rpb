use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// EXTI register block.
#[derive(Debug)]
#[repr(C)]
pub struct EXTI {
    pub IMR: Reg<IMR>,
    pub EMR: Reg<EMR>,
    pub TMSR: Reg<TMSR>,
    pub RTSR: Reg<RTSR>,
    pub FTSR: Reg<FTSR>,
    pub SWIER: Reg<SWIER>,
    pub PR: Reg<PR>,
    pub CFG: Reg<CFG>,
    pub CFGH: Reg32,
    pub SAMP: Reg<SAMP>,
}

bitflags! {
    /// `IMR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IMR: u32 {
        const IMR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `EMR` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EMR: u32 {
        const EMR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `TMSR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TMSR: u32 {
        const TMR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `RTSR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTSR: u32 {
        const RTR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `FTSR` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FTSR: u32 {
        const FTR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `SWIER` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SWIER: u32 {
        const SWIER_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `PR` (0x18).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PR: u32 {
        const PRC_MASK = 0xff << 16;

        const PR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `CFG` (0x1c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFG: u32 {
        const EXTI7_EN = 1 << 7;

        const EXTI6_EN = 1 << 6;

        const EXTI5_EN = 1 << 5;

        const EXTI4_EN = 1 << 4;

        const EXTI3_EN = 1 << 3;

        const EXTI2_EN = 1 << 2;

        const EXTI1_EN = 1 << 1;

        const EXTI0_EN = 1 << 0;
    }
}

bitflags! {
    /// `SAMP` (0x24).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SAMP: u32 {
        const PS7_MASK = 0x3 << 30;

        const SN7_MASK = 0x3 << 28;

        const PS6_MASK = 0x3 << 26;

        const SN6_MASK = 0x3 << 24;

        const PS5_MASK = 0x3 << 22;

        const SN5_MASK = 0x3 << 20;

        const PS4_MASK = 0x3 << 18;

        const SN4_MASK = 0x3 << 16;

        const PS3_MASK = 0x3 << 14;

        const SN3_MASK = 0x3 << 12;

        const PS2_MASK = 0x3 << 10;

        const SN2_MASK = 0x3 << 8;

        const PS1_MASK = 0x3 << 6;

        const SN1_MASK = 0x3 << 4;

        const PS0_MASK = 0x3 << 2;

        const SN0_MASK = 0x3 << 0;
    }
}

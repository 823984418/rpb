use bitflags::bitflags;

use crate::register::Reg;

/// TIM register block.
#[derive(Debug)]
#[repr(C)]
pub struct TIM {
    pub CR: Reg<CR>,
    pub TCNT: Reg<TCNT>,
    pub TPR: Reg<TPR>,
    pub PSQ: Reg<PSQ>,
    pub TIMINTF: Reg<TIMINTF>,
}

bitflags! {
    /// `CR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const CASCEN = 1 << 15;

        const ECF_MASK = 0x3 << 12;

        const TC = 1 << 10;

        const ETEN = 1 << 9;

        const TRIGEN = 1 << 8;

        const IE = 1 << 7;

        const CLKS_MASK = 0x3 << 4;

        const OPM = 1 << 3;

        const STR = 1 << 0;
    }
}

bitflags! {
    /// `TCNT` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TCNT: u32 {
        const TCNTH_MASK = 0xffff << 16;

        const TCNTL_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `TPR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TPR: u32 {
        const TPRH_MASK = 0xffff << 16;

        const TPRL_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PSQ` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PSQ: u32 {
        const PSQH_MASK = 0xffff << 16;

        const PSQL_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `TIMINTF` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TIMINTF: u32 {
        const TFC = 1 << 16;

        const TF = 1 << 0;
    }
}

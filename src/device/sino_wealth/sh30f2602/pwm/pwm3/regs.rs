use bitflags::bitflags;

use crate::register::Reg;

/// PWM3 register block.
#[derive(Debug)]
#[repr(C)]
pub struct PWM3 {
    pub CR: Reg<CR>,
    pub PWMLOCK: Reg<PWMLOCK>,
    pub PWMPR: Reg<PWMPR>,
    pub PWMDR: Reg<PWMDR>,
    pub PWMDTR: Reg<PWMDTR>,
    pub PWMINTF: Reg<PWMINTF>,
    pub PWM_ADTR: Reg<PWM_ADTR>,
}

bitflags! {
    /// `CR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const FLTDEB_MASK = 0xf << 28;

        const FOUTA_MASK = 0x3 << 26;

        const FOUTB_MASK = 0x3 << 24;

        const FLT2IE = 1 << 22;

        const FLT1IE = 1 << 21;

        const FLT0IE = 1 << 20;

        const FLT2EN = 1 << 18;

        const FLT1EN = 1 << 17;

        const FLT0EN = 1 << 16;

        const FLT2S = 1 << 14;

        const FLT1S = 1 << 13;

        const FLT0S = 1 << 12;

        const ADTEN = 1 << 10;

        const PWMIE = 1 << 9;

        const EFLTIE = 1 << 8;

        const PWMSA = 1 << 7;

        const PWMSB = 1 << 6;

        const EFLT = 1 << 5;

        const FLTC = 1 << 4;

        const TCK_MASK = 0x7 << 1;

        const PWMEN = 1 << 0;
    }
}

bitflags! {
    /// `PWMLOCK` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMLOCK: u32 {
        const PWMLO_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWMPR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMPR: u32 {
        const PP_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWMDR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMDR: u32 {
        const PD_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWMDTR` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMDTR: u32 {
        const DT_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWMINTF` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMINTF: u32 {
        const FLT2STATC = 1 << 20;

        const FLT1STATC = 1 << 19;

        const FLT0STATC = 1 << 18;

        const PWMIFC = 1 << 17;

        const FLTSC = 1 << 16;

        const FLT2STAT = 1 << 4;

        const FLT1STAT = 1 << 3;

        const FLT0STAT = 1 << 2;

        const PWMIF = 1 << 1;

        const FLTS = 1 << 0;
    }
}

bitflags! {
    /// `PWM_ADTR` (0x18).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWM_ADTR: u32 {
        const DATA_MASK = 0xffff << 0;
    }
}

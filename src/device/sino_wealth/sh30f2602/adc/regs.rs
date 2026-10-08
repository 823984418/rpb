use bitflags::bitflags;

use crate::register::{Reg, Reserved};

/// ADC register block.
#[derive(Debug)]
#[repr(C)]
pub struct ADC {
    pub ADCON1: Reg<ADCON1>,
    pub ADCON2: Reg<ADCON2>,
    pub ADCON3: Reg<ADCON3>,
    pub ADPCH: Reg<ADPCH>,
    pub ADDR: [Reg<ADDR>; 16],
    /// Reserved (0x50..=0x5b).
    RESERVED0: Reserved<[u32; 3]>,
    pub SEQCHSEL0: Reg<SEQCHSEL0>,
    pub SEQCHSEL1: Reg<SEQCHSEL1>,
    pub ADGAPON: Reg<ADGAPON>,
    pub ADINTF: Reg<ADINTF>,
}

bitflags! {
    /// `ADCON1` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADCON1: u32 {
        const ADIE2 = 1 << 31;

        const ADSTRS2_MASK = 0x7f << 24;

        const ADIE1 = 1 << 23;

        const ADSTRS1_MASK = 0x7f << 16;

        const ADIE = 1 << 15;

        const ADSTRS_MASK = 0x7f << 8;

        const ADON = 1 << 7;

        const ADCTU_MASK = 0x3 << 0;
    }
}

bitflags! {
    /// `ADCON2` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADCON2: u32 {
        const ADOUT = 1 << 28;

        const ADMAXCH2_MASK = 0xf << 24;

        const ADMAXCH1_MASK = 0xf << 20;

        const TGAP_MASK = 0x7 << 17;

        const TS_MASK = 0xf << 8;

        const ADMAXCH_MASK = 0xf << 4;

        const TADC_MASK = 0xf << 0;
    }
}

bitflags! {
    /// `ADCON3` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADCON3: u32 {
        const ADSOC = 1 << 0;
    }
}

bitflags! {
    /// `ADPCH` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADPCH: u32 {
        const ADPCH_MASK = 0xf << 0;
    }
}

bitflags! {
    /// `ADDRn`
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR: u32 {
        const ADDR_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    /// `SEQCHSEL0` (0x5c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SEQCHSEL0: u32 {
        const SEQCH7_MASK = 0xf << 28;

        const SEQCH6_MASK = 0xf << 24;

        const SEQCH5_MASK = 0xf << 20;

        const SEQCH4_MASK = 0xf << 16;

        const SEQCH3_MASK = 0xf << 12;

        const SEQCH2_MASK = 0xf << 8;

        const SEQCH1_MASK = 0xf << 4;

        const SEQCH0_MASK = 0xf << 0;
    }
}

bitflags! {
    /// `SEQCHSEL1` (0x60).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SEQCHSEL1: u32 {
        const SEQCH15_MASK = 0xf << 28;

        const SEQCH14_MASK = 0xf << 24;

        const SEQCH13_MASK = 0xf << 20;

        const SEQCH12_MASK = 0xf << 16;

        const SEQCH11_MASK = 0xf << 12;

        const SEQCH10_MASK = 0xf << 8;

        const SEQCH9_MASK = 0xf << 4;

        const SEQCH8_MASK = 0xf << 0;
    }
}

bitflags! {
    /// `ADGAPON` (0x64).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADGAPON: u32 {
        const GAPEN15 = 1 << 15;

        const GAPEN14 = 1 << 14;

        const GAPEN13 = 1 << 13;

        const GAPEN12 = 1 << 12;

        const GAPEN11 = 1 << 11;

        const GAPEN10 = 1 << 10;

        const GAPEN9 = 1 << 9;

        const GAPEN8 = 1 << 8;

        const GAPEN7 = 1 << 7;

        const GAPEN6 = 1 << 6;

        const GAPEN5 = 1 << 5;

        const GAPEN4 = 1 << 4;

        const GAPEN3 = 1 << 3;

        const GAPEN2 = 1 << 2;

        const GAPEN1 = 1 << 1;

        const GAPEN0 = 1 << 0;
    }
}

bitflags! {
    /// `ADINTF` (0x68).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADINTF: u32 {
        const ADIF2C = 1 << 20;

        const ADIF1C = 1 << 19;

        const ADIFC = 1 << 18;

        const ADIF2 = 1 << 4;

        const ADIF1 = 1 << 3;

        const ADIF = 1 << 2;
    }
}

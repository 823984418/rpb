use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// ADC register block.
#[derive(Debug)]
#[repr(C)]
pub struct ADC {
    pub ADCON1: Reg<ADCON1>,
    pub ADCON2: Reg<ADCON2>,
    pub ADCON3: Reg<ADCON3>,
    pub ADPCH: Reg<ADPCH>,
    pub ADDR0: Reg<ADDR0>,
    pub ADDR1: Reg<ADDR1>,
    pub ADDR2: Reg<ADDR2>,
    pub ADDR3: Reg<ADDR3>,
    pub ADDR4: Reg<ADDR4>,
    pub ADDR5: Reg<ADDR5>,
    pub ADDR6: Reg<ADDR6>,
    pub ADDR7: Reg<ADDR7>,
    pub ADDR8: Reg<ADDR8>,
    pub ADDR9: Reg<ADDR9>,
    pub ADDR10: Reg<ADDR10>,
    pub ADDR11: Reg<ADDR11>,
    pub ADDR12: Reg<ADDR12>,
    pub ADDR13: Reg<ADDR13>,
    pub ADDR14: Reg<ADDR14>,
    pub ADDR15: Reg<ADDR15>,
    /// Reserved (0x50..=0x5b).
    RESERVED0: [Reg32; 3],
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
    /// `ADDR0` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR0: u32 {
        const ADDR0_MASK = 0xffff << 0;
    }
}

impl ADDR0 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR1` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR1: u32 {
        const ADDR1_MASK = 0xffff << 0;
    }
}

impl ADDR1 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR2` (0x18).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR2: u32 {
        const ADDR2_MASK = 0xffff << 0;
    }
}

impl ADDR2 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR3` (0x1c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR3: u32 {
        const ADDR3_MASK = 0xffff << 0;
    }
}

impl ADDR3 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR4` (0x20).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR4: u32 {
        const ADDR4_MASK = 0xffff << 0;
    }
}

impl ADDR4 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR5` (0x24).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR5: u32 {
        const ADDR5_MASK = 0xffff << 0;
    }
}

impl ADDR5 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR6` (0x28).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR6: u32 {
        const ADDR6_MASK = 0xffff << 0;
    }
}

impl ADDR6 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR7` (0x2c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR7: u32 {
        const ADDR7_MASK = 0xffff << 0;
    }
}

impl ADDR7 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR8` (0x30).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR8: u32 {
        const ADDR8_MASK = 0xffff << 0;
    }
}

impl ADDR8 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR9` (0x34).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR9: u32 {
        const ADDR9_MASK = 0xffff << 0;
    }
}

impl ADDR9 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR10` (0x38).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR10: u32 {
        const ADDR10_MASK = 0xffff << 0;
    }
}

impl ADDR10 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR11` (0x3c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR11: u32 {
        const ADDR11_MASK = 0xffff << 0;
    }
}

impl ADDR11 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR12` (0x40).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR12: u32 {
        const ADDR12_MASK = 0xffff << 0;
    }
}

impl ADDR12 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR13` (0x44).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR13: u32 {
        const ADDR13_MASK = 0xffff << 0;
    }
}

impl ADDR13 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR14` (0x48).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR14: u32 {
        const ADDR14_MASK = 0xffff << 0;
    }
}

impl ADDR14 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `ADDR15` (0x4c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR15: u32 {
        const ADDR15_MASK = 0xffff << 0;
    }
}

impl ADDR15 {
    pub fn ADDR(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
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

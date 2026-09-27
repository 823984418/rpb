use bitflags::bitflags;

use crate::register::{Reg, Reserved};

/// MCM register block.
#[derive(Debug)]
#[repr(C)]
pub struct MCM {
    pub PWMOE: Reg<PWMOE>,
    pub PWMCON1: Reg<PWMCON1>,
    pub PWMCON2: Reg<PWMCON2>,
    pub PWMP: Reg<PWMP>,
    pub PWMC: Reg<PWMC>,
    pub PWMPSQ: Reg<PWMPSQ>,
    pub PWM0D: Reg<PWM0D>,
    pub PWM1D: Reg<PWM1D>,
    pub PWM2D: Reg<PWM2D>,
    pub PWM01D: Reg<PWM01D>,
    pub PWM11D: Reg<PWM11D>,
    pub PWM21D: Reg<PWM21D>,
    pub PWMCMP1: Reg<PWMCMP1>,
    pub PWMCMP2: Reg<PWMCMP2>,
    pub PWMCMP3: Reg<PWMCMP3>,
    pub PWMCMP4: Reg<PWMCMP4>,
    pub PWMDT00: Reg<PWMDT00>,
    pub PWMDT01: Reg<PWMDT01>,
    pub PWMDT10: Reg<PWMDT10>,
    pub PWMDT11: Reg<PWMDT11>,
    pub PWMDT20: Reg<PWMDT20>,
    pub PWMDT21: Reg<PWMDT21>,
    pub PMANUALCON1: Reg<PMANUALCON1>,
    pub PMANUALCON2: Reg<PMANUALCON2>,
    pub FLTCON: Reg<FLTCON>,
    /// Reserved (0x64..=0x6f).
    RESERVED0: Reserved<[u32; 3]>,
    pub POSTDCR: Reg<POSTDCR>,
    pub PWMINTEN: Reg<PWMINTEN>,
    pub PWMINTF: Reg<PWMINTF>,
    pub PWMRLDEN0: Reg<PWMRLDEN0>,
    pub PWMRLDEN1: Reg<PWMRLDEN1>,
    pub FLTWEN: Reg<FLTWEN>,
    pub PWMREMAP: Reg<PWMREMAP>,
}

bitflags! {
    /// `PWMOE` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMOE: u32 {
        const PWMOE = 1 << 0;
    }
}

bitflags! {
    /// `PWMCON1` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMCON1: u32 {
        const POUTMOD = 1 << 15;

        const POSTPS_MASK = 0x7 << 12;

        const PTMOD_MASK = 0x3 << 10;

        const PWMSYM = 1 << 9;

        const PDCON2 = 1 << 8;

        const PDCON1 = 1 << 7;

        const PDCON0 = 1 << 6;

        const PWM21S = 1 << 5;

        const PWM11S = 1 << 4;

        const PWM01S = 1 << 3;

        const PWM2S = 1 << 2;

        const PWM1S = 1 << 1;

        const PWM0S = 1 << 0;
    }
}

bitflags! {
    /// `PWMCON2` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMCON2: u32 {
        const PCMLDEN = 1 << 14;

        const ZCMLDEN = 1 << 13;

        const PDLDEN = 1 << 12;

        const ZDLDEN = 1 << 11;

        const CILDEN = 1 << 10;

        const DILDEN = 1 << 9;

        const OSYNC = 1 << 8;

        const CMP4_MASK = 0x3 << 6;

        const CMP3_MASK = 0x3 << 4;

        const CMP2_MASK = 0x3 << 2;

        const CMP1_MASK = 0x3 << 0;
    }
}

bitflags! {
    /// `PWMP` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMP: u32 {
        const PWMP_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWMC` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMC: u32 {
        const PWMC_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWMPSQ` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMPSQ: u32 {
        const PWMPSQ_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWM0D` (0x18).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWM0D: u32 {
        const PWM0D_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWM1D` (0x1c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWM1D: u32 {
        const PWM1D_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWM2D` (0x20).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWM2D: u32 {
        const PWM2D_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWM01D` (0x24).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWM01D: u32 {
        const PWM01D_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWM11D` (0x28).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWM11D: u32 {
        const PWM11D_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWM21D` (0x2c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWM21D: u32 {
        const PWM21D_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWMCMP1` (0x30).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMCMP1: u32 {
        const PWMCMP1_MASK = 0xffff << 0;
    }
}

impl PWMCMP1 {
    pub fn PWMCMP(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMCMP2` (0x34).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMCMP2: u32 {
        const PWMCMP2_MASK = 0xffff << 0;
    }
}

impl PWMCMP2 {
    pub fn PWMCMP(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMCMP3` (0x38).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMCMP3: u32 {
        const PWMCMP3_MASK = 0xffff << 0;
    }
}

impl PWMCMP3 {
    pub fn PWMCMP(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMCMP4` (0x3c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMCMP4: u32 {
        const PWMCMP4_MASK = 0xffff << 0;
    }
}

impl PWMCMP4 {
    pub fn PWMCMP(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMDT00` (0x40).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMDT00: u32 {
        const PWMDT00_MASK = 0xffff << 0;
    }
}

impl PWMDT00 {
    pub fn PWMDT(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMDT01` (0x44).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMDT01: u32 {
        const PWMDT01_MASK = 0xffff << 0;
    }
}

impl PWMDT01 {
    pub fn PWMDT(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMDT10` (0x48).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMDT10: u32 {
        const PWMDT10_MASK = 0xffff << 0;
    }
}

impl PWMDT10 {
    pub fn PWMDT(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMDT11` (0x4c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMDT11: u32 {
        const PWMDT11_MASK = 0xffff << 0;
    }
}

impl PWMDT11 {
    pub fn PWMDT(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMDT20` (0x50).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMDT20: u32 {
        const PWMDT20_MASK = 0xffff << 0;
    }
}

impl PWMDT20 {
    pub fn PWMDT(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PWMDT21` (0x54).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMDT21: u32 {
        const PWMDT21_MASK = 0xffff << 0;
    }
}

impl PWMDT21 {
    pub fn PWMDT(n: u32) -> Self {
        Self::from_bits_retain((n & 0xffff) << 0)
    }
}

bitflags! {
    /// `PMANUALCON1` (0x58).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PMANUALCON1: u32 {
        const PMANUAL21 = 1 << 5;

        const PMANUAL11 = 1 << 4;

        const PMANUAL01 = 1 << 3;

        const PMANUAL2 = 1 << 2;

        const PMANUAL1 = 1 << 1;

        const PMANUAL0 = 1 << 0;
    }
}

bitflags! {
    /// `PMANUALCON2` (0x5c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PMANUALCON2: u32 {
        const POUT21 = 1 << 5;

        const POUT11 = 1 << 4;

        const POUT01 = 1 << 3;

        const POUT2 = 1 << 2;

        const POUT1 = 1 << 1;

        const POUT0 = 1 << 0;
    }
}

bitflags! {
    /// `FLTCON` (0x60).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FLTCON: u32 {
        const FLT3STATC = 1 << 28;

        const FLT3EN = 1 << 27;

        const FLT3S = 1 << 26;

        const FLT3M = 1 << 25;

        const FLT3STAT = 1 << 24;

        const FOUT1_MASK = 0x3 << 22;

        const FOUT0_MASK = 0x3 << 20;

        const FLT0STATC = 1 << 18;

        const FLT1STATC = 1 << 17;

        const FLT2STATC = 1 << 16;

        const FLT0EN = 1 << 15;

        const FLT0S = 1 << 14;

        const FLT0M = 1 << 13;

        const FLT0STAT = 1 << 12;

        const FLT1EN = 1 << 11;

        const FLT1S = 1 << 10;

        const FLT1M = 1 << 9;

        const FLT1STAT = 1 << 8;

        const FLT2DEB_MASK = 0xf << 4;

        const FLT2EN = 1 << 3;

        const FLT2S = 1 << 2;

        const FLT2M = 1 << 1;

        const FLT2STAT = 1 << 0;
    }
}

bitflags! {
    /// `POSTDCR` (0x70).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct POSTDCR: u32 {
        const OSTDEN = 1 << 7;
    }
}

bitflags! {
    /// `PWMINTEN` (0x74).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMINTEN: u32 {
        const FLTCMP2IE = 1 << 13;

        const OSTDIE = 1 << 12;

        const FLTCMP1IE = 1 << 10;

        const FLTCMP0IE = 1 << 9;

        const FLTIE = 1 << 8;

        const PWMPIE = 1 << 7;

        const PWMZIE = 1 << 6;

        const PTDD2IE = 1 << 5;

        const PTUD2IE = 1 << 4;

        const PTDD1IE = 1 << 3;

        const PTUD1IE = 1 << 2;

        const PTDD0IE = 1 << 1;

        const PTUD0IE = 1 << 0;
    }
}

bitflags! {
    /// `PWMINTF` (0x78).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMINTF: u32 {
        const FLTCMP2IFC = 1 << 29;

        const OSTDFC = 1 << 28;

        const FLTCMP1IFC = 1 << 26;

        const FLTCMP0IFC = 1 << 25;

        const FLTIFC = 1 << 24;

        const PWMPIFC = 1 << 23;

        const PWMZIFC = 1 << 22;

        const PTDD2IFC = 1 << 21;

        const PTUD2IFC = 1 << 20;

        const PTDD1IFC = 1 << 19;

        const PTUD1IFC = 1 << 18;

        const PTDD0IFC = 1 << 17;

        const PTUD0IFC = 1 << 16;

        const FLTCMP2IF = 1 << 13;

        const OSTDF = 1 << 12;

        const FLTCMP1IF = 1 << 10;

        const FLTCMP0IF = 1 << 9;

        const FLTIF = 1 << 8;

        const PWMPIF = 1 << 7;

        const PWMZIF = 1 << 6;

        const PTDD2IF = 1 << 5;

        const PTUD2IF = 1 << 4;

        const PTDD1IF = 1 << 3;

        const PTUD1IF = 1 << 2;

        const PTDD0IF = 1 << 1;

        const PTUD0IF = 1 << 0;
    }
}

bitflags! {
    /// `PWMRLDEN0` (0x7c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMRLDEN0: u32 {
        const PWMRLDEN0_MASK = 0xff << 0;
    }
}

impl PWMRLDEN0 {
    pub fn PWMRLDEN(n: u32) -> Self {
        Self::from_bits_retain((n & 0xff) << 0)
    }
}

bitflags! {
    /// `PWMRLDEN1` (0x80).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMRLDEN1: u32 {
        const PWMRLDEN1_MASK = 0xff << 0;
    }
}

impl PWMRLDEN1 {
    pub fn PWMRLDEN(n: u32) -> Self {
        Self::from_bits_retain((n & 0xff) << 0)
    }
}

bitflags! {
    /// `FLTWEN` (0x84).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FLTWEN: u32 {
        const FLTWEN_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `PWMREMAP` (0x88).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PWMREMAP: u32 {
        const PWMIO_MASK = 0x7 << 0;
    }
}

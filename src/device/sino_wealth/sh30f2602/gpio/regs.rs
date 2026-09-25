use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// GPIOA / GPIOB / GPIOC register block.
#[derive(Debug)]
#[repr(C)]
pub struct GPIO {
    pub MODER: Reg<MODER>,
    pub IDR: Reg<IDR>,
    pub ODR: Reg<ODR>,
    pub BSRR: Reg<BSRR>,
    pub LCKR: Reg<LCKR>,
    pub IE: Reg<IE>,
    pub OTYPER: Reg32,
    pub ODRVR: Reg32,
    pub PUPDR: Reg<PUPDR>,
    pub TTLEN: Reg<TTLEN>,
    pub AFRL: Reg<AFRL>,
    pub AFRH: Reg<AFRH>,
}

bitflags! {
    /// `MODER` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MODER: u32 {
        const MODER15 = 1 << 15;

        const MODER14 = 1 << 14;

        const MODER13 = 1 << 13;

        const MODER12 = 1 << 12;

        const MODER11 = 1 << 11;

        const MODER10 = 1 << 10;

        const MODER9 = 1 << 9;

        const MODER8 = 1 << 8;

        const MODER7 = 1 << 7;

        const MODER6 = 1 << 6;

        const MODER5 = 1 << 5;

        const MODER4 = 1 << 4;

        const MODER3 = 1 << 3;

        const MODER2 = 1 << 2;

        const MODER1 = 1 << 1;

        const MODER0 = 1 << 0;
    }
}

bitflags! {
    /// `IDR` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDR: u32 {
        const IDR_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `ODR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ODR: u32 {
        const ODR_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `BSRR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BSRR: u32 {
        const BR_MASK = 0xffff << 16;

        const BS_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `LCKR` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LCKR: u32 {
        const LOCK_MASK = 0xffff << 16;

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

bitflags! {
    /// `IE` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IE: u32 {
        const IE15 = 1 << 15;

        const IE14 = 1 << 14;

        const IE13 = 1 << 13;

        const IE12 = 1 << 12;

        const IE11 = 1 << 11;

        const IE10 = 1 << 10;

        const IE9 = 1 << 9;

        const IE8 = 1 << 8;

        const IE7 = 1 << 7;

        const IE6 = 1 << 6;

        const IE5 = 1 << 5;

        const IE4 = 1 << 4;

        const IE3 = 1 << 3;

        const IE2 = 1 << 2;

        const IE1 = 1 << 1;

        const IE0 = 1 << 0;
    }
}

bitflags! {
    /// `PUPDR` (0x20).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PUPDR: u32 {
        const PHDR15 = 1 << 30;

        const PHDR14 = 1 << 28;

        const PHDR13 = 1 << 26;

        const PHDR12 = 1 << 24;

        const PHDR11 = 1 << 22;

        const PHDR10 = 1 << 20;

        const PHDR9 = 1 << 18;

        const PHDR8 = 1 << 16;

        const PHDR7 = 1 << 14;

        const PHDR6 = 1 << 12;

        const PHDR5 = 1 << 10;

        const PHDR4 = 1 << 8;

        const PHDR3 = 1 << 6;

        const PHDR2 = 1 << 4;

        const PHDR1 = 1 << 2;

        const PHDR0 = 1 << 0;
    }
}

bitflags! {
    /// `TTLEN` (0x24).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TTLEN: u32 {
        const TTLEN15 = 1 << 15;

        const TTLEN14 = 1 << 14;

        const TTLEN13 = 1 << 13;

        const TTLEN12 = 1 << 12;

        const TTLEN11 = 1 << 11;

        const TTLEN9 = 1 << 9;

        const TTLEN7 = 1 << 7;

        const TTLEN5 = 1 << 5;

        const TTLEN4 = 1 << 4;

        const TTLEN2 = 1 << 2;

        const TTLEN1 = 1 << 1;

        const TTLEN0 = 1 << 0;
    }
}

bitflags! {
    /// `AFRL` (0x28).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AFRL: u32 {
        const AFR7_MASK = 0x7 << 28;

        const AFR6_MASK = 0x7 << 24;

        const AFR5_MASK = 0x7 << 20;

        const AFR4_MASK = 0x7 << 16;

        const AFR3_MASK = 0x7 << 12;

        const AFR2_MASK = 0x7 << 8;

        const AFR1_MASK = 0x7 << 4;

        const AFR0_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `AFRH` (0x2c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AFRH: u32 {
        const AFR15_MASK = 0x7 << 28;

        const AFR14_MASK = 0x7 << 24;

        const AFR13_MASK = 0x7 << 20;

        const AFR12_MASK = 0x7 << 16;

        const AFR11_MASK = 0x7 << 12;

        const AFR10_MASK = 0x7 << 8;

        const AFR9_MASK = 0x7 << 4;

        const AFR8_MASK = 0x7 << 0;
    }
}

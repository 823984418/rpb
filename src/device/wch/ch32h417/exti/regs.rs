use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct EXTI {
    pub INTENR: Reg<INTENR>,
    pub EVENR: Reg<EVENR>,
    pub RTENR: Reg<RTENR>,
    pub FTENR: Reg<FTENR>,
    pub SWIEVR: Reg<SWIEVR>,
    pub INTFR: Reg<INTFR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTENR: u32 {
        const MR25 = 1 << 25;
        const MR24 = 1 << 24;
        const MR23 = 1 << 23;
        const MR22 = 1 << 22;
        const MR21 = 1 << 21;
        const MR20 = 1 << 20;
        const MR19 = 1 << 19;
        const MR18 = 1 << 18;
        const MR17 = 1 << 17;
        const MR16 = 1 << 16;
        const MR15 = 1 << 15;
        const MR14 = 1 << 14;
        const MR13 = 1 << 13;
        const MR12 = 1 << 12;
        const MR11 = 1 << 11;
        const MR10 = 1 << 10;
        const MR9 = 1 << 9;
        const MR8 = 1 << 8;
        const MR7 = 1 << 7;
        const MR6 = 1 << 6;
        const MR5 = 1 << 5;
        const MR4 = 1 << 4;
        const MR3 = 1 << 3;
        const MR2 = 1 << 2;
        const MR1 = 1 << 1;
        const MR0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EVENR: u32 {
        const MR25 = 1 << 25;
        const MR24 = 1 << 24;
        const MR23 = 1 << 23;
        const MR22 = 1 << 22;
        const MR21 = 1 << 21;
        const MR20 = 1 << 20;
        const MR19 = 1 << 19;
        const MR18 = 1 << 18;
        const MR17 = 1 << 17;
        const MR16 = 1 << 16;
        const MR15 = 1 << 15;
        const MR14 = 1 << 14;
        const MR13 = 1 << 13;
        const MR12 = 1 << 12;
        const MR11 = 1 << 11;
        const MR10 = 1 << 10;
        const MR9 = 1 << 9;
        const MR8 = 1 << 8;
        const MR7 = 1 << 7;
        const MR6 = 1 << 6;
        const MR5 = 1 << 5;
        const MR4 = 1 << 4;
        const MR3 = 1 << 3;
        const MR2 = 1 << 2;
        const MR1 = 1 << 1;
        const MR0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTENR: u32 {
        const TR25 = 1 << 25;
        const TR24 = 1 << 24;
        const TR23 = 1 << 23;
        const TR22 = 1 << 22;
        const TR21 = 1 << 21;
        const TR20 = 1 << 20;
        const TR19 = 1 << 19;
        const TR18 = 1 << 18;
        const TR17 = 1 << 17;
        const TR16 = 1 << 16;
        const TR15 = 1 << 15;
        const TR14 = 1 << 14;
        const TR13 = 1 << 13;
        const TR12 = 1 << 12;
        const TR11 = 1 << 11;
        const TR10 = 1 << 10;
        const TR9 = 1 << 9;
        const TR8 = 1 << 8;
        const TR7 = 1 << 7;
        const TR6 = 1 << 6;
        const TR5 = 1 << 5;
        const TR4 = 1 << 4;
        const TR3 = 1 << 3;
        const TR2 = 1 << 2;
        const TR1 = 1 << 1;
        const TR0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FTENR: u32 {
        const TR25 = 1 << 25;
        const TR24 = 1 << 24;
        const TR23 = 1 << 23;
        const TR22 = 1 << 22;
        const TR21 = 1 << 21;
        const TR20 = 1 << 20;
        const TR19 = 1 << 19;
        const TR18 = 1 << 18;
        const TR17 = 1 << 17;
        const TR16 = 1 << 16;
        const TR15 = 1 << 15;
        const TR14 = 1 << 14;
        const TR13 = 1 << 13;
        const TR12 = 1 << 12;
        const TR11 = 1 << 11;
        const TR10 = 1 << 10;
        const TR9 = 1 << 9;
        const TR8 = 1 << 8;
        const TR7 = 1 << 7;
        const TR6 = 1 << 6;
        const TR5 = 1 << 5;
        const TR4 = 1 << 4;
        const TR3 = 1 << 3;
        const TR2 = 1 << 2;
        const TR1 = 1 << 1;
        const TR0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SWIEVR: u32 {
        const SWIEVR25 = 1 << 25;
        const SWIEVR24 = 1 << 24;
        const SWIEVR23 = 1 << 23;
        const SWIEVR22 = 1 << 22;
        const SWIEVR21 = 1 << 21;
        const SWIEVR20 = 1 << 20;
        const SWIEVR19 = 1 << 19;
        const SWIEVR18 = 1 << 18;
        const SWIEVR17 = 1 << 17;
        const SWIEVR16 = 1 << 16;
        const SWIEVR15 = 1 << 15;
        const SWIEVR14 = 1 << 14;
        const SWIEVR13 = 1 << 13;
        const SWIEVR12 = 1 << 12;
        const SWIEVR11 = 1 << 11;
        const SWIEVR10 = 1 << 10;
        const SWIEVR9 = 1 << 9;
        const SWIEVR8 = 1 << 8;
        const SWIEVR7 = 1 << 7;
        const SWIEVR6 = 1 << 6;
        const SWIEVR5 = 1 << 5;
        const SWIEVR4 = 1 << 4;
        const SWIEVR3 = 1 << 3;
        const SWIEVR2 = 1 << 2;
        const SWIEVR1 = 1 << 1;
        const SWIEVR0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFR: u32 {
        const INTF25 = 1 << 25;
        const INTF24 = 1 << 24;
        const INTF23 = 1 << 23;
        const INTF22 = 1 << 22;
        const INTF21 = 1 << 21;
        const INTF20 = 1 << 20;
        const INTF19 = 1 << 19;
        const INTF18 = 1 << 18;
        const INTF17 = 1 << 17;
        const INTF16 = 1 << 16;
        const INTF15 = 1 << 15;
        const INTF14 = 1 << 14;
        const INTF13 = 1 << 13;
        const INTF12 = 1 << 12;
        const INTF11 = 1 << 11;
        const INTF10 = 1 << 10;
        const INTF9 = 1 << 9;
        const INTF8 = 1 << 8;
        const INTF7 = 1 << 7;
        const INTF6 = 1 << 6;
        const INTF5 = 1 << 5;
        const INTF4 = 1 << 4;
        const INTF3 = 1 << 3;
        const INTF2 = 1 << 2;
        const INTF1 = 1 << 1;
        const INTF0 = 1 << 0;
    }
}

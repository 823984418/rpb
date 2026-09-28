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
        const MR0 = 0x00000001;
        const MR1 = 0x00000002;
        const MR2 = 0x00000004;
        const MR3 = 0x00000008;
        const MR4 = 0x00000010;
        const MR5 = 0x00000020;
        const MR6 = 0x00000040;
        const MR7 = 0x00000080;
        const MR8 = 0x00000100;
        const MR9 = 0x00000200;
        const MR10 = 0x00000400;
        const MR11 = 0x00000800;
        const MR12 = 0x00001000;
        const MR13 = 0x00002000;
        const MR14 = 0x00004000;
        const MR15 = 0x00008000;
        const MR16 = 0x00010000;
        const MR17 = 0x00020000;
        const MR18 = 0x00040000;
        const MR19 = 0x00080000;
        const MR20 = 0x00100000;
        const MR21 = 0x00200000;
        const MR22 = 0x00400000;
        const MR23 = 0x00800000;
        const MR24 = 0x01000000;
        const MR25 = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EVENR: u32 {
        const MR0 = 0x00000001;
        const MR1 = 0x00000002;
        const MR2 = 0x00000004;
        const MR3 = 0x00000008;
        const MR4 = 0x00000010;
        const MR5 = 0x00000020;
        const MR6 = 0x00000040;
        const MR7 = 0x00000080;
        const MR8 = 0x00000100;
        const MR9 = 0x00000200;
        const MR10 = 0x00000400;
        const MR11 = 0x00000800;
        const MR12 = 0x00001000;
        const MR13 = 0x00002000;
        const MR14 = 0x00004000;
        const MR15 = 0x00008000;
        const MR16 = 0x00010000;
        const MR17 = 0x00020000;
        const MR18 = 0x00040000;
        const MR19 = 0x00080000;
        const MR20 = 0x00100000;
        const MR21 = 0x00200000;
        const MR22 = 0x00400000;
        const MR23 = 0x00800000;
        const MR24 = 0x01000000;
        const MR25 = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTENR: u32 {
        const TR0 = 0x00000001;
        const TR1 = 0x00000002;
        const TR2 = 0x00000004;
        const TR3 = 0x00000008;
        const TR4 = 0x00000010;
        const TR5 = 0x00000020;
        const TR6 = 0x00000040;
        const TR7 = 0x00000080;
        const TR8 = 0x00000100;
        const TR9 = 0x00000200;
        const TR10 = 0x00000400;
        const TR11 = 0x00000800;
        const TR12 = 0x00001000;
        const TR13 = 0x00002000;
        const TR14 = 0x00004000;
        const TR15 = 0x00008000;
        const TR16 = 0x00010000;
        const TR17 = 0x00020000;
        const TR18 = 0x00040000;
        const TR19 = 0x00080000;
        const TR20 = 0x00100000;
        const TR21 = 0x00200000;
        const TR22 = 0x00400000;
        const TR23 = 0x00800000;
        const TR24 = 0x01000000;
        const TR25 = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FTENR: u32 {
        const TR0 = 0x00000001;
        const TR1 = 0x00000002;
        const TR2 = 0x00000004;
        const TR3 = 0x00000008;
        const TR4 = 0x00000010;
        const TR5 = 0x00000020;
        const TR6 = 0x00000040;
        const TR7 = 0x00000080;
        const TR8 = 0x00000100;
        const TR9 = 0x00000200;
        const TR10 = 0x00000400;
        const TR11 = 0x00000800;
        const TR12 = 0x00001000;
        const TR13 = 0x00002000;
        const TR14 = 0x00004000;
        const TR15 = 0x00008000;
        const TR16 = 0x00010000;
        const TR17 = 0x00020000;
        const TR18 = 0x00040000;
        const TR19 = 0x00080000;
        const TR20 = 0x00100000;
        const TR21 = 0x00200000;
        const TR22 = 0x00400000;
        const TR23 = 0x00800000;
        const TR24 = 0x01000000;
        const TR25 = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SWIEVR: u32 {
        const SWIEVR0 = 0x00000001;
        const SWIEVR1 = 0x00000002;
        const SWIEVR2 = 0x00000004;
        const SWIEVR3 = 0x00000008;
        const SWIEVR4 = 0x00000010;
        const SWIEVR5 = 0x00000020;
        const SWIEVR6 = 0x00000040;
        const SWIEVR7 = 0x00000080;
        const SWIEVR8 = 0x00000100;
        const SWIEVR9 = 0x00000200;
        const SWIEVR10 = 0x00000400;
        const SWIEVR11 = 0x00000800;
        const SWIEVR12 = 0x00001000;
        const SWIEVR13 = 0x00002000;
        const SWIEVR14 = 0x00004000;
        const SWIEVR15 = 0x00008000;
        const SWIEVR16 = 0x00010000;
        const SWIEVR17 = 0x00020000;
        const SWIEVR18 = 0x00040000;
        const SWIEVR19 = 0x00080000;
        const SWIEVR20 = 0x00100000;
        const SWIEVR21 = 0x00200000;
        const SWIEVR22 = 0x00400000;
        const SWIEVR23 = 0x00800000;
        const SWIEVR24 = 0x01000000;
        const SWIEVR25 = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFR: u32 {
        const INTF0 = 0x00000001;
        const INTF1 = 0x00000002;
        const INTF2 = 0x00000004;
        const INTF3 = 0x00000008;
        const INTF4 = 0x00000010;
        const INTF5 = 0x00000020;
        const INTF6 = 0x00000040;
        const INTF7 = 0x00000080;
        const INTF8 = 0x00000100;
        const INTF9 = 0x00000200;
        const INTF10 = 0x00000400;
        const INTF11 = 0x00000800;
        const INTF12 = 0x00001000;
        const INTF13 = 0x00002000;
        const INTF14 = 0x00004000;
        const INTF15 = 0x00008000;
        const INTF16 = 0x00010000;
        const INTF17 = 0x00020000;
        const INTF18 = 0x00040000;
        const INTF19 = 0x00080000;
        const INTF20 = 0x00100000;
        const INTF21 = 0x00200000;
        const INTF22 = 0x00400000;
        const INTF23 = 0x00800000;
        const INTF24 = 0x01000000;
        const INTF25 = 0x02000000;
    }
}

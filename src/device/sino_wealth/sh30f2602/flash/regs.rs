use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

/// FLASH register block.
#[derive(Debug)]
#[repr(C)]
pub struct FLASH {
    pub ACR: Reg<ACR>,
    pub MKYR: Reg32,
    pub E2KYR: Reg32,
    pub SR: Reg<SR>,
    pub CR: Reg<CR>,
    pub CR1: Reg<CR1>,
    pub OPR: Reg<OPR>,
    pub RPR: Reg<RPR>,
    pub WRPR: Reg32,
    pub WRPR1: Reg32,
    pub CNTR: Reg32,
    pub UPCNTR: Reg32,
    pub CNTCR: Reg<CNTCR>,
    pub IKYR: Reg32,
    pub DATA0: Reg32,
    pub DATA1: Reg32,
    pub DATA2: Reg32,
    /// Reserved (0x44..=0xff).
    RESERVED0: Reserved<[u32; 47]>,
    pub MEMRMP: Reg<MEMRMP>,
    /// Reserved (0x104..=0x1ff).
    RESERVED1: Reserved<[u32; 63]>,
    pub OPR_CUST1: Reg<OPR_CUST1>,
    pub OPR_DESI0: Reg<OPR_DESI0>,
    pub OPR_DESI1: Reg<OPR_DESI1>,
    pub OPR_DESI2: Reg<OPR_DESI2>,
    pub OPR_DESI3: Reg<OPR_DESI3>,
}

bitflags! {
    /// `ACR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ACR: u32 {
        const LOCK_MASK = 0xffff << 16;

        const PRFTEN = 1 << 12;

        const CRST = 1 << 11;

        const DBEN = 1 << 10;

        const IBEN = 1 << 9;

        const CACHEN = 1 << 8;

        const LATENCY_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `SR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const STAERRC = 1 << 23;

        const PGWERRC = 1 << 22;

        const PGPERRC = 1 << 21;

        const WRPRTERRC = 1 << 20;

        const FLSERRC = 1 << 19;

        const OPERRC = 1 << 17;

        const EOPC = 1 << 16;

        const BSY = 1 << 15;

        const STAERR = 1 << 7;

        const PGWERR = 1 << 6;

        const PGPERR = 1 << 5;

        const WRPRTERR = 1 << 4;

        const FLSERR = 1 << 3;

        const OPERR = 1 << 1;

        const EOP = 1 << 0;
    }
}

bitflags! {
    /// `CR` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const CMD_MASK = 0xffff << 16;

        const MNLCK = 1 << 15;

        const E2LCK = 1 << 14;

        const INFLCK = 1 << 12;

        const PSIZE_MASK = 0x7 << 9;

        const STRT = 1 << 8;
    }
}

bitflags! {
    /// `CR1` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR1: u32 {
        const SNB_MASK = 0x3ffff << 0;
    }
}

bitflags! {
    /// `OPR` (0x18).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `RPR` (0x1c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RPR: u32 {
        const RDP_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `CNTCR` (0x30).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTCR: u32 {
        const CNTEN = 1 << 0;
    }
}

bitflags! {
    /// `MEMRMP` (0x100).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MEMRMP: u32 {
        const MEMMODE_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `OPR_CUST1` (0x200).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_CUST1: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `OPR_DESI0` (0x204).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_DESI0: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `OPR_DESI1` (0x208).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_DESI1: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `OPR_DESI2` (0x20c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_DESI2: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `OPR_DESI3` (0x210).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_DESI3: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

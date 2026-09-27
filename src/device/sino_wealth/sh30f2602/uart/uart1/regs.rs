use bitflags::bitflags;

use crate::register::{Reg, Reserved};

/// UART1 register block.
#[derive(Debug)]
#[repr(C)]
pub struct UART1 {
    pub FR: Reg<FR>,
    /// Reserved (0x4..=0x7).
    RESERVED0: Reserved<[u32; 1]>,
    pub TDR: Reg<TDR>,
    pub RDR: Reg<RDR>,
    pub ADDR: Reg<ADDR>,
    pub BRT: Reg<BRT>,
    pub CR: Reg<CR>,
}

bitflags! {
    /// `FR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FR: u32 {
        const LBDC = 1 << 23;

        const PEC = 1 << 22;

        const FEC = 1 << 21;

        const RXOVC = 1 << 20;

        const TXCOLC = 1 << 19;

        const TCC = 1 << 18;

        const LBD = 1 << 7;

        const PE = 1 << 6;

        const FE = 1 << 5;

        const RXOV = 1 << 4;

        const TXCOL = 1 << 3;

        const TC = 1 << 2;

        const TI = 1 << 1;

        const RI = 1 << 0;
    }
}

bitflags! {
    /// `TDR` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TDR: u32 {
        const TB8 = 1 << 8;

        const TDR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `RDR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDR: u32 {
        const RB8 = 1 << 8;

        const RDR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `ADDR` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADDR: u32 {
        const SMAR_MASK = 0xff << 8;

        const SADDR_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `BRT` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BRT: u32 {
        const BFINE_MASK = 0xf << 16;

        const SBRT_MASK = 0x7fff << 0;
    }
}

bitflags! {
    /// `CR` (0x18).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const FER = 1 << 23;

        const TNEG = 1 << 22;

        const RNEG = 1 << 21;

        const TEN = 1 << 18;

        const REN = 1 << 17;

        const LINEN = 1 << 16;

        const SBK = 1 << 15;

        const SM_MASK = 0x3 << 13;

        const MULTIE_MASK = 0x3 << 11;

        const PS = 1 << 10;

        const SMOD0 = 1 << 8;

        const LBDL = 1 << 7;

        const LBDIE = 1 << 6;

        const TCIE = 1 << 5;

        const TIE = 1 << 4;

        const RIE = 1 << 3;

        const SMOD2 = 1 << 2;

        const SBRTEN = 1 << 1;

        const STOP = 1 << 0;
    }
}

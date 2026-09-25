use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// SPI0 register block.
#[derive(Debug)]
#[repr(C)]
pub struct SPI0 {
    pub FR: Reg<FR>,
    pub TDR: Reg32,
    pub RDR: Reg32,
    pub CR: Reg<CR>,
}

bitflags! {
    /// `FR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FR: u32 {
        const WCOLC = 1 << 21;

        const RXOVC = 1 << 20;

        const MODFC = 1 << 19;

        const SPTIC = 1 << 17;

        const SPRIC = 1 << 16;

        const WCOL = 1 << 5;

        const RXOV = 1 << 4;

        const MODF = 1 << 3;

        const BUSY = 1 << 2;

        const SPTI = 1 << 1;

        const SPRI = 1 << 0;
    }
}

bitflags! {
    /// `CR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const SPDATL_MASK = 0x3 << 16;

        const SPSFF = 1 << 15;

        const SPIEN = 1 << 14;

        const SPTIE = 1 << 11;

        const SPRIE = 1 << 10;

        const DIR = 1 << 8;

        const MSTR = 1 << 7;

        const CPHA = 1 << 6;

        const CPOL = 1 << 5;

        const SSDIS = 1 << 4;

        const SPR_MASK = 0xf << 0;
    }
}

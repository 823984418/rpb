use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct SWPMI {
    pub CR: Reg<CR>,
    pub BRR: Reg<BRR>,
    RESERVED0: Reserved<u32>,
    pub ISR: Reg<ISR>,
    pub ICR: Reg<ICR>,
    pub IER: Reg<IER>,
    pub RFL: Reg<RFL>,
    pub TDR: Reg<TDR>,
    pub RDR: Reg32,
    pub OR: Reg<OR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const SWPTEN = 1 << 11;
        const DEACT = 1 << 10;
        const SWPACT = 1 << 5;
        const LPBK = 1 << 4;
        const TXMODE = 1 << 3;
        const RXMODE = 1 << 2;
        const TXDMA = 1 << 1;
        const RXDMA = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BRR: u32 {
        const BR_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const RDYF = 1 << 11;
        const DEACTF = 1 << 10;
        const SUSP = 1 << 9;
        const SRF = 1 << 8;
        const TCF = 1 << 7;
        const TXE = 1 << 6;
        const RXNE = 1 << 5;
        const TXUNRF = 1 << 4;
        const RXOVRF = 1 << 3;
        const RXBERF = 1 << 2;
        const TXBEF = 1 << 1;
        const RXBFF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ICR: u32 {
        const CRDYF = 1 << 11;
        const CSRF = 1 << 8;
        const CTCF = 1 << 7;
        const CTXUNRF = 1 << 4;
        const CRXOVRF = 1 << 3;
        const CRXBERF = 1 << 2;
        const CTXBEF = 1 << 1;
        const CRXBFF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u32 {
        const RDYIE = 1 << 11;
        const SRIE = 1 << 8;
        const TCIE = 1 << 7;
        const TIE = 1 << 6;
        const RIE = 1 << 5;
        const TXUNRIE = 1 << 4;
        const RXOVRIE = 1 << 3;
        const RXBERIE = 1 << 2;
        const TXBEIE = 1 << 1;
        const RXBFIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RFL: u32 {
        const RFL_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TDR: u32 {
        const TD_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OR: u32 {
        const SWP_ISEL_1 = 1 << 3;
        const SWP_ISEL_0 = 1 << 2;
        const SWP_ISEL_MASK = 0x3 << 2;

        const SWP_TBYP = 1 << 0;
    }
}

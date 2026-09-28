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
        const RXDMA = 0x00000001;
        const TXDMA = 0x00000002;
        const RXMODE = 0x00000004;
        const TXMODE = 0x00000008;
        const LPBK = 0x00000010;
        const SWPACT = 0x00000020;
        const DEACT = 0x00000400;
        const SWPTEN = 0x00000800;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BRR: u32 {
        const BR = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const RXBFF = 0x00000001;
        const TXBEF = 0x00000002;
        const RXBERF = 0x00000004;
        const RXOVRF = 0x00000008;
        const TXUNRF = 0x00000010;
        const RXNE = 0x00000020;
        const TXE = 0x00000040;
        const TCF = 0x00000080;
        const SRF = 0x00000100;
        const SUSP = 0x00000200;
        const DEACTF = 0x00000400;
        const RDYF = 0x00000800;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ICR: u32 {
        const CRXBFF = 0x00000001;
        const CTXBEF = 0x00000002;
        const CRXBERF = 0x00000004;
        const CRXOVRF = 0x00000008;
        const CTXUNRF = 0x00000010;
        const CTCF = 0x00000080;
        const CSRF = 0x00000100;
        const CRDYF = 0x00000800;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u32 {
        const RXBFIE = 0x00000001;
        const TXBEIE = 0x00000002;
        const RXBERIE = 0x00000004;
        const RXOVRIE = 0x00000008;
        const TXUNRIE = 0x00000010;
        const RIE = 0x00000020;
        const TIE = 0x00000040;
        const TCIE = 0x00000080;
        const SRIE = 0x00000100;
        const RDYIE = 0x00000800;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RFL: u32 {
        const RFL = 0x0000001F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TDR: u32 {
        const TD = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OR: u32 {
        const SWP_TBYP = 0x00000001;

        const SWP_ISEL = 0x0000000C;
        const SWP_ISEL_0 = 0x00000004;
        const SWP_ISEL_1 = 0x00000008;
    }
}

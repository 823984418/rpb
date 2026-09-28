use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct DAC {
    pub CTLR: Reg<CTLR>,
    pub SWTR: Reg<SWTR>,
    pub R12BDHR1: Reg<R12BDHR1>,
    pub L12BDHR1: Reg<L12BDHR1>,
    pub R8BDHR1: Reg<R8BDHR1>,
    pub R12BDHR2: Reg<R12BDHR2>,
    pub L12BDHR2: Reg<L12BDHR2>,
    pub R8BDHR2: Reg<R8BDHR2>,
    pub RD12BDHR: Reg<RD12BDHR>,
    pub LD12BDHR: Reg<LD12BDHR>,
    pub RD8BDHR: Reg<RD8BDHR>,
    pub DOR1: Reg<DOR1>,
    pub DOR2: Reg<DOR2>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const DMAEN2 = 1 << 28;

        const MAMP2_MASK = 0xF << 24;

        const WAVE2_MASK = 0x3 << 22;

        const TSEL2_MASK = 0x7 << 19;

        const TEN2 = 1 << 18;
        const BOFF2 = 1 << 17;
        const EN2 = 1 << 16;
        const DMAEN1 = 1 << 12;

        const MAMP1_MASK = 0xF << 8;

        const WAVE1_MASK = 0x3 << 6;

        const TSEL1_MASK = 0x7 << 3;

        const TEN1 = 1 << 2;
        const BOFF1 = 1 << 1;
        const EN1 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SWTR: u32 {
        const SWTRIG2 = 1 << 1;
        const SWTRIG1 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct R12BDHR1: u32 {
        const DHR12R1_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct L12BDHR1: u32 {
        const DHR12L1_MASK = 0xFFF << 4;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct R8BDHR1: u32 {
        const DHR8R1_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct R12BDHR2: u32 {
        const DHR12R2_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct L12BDHR2: u32 {
        const DHR12L2_MASK = 0xFFF << 4;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct R8BDHR2: u32 {
        const DHR8R2_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RD12BDHR: u32 {
        const DACC2DHR_MASK = 0xFFF << 16;

        const DACC1DHR_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LD12BDHR: u32 {
        const DACC2DHR_MASK = 0xFFF << 20;

        const DACC1DHR_MASK = 0xFFF << 4;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RD8BDHR: u32 {
        const DACC2DHR_MASK = 0xFF << 8;

        const DACC1DHR_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DOR1: u32 {
        const DACC1DOR_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DOR2: u32 {
        const DACC2DOR_MASK = 0xFFF << 0;
    }
}

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
        const EN1 = 0x00000001;
        const BOFF1 = 0x00000002;
        const TEN1 = 0x00000004;

        const TSEL1 = 0x00000038;
        const TSEL1_0 = 0x00000008;
        const TSEL1_1 = 0x00000010;
        const TSEL1_2 = 0x00000020;

        const WAVE1 = 0x000000C0;
        const WAVE1_0 = 0x00000040;
        const WAVE1_1 = 0x00000080;

        const MAMP1 = 0x00000F00;
        const MAMP1_0 = 0x00000100;
        const MAMP1_1 = 0x00000200;
        const MAMP1_2 = 0x00000400;
        const MAMP1_3 = 0x00000800;

        const DMAEN1 = 0x00001000;
        const EN2 = 0x00010000;
        const BOFF2 = 0x00020000;
        const TEN2 = 0x00040000;

        const TSEL2 = 0x00380000;
        const TSEL2_0 = 0x00080000;
        const TSEL2_1 = 0x00100000;
        const TSEL2_2 = 0x00200000;

        const WAVE2 = 0x00C00000;
        const WAVE2_0 = 0x00400000;
        const WAVE2_1 = 0x00800000;

        const MAMP2 = 0x0F000000;
        const MAMP2_0 = 0x01000000;
        const MAMP2_1 = 0x02000000;
        const MAMP2_2 = 0x04000000;
        const MAMP2_3 = 0x08000000;

        const DMAEN2 = 0x10000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SWTR: u32 {
        const SWTRIG1 = 0x01;
        const SWTRIG2 = 0x02;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct R12BDHR1: u32 {
        const DHR12R1 = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct L12BDHR1: u32 {
        const DHR12L1 = 0xFFF0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct R8BDHR1: u32 {
        const DHR8R1 = 0xFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct R12BDHR2: u32 {
        const DHR12R2 = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct L12BDHR2: u32 {
        const DHR12L2 = 0xFFF0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct R8BDHR2: u32 {
        const DHR8R2 = 0xFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RD12BDHR: u32 {
        const DACC1DHR = 0x00000FFF;
        const DACC2DHR = 0x0FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LD12BDHR: u32 {
        const DACC1DHR = 0x0000FFF0;
        const DACC2DHR = 0xFFF00000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RD8BDHR: u32 {
        const DACC1DHR = 0x00FF;
        const DACC2DHR = 0xFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DOR1: u32 {
        const DACC1DOR = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DOR2: u32 {
        const DACC2DOR = 0x0FFF;
    }
}

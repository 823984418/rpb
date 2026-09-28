use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct OPA {
    pub CTLR1: Reg<CTLR1>,
    pub CTLR2: Reg<CTLR2>,
    pub CTLR3: Reg<CTLR3>,
    pub CMP_CTLR: Reg<CMP_CTLR>,
    pub CMP_STATR: Reg<CMP_STATR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u32 {
        const EN1 = 0x0001;

        const MODE1 = 0x0006;
        const MODE1_0 = 0x0002;
        const MODE1_1 = 0x0004;

        const PSEL1 = 0x0008;

        const NSEL1 = 0x0070;
        const NSEL1_0 = 0x0010;
        const NSEL1_1 = 0x0020;
        const NSEL1_2 = 0x0030;

        const FBEN1 = 0x0100;
        const PGADIF1 = 0x0200;
        const HS1 = 0x0400;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u32 {
        const EN2 = 0x0001;

        const MODE2 = 0x0006;
        const MODE2_0 = 0x0002;
        const MODE2_1 = 0x0004;

        const PSEL2 = 0x0008;

        const NSEL2 = 0x0070;
        const NSEL2_0 = 0x0010;
        const NSEL2_1 = 0x0020;
        const NSEL2_2 = 0x0030;

        const FBEN2 = 0x0100;
        const PGADIF2 = 0x0200;
        const HS2 = 0x0400;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR3: u32 {
        const EN3 = 0x0001;

        const MODE3 = 0x0006;
        const MODE3_0 = 0x0002;
        const MODE3_1 = 0x0004;

        const PSEL3 = 0x0008;

        const NSEL3 = 0x0070;
        const NSEL3_0 = 0x0010;
        const NSEL3_1 = 0x0020;
        const NSEL3_2 = 0x0030;

        const FBEN3 = 0x0100;
        const PGADIF3 = 0x0200;
        const HS3 = 0x0400;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP_CTLR: u32 {
        const PSEL = 0x00000003;
        const PSEL_0 = 0x00000001;
        const PSEL_1 = 0x00000002;

        const NSEL = 0x0000000C;
        const NSEL_0 = 0x00000004;
        const NSEL_1 = 0x00000008;

        const MODE = 0x000000F0;
        const MODE_0 = 0x00000010;
        const MODE_1 = 0x00000020;
        const MODE_2 = 0x00000040;
        const MODE_3 = 0x00000080;

        const EN = 0x00000100;

        const HYPSEL = 0x00000600;
        const HYPSEL_0 = 0x00000200;
        const HYPSEL_1 = 0x00000400;

        const VREF = 0x00001800;
        const VREF_0 = 0x00000800;
        const VREF_1 = 0x00001000;

        const FILT_EN = 0x00002000;

        const FILT_CFG = 0x01FF0000;
        const FILT_BASE = 0x70000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP_STATR: u32 {
        const OUTFILT = 0x01;
    }
}

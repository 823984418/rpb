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
        const HS1 = 1 << 10;
        const PGADIF1 = 1 << 9;
        const FBEN1 = 1 << 8;

        const NSEL1_1 = 1 << 5;
        const NSEL1_0 = 1 << 4;

        const NSEL1_2 = 0b011 << 4;
        const NSEL1_MASK = 0x7 << 4;

        const PSEL1 = 1 << 3;

        const MODE1_1 = 1 << 2;
        const MODE1_0 = 1 << 1;
        const MODE1_MASK = 0x3 << 1;

        const EN1 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u32 {
        const HS2 = 1 << 10;
        const PGADIF2 = 1 << 9;
        const FBEN2 = 1 << 8;

        const NSEL2_1 = 1 << 5;
        const NSEL2_0 = 1 << 4;

        const NSEL2_2 = 0b011 << 4;
        const NSEL2_MASK = 0x7 << 4;

        const PSEL2 = 1 << 3;

        const MODE2_1 = 1 << 2;
        const MODE2_0 = 1 << 1;
        const MODE2_MASK = 0x3 << 1;

        const EN2 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR3: u32 {
        const HS3 = 1 << 10;
        const PGADIF3 = 1 << 9;
        const FBEN3 = 1 << 8;

        const NSEL3_1 = 1 << 5;
        const NSEL3_0 = 1 << 4;

        const NSEL3_2 = 0b011 << 4;
        const NSEL3_MASK = 0x7 << 4;

        const PSEL3 = 1 << 3;

        const MODE3_1 = 1 << 2;
        const MODE3_0 = 1 << 1;
        const MODE3_MASK = 0x3 << 1;

        const EN3 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP_CTLR: u32 {
        const FILT_BASE_MASK = 0x7 << 28;

        const FILT_CFG_MASK = 0x1FF << 16;

        const FILT_EN = 1 << 13;

        const VREF_1 = 1 << 12;
        const VREF_0 = 1 << 11;
        const VREF_MASK = 0x3 << 11;

        const HYPSEL_1 = 1 << 10;
        const HYPSEL_0 = 1 << 9;
        const HYPSEL_MASK = 0x3 << 9;

        const EN = 1 << 8;

        const MODE_3 = 1 << 7;
        const MODE_2 = 1 << 6;
        const MODE_1 = 1 << 5;
        const MODE_0 = 1 << 4;
        const MODE_MASK = 0xF << 4;

        const NSEL_1 = 1 << 3;
        const NSEL_0 = 1 << 2;
        const NSEL_MASK = 0x3 << 2;

        const PSEL_1 = 1 << 1;
        const PSEL_0 = 1 << 0;
        const PSEL_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP_STATR: u32 {
        const OUTFILT = 1 << 0;
    }
}

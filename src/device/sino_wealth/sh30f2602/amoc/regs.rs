use bitflags::bitflags;

use crate::register::Reg;

/// AMOC register block.
#[derive(Debug)]
#[repr(C)]
pub struct AMOC {
    pub CMP0CON: Reg<CMP0CON>,
    pub CMP1CON: Reg<CMP1CON>,
    pub CMPINTF: Reg<CMPINTF>,
    pub OPCON: Reg<OPCON>,
    pub CMP2CON: Reg<CMP2CON>,
}

bitflags! {
    /// `CMP0CON` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP0CON: u32 {
        const CP0NOUTEN = 1 << 30;

        const CMP0VREF_MASK = 0xf << 26;

        const TRGPOL = 1 << 22;

        const PWMTRGS_MASK = 0x3 << 20;

        const VSUMCHS_MASK = 0x3 << 17;

        const CHSUMOUTEN = 1 << 16;

        const CMP0EN = 1 << 15;

        const C0OUTEN = 1 << 12;

        const C0IES_MASK = 0x3 << 10;

        const OP0NCHS_MASK = 0x3 << 8;

        const OP0PCHS_MASK = 0x7 << 4;

        const C0OUT = 1 << 3;

        const C0DEB_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `CMP1CON` (0x4).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP1CON: u32 {
        const CP1NOUTEN = 1 << 30;

        const CMP1VRS_MASK = 0xf << 26;

        const CMP1EN = 1 << 15;

        const C1SMT = 1 << 13;

        const C1OUTEN = 1 << 12;

        const C1IES_MASK = 0x3 << 10;

        const C1NCHS = 1 << 8;

        const C1PCHS_MASK = 0xf << 4;

        const C1OUT = 1 << 3;

        const C1DEB_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `CMPINTF` (0x8).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMPINTF: u32 {
        const C2IFC = 1 << 18;

        const C1IFC = 1 << 17;

        const C0IFC = 1 << 16;

        const C2IF = 1 << 2;

        const C1IF = 1 << 1;

        const C0IF = 1 << 0;
    }
}

bitflags! {
    /// `OPCON` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPCON: u32 {
        const OP3EN = 1 << 3;

        const OP2EN = 1 << 2;

        const OP1EN = 1 << 1;

        const OP0EN = 1 << 0;
    }
}

bitflags! {
    /// `CMP2CON` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CMP2CON: u32 {
        const CP2NOUTEN = 1 << 30;

        const CMP2VRS_MASK = 0xf << 26;

        const CMP2EN = 1 << 15;

        const C2SMT = 1 << 13;

        const C2OUTEN = 1 << 12;

        const C2IES_MASK = 0x3 << 10;

        const C2NCHS = 1 << 8;

        const C2PCHS_MASK = 0xf << 4;

        const C2OUT = 1 << 3;

        const C2DEB_MASK = 0x7 << 0;
    }
}

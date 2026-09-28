use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct PWR {
    pub CTLR: Reg<CTLR>,
    pub CSR: Reg<CSR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const VSEL_VIO18_MODE7 = 0b111 << 10;
        const VSEL_VIO18_MODE6 = 0b110 << 10;
        const VSEL_VIO18_MODE5 = 0b101 << 10;
        const VSEL_VIO18_MODE4 = 0b100 << 10;
        const VSEL_VIO18_MODE3 = 0b011 << 10;
        const VSEL_VIO18_MODE2 = 0b010 << 10;
        const VSEL_VIO18_MODE1 = 0b001 << 10;
        const VSEL_VIO18_MODE0 = 0b000 << 10;
        const VSEL_VIO18_MASK = 0x7 << 10;

        const VIO_SWCR = 1 << 9;
        const DBP = 1 << 8;

        const PLS_MODE7 = 0b111 << 5;
        const PLS_MODE6 = 0b110 << 5;
        const PLS_MODE5 = 0b101 << 5;
        const PLS_MODE4 = 0b100 << 5;
        const PLS_MODE3 = 0b011 << 5;
        const PLS_MODE2 = 0b010 << 5;
        const PLS_MODE1 = 0b001 << 5;
        const PLS_MODE0 = 0b000 << 5;
        const PLS_MASK = 0x7 << 5;

        const PVDE = 1 << 4;
        const LPDS = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CSR: u32 {
        const VIO18_SR_MASK = 0x3 << 8;

        const PVDO = 1 << 0;
    }
}

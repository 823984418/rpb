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
        const LPDS = 0x0001;
        const PVDE = 0x0010;

        const PLS = 0x00E0;
        const PLS_0 = 0x0020;
        const PLS_1 = 0x0040;
        const PLS_2 = 0x0080;

        const PLS_MODE0 = 0x0000;
        const PLS_MODE1 = 0x0020;
        const PLS_MODE2 = 0x0040;
        const PLS_MODE3 = 0x0060;
        const PLS_MODE4 = 0x0080;
        const PLS_MODE5 = 0x00A0;
        const PLS_MODE6 = 0x00C0;
        const PLS_MODE7 = 0x00E0;

        const DBP = 0x0100;
        const VIO_SWCR = 0x0200;

        const VSEL_VIO18 = 0x1C00;
        const VSEL_VIO18_0 = 0x0400;
        const VSEL_VIO18_1 = 0x0800;
        const VSEL_VIO18_2 = 0x1000;

        const VSEL_VIO18_MODE0 = 0x0000;
        const VSEL_VIO18_MODE1 = 0x0400;
        const VSEL_VIO18_MODE2 = 0x0800;
        const VSEL_VIO18_MODE3 = 0x0C00;
        const VSEL_VIO18_MODE4 = 0x1000;
        const VSEL_VIO18_MODE5 = 0x1400;
        const VSEL_VIO18_MODE6 = 0x1800;
        const VSEL_VIO18_MODE7 = 0x1C00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CSR: u32 {
        const PVDO = 0x0001;

        const VIO18_SR = 0x0300;
        const VIO18_SR_0 = 0x0100;
        const VIO18_SR_1 = 0x0200;
    }
}

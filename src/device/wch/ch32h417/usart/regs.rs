use bitflags::bitflags;

use crate::register::{Reg, Reg8, Reg32};

#[derive(Debug)]
#[repr(C)]
pub struct USART {
    pub STATR: Reg<STATR>,
    pub DATAR: Reg32,
    // IQ4
    pub BRR: Reg32,
    pub CTLR1: Reg<CTLR1>,
    pub CTLR2: Reg<CTLR2>,
    pub CTLR3: Reg<CTLR3>,
    pub GPR: Reg<GPR>,
    pub CTLR4: Reg<CTLR4>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const LPWKUP_ACT_FLAG = 1 << 15;
        const STATR_USART_WKUP = 0x8000;
        const MS_ERR = 1 << 11;
        const RX_BUSY = 1 << 10;
        const CTS = 1 << 9;
        const LBD = 1 << 8;
        const TXE = 1 << 7;
        const TC = 1 << 6;
        const RXNE = 1 << 5;
        const IDLE = 1 << 4;
        const ORE = 1 << 3;
        const NE = 1 << 2;
        const FE = 1 << 1;
        const PE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATAR: u32 {
        const DATAR_DR = 0x01FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BRR: u32 {
        const BRR_DIV_Fraction = 0x000F;
        const BRR_DIV_Mantissa = 0xFFF0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u32 {
        const M_EXT_5 = 0b11 << 14;
        const M_EXT_6 = 0b10 << 14;
        const M_EXT_7 = 0b01 << 14;
        const M_EXT_M = 0b00 << 14;
        const M_EXT_MASK = 0x3 << 14;

        const CTLR1_M_EXT = 0xC000;
        const CTLR1_M_EXT_0 = 0x4000;
        const CTLR1_M_EXT_1 = 0x8000;

        const UE = 1 << 13;
        const M = 1 << 12;
        const WAKE = 1 << 11;
        const PCE = 1 << 10;
        const PS = 1 << 9;
        const PEIE = 1 << 8;
        const TXEIE = 1 << 7;
        const TCIE = 1 << 6;
        const RXNEIE = 1 << 5;
        const IDLEIE = 1 << 4;
        const TE = 1 << 3;
        const RE = 1 << 2;
        const RWU = 1 << 1;
        const SBK = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u32 {
        const LINEN = 1 << 14;

        const STOP_1_5 = 0b11 << 12;
        const STOP_2 = 0b10 << 12;
        const STOP_0_5 = 0b01 << 12;
        const STOP_1 = 0b00 << 12;
        const STOP_MASK = 0x3 << 12;

        const CLKEN = 1 << 11;
        const CPOL = 1 << 10;
        const CPHA = 1 << 9;
        const LBCL = 1 << 8;
        const LBDIE = 1 << 6;
        const LBDL = 1 << 5;

        const ADD_MASK = 0xF << 0;

        const CTLR2_ADD = 0x000F;
        const CTLR2_STOP_0 = 0x1000;
        const CTLR2_STOP_1 = 0x2000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR3: u32 {
        const LPWKUP_DLY_CFG = 0x7 << 13;
        const LPWKUP_CK_SRC = 1 << 12;
        const LPWKUP_EN = 1 << 11;
        const CTSIE = 1 << 10;
        const CTSE = 1 << 9;
        const RTSE = 1 << 8;
        const DMAT = 1 << 7;
        const DMAR = 1 << 6;
        const SCEN = 1 << 5;
        const NACK = 1 << 4;
        const HDSEL = 1 << 3;
        const IRLP = 1 << 2;
        const IREN = 1 << 1;
        const EIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GPR: u32 {
        const GT_MASK = 0xFF << 8;
        const PSC_MASK = 0xFF << 0;

        const GPR_PSC = 0x00FF;
        const GPR_PSC_0 = 0x0001;
        const GPR_PSC_1 = 0x0002;
        const GPR_PSC_2 = 0x0004;
        const GPR_PSC_3 = 0x0008;
        const GPR_PSC_4 = 0x0010;
        const GPR_PSC_5 = 0x0020;
        const GPR_PSC_6 = 0x0040;
        const GPR_PSC_7 = 0x0080;

        const GPR_GT = 0xFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR4: u32 {
        const CHECK_SEL_MASK = 0x3 << 2;
        const MS_ERRIE = 1 << 1;

        const CTLR4_CHECK_MARKENABLE = 0x0008;
        const CTLR4_CHECK_APACEENABLE = 0x000C;
    }
}

impl USART {
    pub const fn GT(&self) -> &Reg8 {
        unsafe { Reg::from_ptr((self.GPR.as_ptr() as *mut u8).add(1)) }
    }

    pub const fn PSC(&self) -> &Reg8 {
        unsafe { Reg::from_ptr(self.GPR.as_ptr() as *mut u8) }
    }
}

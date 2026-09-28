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
        const DR_MASK = 0x1FF << 0;
    }
}

/// USART波特率分频寄存器
/// 虽然官方手册指示其为小数分频，但前方已经经过相应预分频，因此实际上这个寄存器相对模块时钟就是整数分频
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BRR: u32 {
        const DIV_Mantissa_MASK = 0xFFF << 4;
        const DIV_Fraction_MASK = 0xF << 0;
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
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR3: u32 {
        const LPWKUP_DLY_CFG_MASK = 0x7 << 13;
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
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR4: u32 {
        const CHECK_SEL_SPACEENABLE = 0b11 << 2;
        const CHECK_SEL_MARKENABLE = 0b10 << 2;
        const CHECK_SEL_MASK = 0x3 << 2;

        const MS_ERRIE = 1 << 1;
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

use bitflags::bitflags;

use crate::device::wch::ch32h417::rcc::RCC;
use crate::register::{Reg, Reg32};

#[derive(Debug)]
#[repr(C)]
pub struct USART {
    pub START: Reg<START>,
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
    pub struct START: u32 {
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
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR4: u32 {
        const CHECK_SEL_MASK = 0x3 << 2;
        const MS_ERRIE = 1 << 1;
    }
}

pub struct USART_Init {
    pub BaudRate: u32,
    pub WordLength: USART_WordLength,
    pub StopBits: USART_StopBits,
    pub Parity: USART_Parity,
    pub Mode: USART_Mode,
    pub HardwareFlowControl: USART_HardwareFlowControl,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct USART_WordLength: u32 {
        const WordLength_5b = 0b1100 << 12;
        const WordLength_6b = 0b1000 << 12;
        const WordLength_7b = 0b0100 << 12;
        const WordLength_8b = 0b0000 << 12;
        const WordLength_9b = 0b0001 << 12;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct USART_StopBits: u32 {
        const StopBits_1_5 = 0b11 << 12;
        const StopBits_2 = 0b10 << 12;
        const StopBits_0_5 = 0b01 << 12;
        const StopBits_1 = 0b00 << 12;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct USART_Parity: u32 {
        const Parity_Space = 0xF00C;
        const Parity_Mark = 0xF008;
        const Parity_Odd = 0x0600;
        const Parity_Even = 0x0400;
        const Parity_No = 0x0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct USART_Mode: u32 {
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct USART_HardwareFlowControl: u32 {
    }
}

impl USART {
    pub fn Init(&self, InitStruct: &USART_Init) {
        let mut Parity = InitStruct.Parity;
        if Parity.bits() & 0xF000 == 0xF000 {
            self.CTLR4.modify(
                CTLR4::CHECK_SEL_MASK,
                CTLR4::from_bits_retain(Parity.bits() & 0xC),
            );
            Parity = USART_Parity::empty();
        }
        self.CTLR2.modify(
            CTLR2::STOP_MASK,
            CTLR2::from_bits_retain(InitStruct.StopBits.bits()),
        );
        self.CTLR1.modify(
            CTLR1::M_EXT_MASK | CTLR1::M | CTLR1::PCE | CTLR1::PS | CTLR1::TE | CTLR1::RE,
            CTLR1::from_bits_retain(
                InitStruct.WordLength.bits() | Parity.bits() | InitStruct.Mode.bits(),
            ),
        );
        self.CTLR3.modify(
            CTLR3::CTSE | CTLR3::RTSE,
            CTLR3::from_bits_retain(InitStruct.HardwareFlowControl.bits()),
        );

        let device = unsafe { core::mem::zeroed::<super::Device>() };
        let ClocksStatus = device.RCC.GetClocksFreq();
        let brr = ClocksStatus.HCLK as u64 * 16 / InitStruct.BaudRate as u64;
        self.BRR.set(brr.clamp(0, 0xFFFF) as u32);
    }
}

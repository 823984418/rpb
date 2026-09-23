mod regs;

use bitflags::bitflags;
pub use regs::*;

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
        const Mode_Tx = 0x0008;
        const Mode_Rx = 0x0004;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct USART_HardwareFlowControl: u32 {
        const HardwareFlowControl_CTS = 0x0200;
        const HardwareFlowControl_RTS = 0x0100;
    }
}

impl USART {
    pub fn Init(&self, InitStruct: &USART_Init, HCLK: u32) {
        let mut Parity = InitStruct.Parity;
        if Parity.bits() & 0xF000 == 0xF000 {
            self.CTLR4.bit_clear_or(
                CTLR4::CHECK_SEL_MASK,
                CTLR4::from_bits_retain(Parity.bits() & 0xC),
            );
            Parity = USART_Parity::empty();
        }
        self.CTLR2.bit_clear_or(
            CTLR2::STOP_MASK,
            CTLR2::from_bits_retain(InitStruct.StopBits.bits()),
        );
        self.CTLR1.bit_clear_or(
            CTLR1::M_EXT_MASK | CTLR1::M | CTLR1::PCE | CTLR1::PS | CTLR1::TE | CTLR1::RE,
            CTLR1::from_bits_retain(
                InitStruct.WordLength.bits() | Parity.bits() | InitStruct.Mode.bits(),
            ),
        );
        self.CTLR3.bit_clear_or(
            CTLR3::CTSE | CTLR3::RTSE,
            CTLR3::from_bits_retain(InitStruct.HardwareFlowControl.bits()),
        );

        let brr = HCLK as u64 / InitStruct.BaudRate as u64;
        self.BRR.set(brr.clamp(0, 0xFFFF) as u32);
    }

    pub fn Cmd(&self, NewState: bool) {
        self.CTLR1.bit_set(CTLR1::UE, NewState);
    }

    pub fn GetFlagStatus(&self) -> STATR {
        self.STATR.get()
    }

    pub fn SendData(&self, Data: u32) {
        self.DATAR.set(Data);
    }

    pub fn ReceiveData(&self) -> u32 {
        self.DATAR.get()
    }

    pub fn SendBreak(&self) {
        self.CTLR1.bit_or(CTLR1::SBK);
    }
}

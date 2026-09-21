use crate::register::Reg32;

#[repr(C)]
pub struct NVIC {
    /// Interrupt Set-Enable
    pub ISER: [Reg32; 16],

    RESERVED0: [Reg32; 16],

    /// Interrupt Clear-Enable
    pub ICER: [Reg32; 16],

    RESERVED1: [Reg32; 16],

    /// Interrupt Set-Pending
    pub ISPR: [Reg32; 16],

    RESERVED2: [Reg32; 16],

    /// Interrupt Clear-Pending
    pub ICPR: [Reg32; 16],

    RESERVED3: [Reg32; 16],

    /// Interrupt Active Bit
    /// v6不可用
    pub IABR: [Reg32; 16],

    RESERVED4: [Reg32; 16],

    /// Interrupt Target Non-secure
    /// 仅 v8
    pub ITNS: [Reg32; 16],

    RESERVED5: [Reg32; 16],

    /// v6 为8个32位寄存器
    /// v7以上允许进行8位和16位访问
    pub IPR: [Reg32; 124],

    RESERVED8: [Reg32; 580],

    /// Software Trigger Interrupt
    /// ARMv6-M不可用
    pub STIR: Reg32,
}

use crate::register::Reg32;

/// 系统计数器
#[repr(C)]
pub struct SysTick {
    pub CTLR: Reg32,

    /// 仅用于 SysTick0
    pub ISR: Reg32,

    pub CNT: Reg32,

    pub RESERVED0: Reg32,

    pub CMP: Reg32,
}

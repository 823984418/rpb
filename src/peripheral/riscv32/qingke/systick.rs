use crate::register::Reg32;


/// 系统计数器
/// 这是 WCH 的专用外设
#[repr(C)]
pub struct SysTick {
    pub CTLR: Reg32,

    /// SysTick0可用
    /// 与其他系统计数器共用
    pub ISR: Reg32,

    pub CNT: Reg32,

    pub RESERVED0: Reg32,

    pub CMP: Reg32,
}

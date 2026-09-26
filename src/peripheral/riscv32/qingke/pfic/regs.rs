use bitflags::bitflags;

use crate::register::{Reg, Reg8, Reg32};

/// 可编程快速中断控制器
/// 这是 WCH 的专用外设
#[derive(Debug)]
#[repr(C)]
pub struct PFIC {
    /// 中断使能状态寄存器 x
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub ISR: [Reg32; 8],

    /// 中断挂起状态寄存器 x
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub IPR: [Reg32; 8],

    /// 中断优先级阈值配置寄存器
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub ITHRESDR: Reg32,

    /// 免表中断基地址寄存器
    ///
    /// # 可用架构
    /// * V3(A)
    pub VTFBADDRR: Reg32,

    /// 中断配置寄存器
    ///
    /// # 可用架构
    /// * V3(AFV)
    /// * V5
    pub CFGR: Reg<CFGR>,

    /// 中断全局状态寄存器
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub GISR: Reg32,

    /// VTF 中断 ID 配置寄存器
    ///
    /// # 可用架构
    /// * V3(BCFV)
    /// * V5
    pub VTFIDR: Reg32,

    RESERVED0: [Reg32; 3],

    /// VTF 中断 x 偏移地址寄存器
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub VTFADDRR: [Reg32; 4],

    RESERVED1: [Reg8; 0x90],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IENR: [Reg32; 8],

    RESERVED2: [Reg8; 0x60],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IRER: [Reg32; 8],

    RESERVED3: [Reg8; 0x60],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IPSR: [Reg32; 8],

    RESERVED4: [Reg8; 0x60],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IPRR: [Reg32; 8],

    RESERVED5: [Reg8; 0x60],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IACTR: [Reg32; 8],

    RESERVED6: [Reg8; 0xE0],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IPRIOR: [Reg8; 256],

    RESERVED7: [Reg8; 0x100],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IALLOCR: [Reg8; 256],

    /// # 可用架构
    /// * V3
    /// * V5
    pub IAUTR: [Reg32; 8],

    /// # 可用架构
    /// * V3
    /// * V5
    pub WAKEIP: [Reg32; 2],

    RESERVED8: [Reg8; 0x58],

    /// 内核状态寄存器
    ///
    /// # 可用架构
    /// * V3
    /// * V5
    pub CSTAR: [Reg<CSTAR>; 2],

    RESERVED9: [Reg8; 0x4F8],

    /// # 可用架构
    /// * V3
    /// * V5
    pub EENR: Reg32,

    /// # 可用架构
    /// * V3
    /// * V5
    pub EPR: Reg32,

    /// # 可用架构
    /// * V3
    /// * V5
    pub EWUPR: Reg32,

    RESERVED10: [Reg8; 0x84],

    /// # 可用架构
    /// * V3
    /// * V5
    pub SCTLR: Reg<SCTLR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const KEYCODE = 0xBEEF << 16;
        const KEYCODE_MASK = 0xFFFF << 16;

        const SYSRST = 1 << 7;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SCTLR: u32 {
        /// # 可用架构
        /// * V3(BCFV)
        /// * V5
        const SYSRESET = 1 << 31;

        /// # 可用架构
        /// * V3F
        /// * V5
        const HART_ID_MASK = 0xFF << 16;
        const SETEVENT = 1 << 5;
        const SEVONPEND = 1 << 4;
        const WFITOWFE = 1 << 3;
        const SLEEPDEEP = 1 << 2;
        const SLEEPONEXIT = 1 << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CSTAR: u32 {
        const CPU_EX_STATE_LOCK_UP = 0b11 << 14;
        const CPU_EX_STATE_DEEP_SLEEP = 0b10 << 14;
        const CPU_EX_STATE_SLEEP = 0b01 << 14;
        const CPU_EX_STATE_RUN = 0b00 << 14;
        const CPU_EX_STATE_MASK = 0x3 << 14;

        const CPU_LOCK_UP = 1 << 13;
        const CPU_DBG_MODE = 1 << 12;
        const CPU_GLOBL_IE = 1 << 11;
        const CPU_IRQ_PEND = 1 << 9;
        const CPU_IRQ_ACTIVE = 1 << 8;
        const CPU_NEST_STA_MASK = 0xFF << 0;
    }
}

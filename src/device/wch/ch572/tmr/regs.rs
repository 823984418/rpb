use bitflags::bitflags;

use crate::register::{Reg, Reg16, Reg32, Reg8};

/// 定时器寄存器组，基地址 `0x40002400`
#[derive(Debug)]
#[repr(C)]
pub struct TMR {
    /// 定时器模式控制寄存器
    pub MOD: Reg<MOD>, // 0x00

    /// 定时器 DMA 控制寄存器
    pub DMA: Reg<DMA>, // 0x01

    /// 定时器中断使能寄存器
    pub INTER_EN: Reg<INTER_EN>, // 0x02

    RESERVED0: Reg8, // 0x03

    RESERVED1: Reg16, // 0x04 ~ 0x05

    /// 定时器中断标志寄存器，写 1 清除
    pub INT_FLAG: Reg<INT_FLAG>, // 0x06

    /// 定时器 FIFO 计数状态寄存器
    pub FIFO_COUNT: Reg8, // 0x07

    /// 定时器当前计数值，低 16 位与低 8 位有效
    pub COUNT: Reg32, // 0x08

    /// 定时器结束计数值，仅低 26 位有效
    pub CNT_END: Reg32, // 0x0C

    /// 定时器 FIFO 寄存器，仅低 26 位有效
    pub FIFO: Reg32, // 0x10

    /// 定时器 DMA 当前地址
    pub DMA_NOW: Reg32, // 0x14

    /// 定时器 DMA 起始地址
    pub DMA_BEG: Reg32, // 0x18

    /// 定时器 DMA 结束地址
    pub DMA_END: Reg32, // 0x1C
}

impl TMR {
    /// 当前计数值的低 16 位
    pub const fn COUNT16(&self) -> &Reg16 {
        unsafe { Reg::from_ptr(self.COUNT.as_ptr() as *mut u16) }
    }

    /// 当前计数值的低 8 位
    pub const fn COUNT8(&self) -> &Reg8 {
        unsafe { Reg::from_ptr(self.COUNT.as_ptr() as *mut u8) }
    }

    /// FIFO 寄存器的低 16 位
    pub const fn FIFO16(&self) -> &Reg16 {
        unsafe { Reg::from_ptr(self.FIFO.as_ptr() as *mut u16) }
    }

    /// FIFO 寄存器的低 8 位
    pub const fn FIFO8(&self) -> &Reg8 {
        unsafe { Reg::from_ptr(self.FIFO.as_ptr() as *mut u8) }
    }

    /// DMA 当前地址的低 16 位
    pub const fn DMA_NOW16(&self) -> &Reg16 {
        unsafe { Reg::from_ptr(self.DMA_NOW.as_ptr() as *mut u16) }
    }

    /// DMA 起始地址的低 16 位
    pub const fn DMA_BEG16(&self) -> &Reg16 {
        unsafe { Reg::from_ptr(self.DMA_BEG.as_ptr() as *mut u16) }
    }

    /// DMA 结束地址的低 16 位
    pub const fn DMA_END16(&self) -> &Reg16 {
        unsafe { Reg::from_ptr(self.DMA_END.as_ptr() as *mut u16) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MOD: u8 {
        /// PWM 重复模式：00=1，01=4，10=8，11=16；捕获模式下为捕获边沿模式
        const PWM_REPEAT_MASK = 0xC0;
        /// 定时器 PWM 输出极性：0=默认低电平高动作，1=默认高电平低动作
        const OUT_POLAR = 1 << 4;
        /// 定时器输出使能
        const OUT_EN = 1 << 3;
        /// 定时器计数使能
        const COUNT_EN = 1 << 2;
        /// 强制清除定时器 FIFO 和计数
        const ALL_CLEAR = 1 << 1;
        /// 定时器输入模式：0=定时/PWM，1=捕获/计数
        const MODE_IN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DMA: u8 {
        /// 定时器 DMA 地址循环使能
        const DMA_LOOP = 1 << 2;
        /// 定时器 DMA 使能
        const DMA_ENABLE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTER_EN: u8 {
        /// 定时器 FIFO 溢出中断使能
        const IE_FIFO_OV = 1 << 4;
        /// 定时器 DMA 完成中断使能
        const IE_DMA_END = 1 << 3;
        /// 定时器 FIFO 半满中断使能
        const IE_FIFO_HF = 1 << 2;
        /// 定时器捕获输入动作或 PWM 触发中断使能
        const IE_DATA_ACT = 1 << 1;
        /// 定时器捕获计数超时或 PWM 周期结束中断使能
        const IE_CYC_END = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FLAG: u8 {
        /// 定时器 FIFO 溢出，写 1 清除
        const IF_FIFO_OV = 1 << 4;
        /// 定时器 DMA 完成，写 1 清除
        const IF_DMA_END = 1 << 3;
        /// 定时器 FIFO 半满，写 1 清除
        const IF_FIFO_HF = 1 << 2;
        /// 定时器捕获输入动作或 PWM 触发，写 1 清除
        const IF_DATA_ACT = 1 << 1;
        /// 定时器捕获计数超时或 PWM 周期结束，写 1 清除
        const IF_CYC_END = 1 << 0;
    }
}

/// 编码器寄存器组，基地址 `0x40002420`
#[derive(Debug)]
#[repr(C)]
pub struct ENC {
    /// 编码器控制寄存器
    pub REG_CTRL: Reg<ENC_CTRL>, // 0x00

    /// 编码器中断使能寄存器
    pub INTER_EN: Reg<ENC_INTER_EN>, // 0x01

    /// 编码器中断标志寄存器
    pub INT_FLAG: Reg<ENC_INT_FLAG>, // 0x02

    RESERVED0: Reg8, // 0x03

    /// 编码器结束计数值
    pub REG_CEND: Reg32, // 0x04

    /// 编码器当前计数值
    pub REG_CCNT: Reg32, // 0x08
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ENC_CTRL: u8 {
        /// 编码器方向：0=正向，1=反向
        const ENC_DIR = 1 << 5;
        /// 读取当前计数值后清除编码器计数
        const RD_CLR_EN = 1 << 3;
        /// SMS 模式：10=T1EDGE，01=T2EDGE，11=T12EDGE
        const SMS_MODE_MASK = 0x06;
        /// 启动编码器
        const START_ENC_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ENC_INTER_EN: u8 {
        /// 编码器减计数中断使能
        const IE_DIR_DEC = 1 << 1;
        /// 编码器增计数中断使能
        const IE_DIR_INC = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ENC_INT_FLAG: u8 {
        /// 编码器减计数中断标志
        const IF_DIR_DEC = 1 << 1;
        /// 编码器增计数中断标志
        const IF_DIR_INC = 1 << 0;
    }
}

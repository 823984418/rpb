use bitflags::bitflags;

use crate::register::{Reg, Reg16, Reg32, Reg8};

/// PWM1~PWM5 寄存器组，基地址 `0x40005000`
///
/// 数据寄存器存在 16 位与 8 位两套视图，且地址布局不同：
/// 16 位模式下为 `PWM1@0x04`、`PWM2@0x06`、`PWM3@0x08`、`PWM4@0x10`、`PWM5@0x12`，
/// 8 位模式下为 `PWM1@0x04`、`PWM2@0x05`、`PWM3@0x06`、`PWM4@0x10`、`PWM5@0x11`。
#[derive(Debug)]
#[repr(C)]
pub struct PWMX {
    /// PWM 输出使能控制寄存器
    pub OUT_EN: Reg<OUT_EN>, // 0x00

    /// PWM 输出极性控制寄存器
    pub POLAR: Reg<POLAR>, // 0x01

    /// PWM 配置寄存器
    pub CONFIG: Reg<CONFIG>, // 0x02

    /// PWM DMA 控制寄存器
    pub DMA_CTRL: Reg<DMA_CTRL>, // 0x03

    /// PWM1 数据寄存器（16 位）
    pub PWM1_DATA: Reg16, // 0x04

    /// PWM2 数据寄存器（16 位）
    pub PWM2_DATA: Reg16, // 0x06

    /// PWM3 数据寄存器（16 位）
    pub PWM3_DATA: Reg16, // 0x08

    RESERVED0: Reg16, // 0x0A

    /// PWM 中断使能寄存器
    pub INT_EN: Reg<INT_EN>, // 0x0C

    /// PWM 中断标志寄存器，写 1 清除
    pub INT_FLAG: Reg<INT_FLAG>, // 0x0D

    RESERVED1: Reg16, // 0x0E

    /// PWM4 数据寄存器（16 位）
    pub PWM4_DATA: Reg16, // 0x10

    /// PWM5 数据寄存器（16 位）
    pub PWM5_DATA: Reg16, // 0x12

    /// PWM1/2/3 周期值（16 位）
    pub CYC_VALUE: Reg16, // 0x14

    /// PWM4/5 周期值（16 位）
    pub CYC1_VALUE: Reg16, // 0x16

    /// PWM 时钟分频寄存器
    pub CLOCK_DIV: Reg16, // 0x18

    RESERVED2: Reg16, // 0x1A

    /// PWM DMA 当前地址
    pub DMA_NOW: Reg32, // 0x1C

    /// PWM DMA 起始地址
    pub DMA_BEG: Reg32, // 0x20

    /// PWM DMA 结束地址
    pub DMA_END: Reg32, // 0x24
}

impl PWMX {
    /// 8 位数据模式下的 PWM1 数据寄存器
    pub const fn PWM1_DATA8(&self) -> &Reg8 {
        unsafe { Reg::from_ptr(self.PWM1_DATA.as_ptr() as *mut u8) }
    }

    /// 8 位数据模式下的 PWM2 数据寄存器
    pub const fn PWM2_DATA8(&self) -> &Reg8 {
        unsafe { Reg::from_ptr((self.PWM1_DATA.as_ptr() as *mut u8).add(1)) }
    }

    /// 8 位数据模式下的 PWM3 数据寄存器
    pub const fn PWM3_DATA8(&self) -> &Reg8 {
        unsafe { Reg::from_ptr((self.PWM1_DATA.as_ptr() as *mut u8).add(2)) }
    }

    /// 8 位数据模式下的 PWM4 数据寄存器
    pub const fn PWM4_DATA8(&self) -> &Reg8 {
        unsafe { Reg::from_ptr(self.PWM4_DATA.as_ptr() as *mut u8) }
    }

    /// 8 位数据模式下的 PWM5 数据寄存器
    pub const fn PWM5_DATA8(&self) -> &Reg8 {
        unsafe { Reg::from_ptr((self.PWM4_DATA.as_ptr() as *mut u8).add(1)) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OUT_EN: u8 {
        /// PWM5 输出使能
        const PWM5_OUT_EN = 0x10;
        /// PWM4 输出使能
        const PWM4_OUT_EN = 0x08;
        /// PWM3 输出使能
        const PWM3_OUT_EN = 0x04;
        /// PWM2 输出使能
        const PWM2_OUT_EN = 0x02;
        /// PWM1 输出使能
        const PWM1_OUT_EN = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct POLAR: u8 {
        /// PWM5 输出极性：0=默认低电平高动作，1=默认高电平低动作
        const PWM5_POLAR = 0x10;
        /// PWM4 输出极性
        const PWM4_POLAR = 0x08;
        /// PWM3 输出极性
        const PWM3_POLAR = 0x04;
        /// PWM2 输出极性
        const PWM2_POLAR = 0x02;
        /// PWM1 输出极性
        const PWM1_POLAR = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CONFIG: u8 {
        /// 同步使能
        const SYNC_EN = 0x80;
        /// 同步启动使能
        const SYNC_START = 0x40;
        /// 1=PWM4 通道输出，0=PWM5 通道输出
        const PWM4_5_CH = 0x10;
        /// PWM4/5 交错输出使能
        const PWM4_5_STAG_EN = 0x08;
        /// PWM 数据宽度模式：00=8 位，01=7 位，10=6 位，11=16 位
        const CYC_MOD_MASK = 0x06;
        /// PWM 周期选择：0=256/128/64/32 时钟，1=255/127/63/31 时钟
        const CYCLE_SEL = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DMA_CTRL: u8 {
        /// DMA 通道选择
        const DMA_SEL = 0x04;
        /// DMA 模式：1=循环，0=非循环
        const DMA_ADDR_LOOP = 0x02;
        /// DMA 使能，仅 16 位数据
        const DMA_ENABLE = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_EN: u8 {
        /// FIFO 溢出中断使能
        const IE_OVER = 0x40;
        /// DMA 传输结束中断使能
        const IE_DMA = 0x20;
        /// FIFO 计数小于 4 中断使能
        const IE_FIFO = 0x10;
        /// PWM4/5 周期结束中断使能
        const PWM1_IE_CYC = 0x04;
        /// PWM 周期中断点选择
        const CYC_PRE = 0x02;
        /// PWM1/2/3 周期结束中断使能
        const IE_CYC = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FLAG: u8 {
        /// FIFO 溢出，写 1 清除
        const IF_OVER = 0x10;
        /// DMA 传输结束，写 1 清除
        const IF_DMA = 0x08;
        /// FIFO 计数小于 4，写 1 清除
        const IF_FIFO = 0x04;
        /// PWM4/5 周期结束，写 1 清除
        const PWM1_IF_CYC = 0x02;
        /// PWM1/2/3 周期结束，写 1 清除
        const IF_CYC = 0x01;
    }
}

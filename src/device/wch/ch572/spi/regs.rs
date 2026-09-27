use bitflags::bitflags;

use crate::register::{Reg, Reg8, Reg16, Reg32, Reserved};

/// SPI 寄存器组，基地址 `0x40004000`
#[derive(Debug)]
#[repr(C)]
pub struct SPI {
    /// SPI 模式控制寄存器
    pub CTRL_MOD: Reg<CTRL_MOD>, // 0x00

    /// SPI 配置控制寄存器
    pub CTRL_CFG: Reg<CTRL_CFG>, // 0x01

    /// SPI 中断使能寄存器
    pub INTER_EN: Reg<INTER_EN>, // 0x02

    /// 主机时钟分频寄存器，从机模式下为从机预置值
    pub CLOCK_DIV: Reg8, // 0x03

    /// SPI 数据缓冲寄存器
    pub BUFFER: Reg8, // 0x04

    /// SPI 工作标志寄存器
    pub RUN_FLAG: Reg<RUN_FLAG>, // 0x05

    /// SPI 中断标志寄存器，写 1 清除
    pub INT_FLAG: Reg<INT_FLAG>, // 0x06

    /// SPI FIFO 计数状态寄存器
    pub FIFO_COUNT: Reg8, // 0x07

    /// SPI 中断配置寄存器 1
    pub INTER_CFG1: Reg<INTER_CFG1>, // 0x08

    /// SPI 总字节数寄存器，仅低 12 位有效
    pub TOTAL_CNT: Reg16, // 0x0C

    RESERVED0: Reserved<u16>, // 0x0E

    /// SPI FIFO 寄存器，最高字节为 FIFO 计数状态
    pub FIFO: Reg32, // 0x10

    /// SPI DMA 当前地址
    pub DMA_NOW: Reg16, // 0x14

    RESERVED1: Reserved<u16>, // 0x16

    /// SPI DMA 起始地址
    pub DMA_BEG: Reg16, // 0x18

    RESERVED2: Reserved<u16>, // 0x1A

    /// SPI DMA 结束地址
    pub DMA_END: Reg16, // 0x1C

    RESERVED3: Reserved<u16>, // 0x1E
}

impl SPI {
    /// FIFO 寄存器的低 8 位
    pub const fn FIFO8(&self) -> &Reg8 {
        unsafe { Reg::from_ptr(self.FIFO.as_ptr() as *mut u8) }
    }

    /// `R8_SPI_FIFO_COUNT1`，为 [`SPI::FIFO`] 的最高字节
    pub const fn FIFO_COUNT1(&self) -> &Reg8 {
        unsafe { Reg::from_ptr((self.FIFO.as_ptr() as *mut u8).add(3)) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL_MOD: u8 {
        /// SPI MISO 输出使能
        const MISO_OE = 0x80;
        /// SPI MOSI 输出使能
        const MOSI_OE = 0x40;
        /// SPI SCK 输出使能
        const SCK_OE = 0x20;
        /// SPI FIFO 方向：0=主机模式写，1=主机模式读
        const FIFO_DIR = 0x10;
        /// 从机命令模式：0=字节流，1=首字节为命令；主机时钟模式：0=mode 0，1=mode 3
        const SLV_CMD_MOD = 0x08;
        /// 从机 2 线模式使能：0=3 线，1=2 线
        const TWO_WIRE_MOD = 0x04;
        /// 强制清除 SPI FIFO 和计数
        const ALL_CLEAR = 0x02;
        /// SPI 从机模式：0=主机，1=从机
        const MODE_SLAVE = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL_CFG: u8 {
        /// 主机输入延迟使能
        const MST_DLY_EN = 0x40;
        /// 位序：0=MSB 在前，1=LSB 在前
        const BIT_ORDER = 0x20;
        /// 访问缓冲/FIFO 时自动清除字节交换中断标志
        const AUTO_IF = 0x10;
        /// SPI DMA 地址循环使能
        const DMA_LOOP = 0x04;
        /// hclk 极性反转
        const MST_CLK_SEL = 0x02;
        /// SPI DMA 使能
        const DMA_ENABLE = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTER_EN: u8 {
        /// 从机模式首字节接收中断使能
        const IE_FST_BYTE = 0x80;
        /// SPI FIFO 溢出中断使能
        const IE_FIFO_OV = 0x10;
        /// SPI DMA 完成中断使能
        const IE_DMA_END = 0x08;
        /// SPI FIFO 半满中断使能
        const IE_FIFO_HF = 0x04;
        /// SPI 字节交换中断使能
        const IE_BYTE_END = 0x02;
        /// SPI 总字节数结束中断使能
        const IE_CNT_END = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RUN_FLAG: u8 {
        /// 从机被选中状态
        const SLV_SELECT = 0x80;
        /// 从机片选加载状态
        const SLV_CS_LOAD = 0x40;
        /// SPI FIFO 就绪状态
        const FIFO_READY = 0x20;
        /// 从机首字节/命令标志
        const SLV_CMD_ACT = 0x10;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FLAG: u8 {
        /// 从机模式首字节接收，写 1 清除
        const IF_FST_BYTE = 0x80;
        /// 当前 SPI 空闲状态
        const FREE = 0x40;
        /// SPI FIFO 溢出，写 1 清除
        const IF_FIFO_OV = 0x10;
        /// SPI DMA 完成，写 1 清除
        const IF_DMA_END = 0x08;
        /// SPI FIFO 半满，写 1 清除
        const IF_FIFO_HF = 0x04;
        /// SPI 字节交换，写 1 清除
        const IF_BYTE_END = 0x02;
        /// SPI 总字节数结束，写 1 清除
        const IF_CNT_END = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTER_CFG1: u32 {
        /// FIFO 满中断标志
        const IF_FIFO_FULL = 0x020000;
        /// FIFO 空中断标志
        const IF_FIFO_EMPTY = 0x010000;
        /// FIFO 满中断使能
        const IE_FIFO_FULL = 0x0200;
        /// FIFO 空中断使能
        const IE_FIFO_EMPTY = 0x0100;
        /// 中断触发模式选择：1=边沿，0=电平
        const INT_TYPE_MASK = 0x001F;
    }
}

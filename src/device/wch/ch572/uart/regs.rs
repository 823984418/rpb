use bitflags::bitflags;

use crate::register::{Reg, Reg16, Reg8};

/// UART 寄存器组，基地址 `0x40003400`
#[derive(Debug)]
#[repr(C)]
pub struct UART {
    /// 调制解调器控制寄存器
    pub MCR: Reg<MCR>, // 0x00

    /// 中断使能寄存器
    pub IER: Reg<IER>, // 0x01

    /// FIFO 控制寄存器
    pub FCR: Reg<FCR>, // 0x02

    /// 线路控制寄存器
    pub LCR: Reg<LCR>, // 0x03

    /// 中断识别寄存器
    pub IIR: Reg<IIR>, // 0x04

    /// 线路状态寄存器
    pub LSR: Reg<LSR>, // 0x05

    RESERVED0: [Reg8; 2], // 0x06 ~ 0x07

    /// 接收缓冲寄存器，写访问见 [`UART::THR`]
    pub RBR: Reg8, // 0x08

    RESERVED1: Reg8, // 0x09

    /// 接收 FIFO 计数寄存器
    pub RFC: Reg8, // 0x0A

    /// 发送 FIFO 计数寄存器
    pub TFC: Reg8, // 0x0B

    /// 波特率除数锁存器低位
    pub DLL: Reg8, // 0x0C

    /// 波特率除数锁存器高位
    pub DLM: Reg8, // 0x0D

    /// 波特率预分频锁存器，仅低 7 位有效
    pub DIV: Reg<DIV>, // 0x0E

    /// 地址寄存器
    pub ADR: Reg8, // 0x0F
}

impl UART {
    /// `R8_UART_THR`，发送保持寄存器，与 [`UART::RBR`] 同地址
    pub const fn THR(&self) -> &Reg8 {
        unsafe { Reg::from_ptr(self.RBR.as_ptr()) }
    }

    /// `R16_UART_DL`，波特率除数锁存器，与 [`UART::DLL`] 同地址
    pub const fn DL(&self) -> &Reg16 {
        unsafe { Reg::from_ptr(self.DLL.as_ptr() as *mut u16) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MCR: u8 {
        /// UART 控制 OUT2，同时为中断输出使能
        const OUT2 = 0x08;
        /// UART 中断输出使能
        const INT_OE = 0x08;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u8 {
        /// UART TXD 引脚使能
        const TXD_EN = 0x40;
        /// 接收线路状态中断使能
        const LINE_STAT = 0x04;
        /// 发送保持寄存器空中断使能
        const THR_EMPTY = 0x02;
        /// 接收数据就绪中断使能
        const RECV_RDY = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FCR: u8 {
        /// 接收 FIFO 触发等级：00=1 字节，01=2 字节，10=4 字节，11=7 字节
        const FIFO_TRIG_MASK = 0xC0;
        /// FIFO 使能
        const FIFO_EN = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LCR: u8 {
        /// 除数锁存访问位，同时为通用位
        const DLAB = 0x80;
        /// 通用位
        const GP_BIT = 0x80;
        /// 间隔控制使能
        const BREAK_EN = 0x40;
        /// 校验模式：00=奇，01=偶，10=Mark，11=Space
        const PAR_MOD_MASK = 0x30;
        /// 校验使能
        const PAR_EN = 0x08;
        /// 停止位长度：0=1 位，1=2 位
        const STOP_BIT = 0x04;
        /// 字长：00=5 位，01=6 位，10=7 位，11=8 位
        const WORD_SZ_MASK = 0x03;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IIR: u8 {
        /// FIFO 使能标志
        const FIFO_ID_MASK = 0xC0;
        /// 中断标志位掩码
        const INT_MASK = 0x0F;
        /// 0=有中断，1=无中断
        const NO_INT = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LSR: u8 {
        /// 接收 FIFO 中存在错误
        const ERR_RX_FIFO = 0x80;
        /// 发送器全部为空
        const TX_ALL_EMP = 0x40;
        /// 发送 FIFO 为空
        const TX_FIFO_EMP = 0x20;
        /// 接收间隔错误
        const BREAK_ERR = 0x10;
        /// 接收帧错误
        const FRAME_ERR = 0x08;
        /// 接收校验错误
        const PAR_ERR = 0x04;
        /// 接收溢出错误
        const OVER_ERR = 0x02;
        /// 接收 FIFO 数据就绪
        const DATA_RDY = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DIV: u8 {
        /// 预分频值，取值 1 ~ 128
        const DIV_MASK = 0x7F;
    }
}

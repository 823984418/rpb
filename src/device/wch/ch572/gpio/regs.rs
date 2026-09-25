use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// GPIO 中断控制寄存器组，基地址 `0x40001090`
#[derive(Debug)]
#[repr(C)]
pub struct GPIO_INT {
    /// GPIO 中断使能寄存器
    pub EN: Reg<PA_PINS>, // 0x00

    /// GPIO 中断模式寄存器：0=电平触发，1=边沿触发
    pub MODE: Reg<PA_PINS>, // 0x04

    RESERVED0: Reg32, // 0x08 ~ 0x0B

    /// GPIO 中断标志寄存器，写 1 清除
    pub IF: Reg<PA_PINS>, // 0x0C
}

impl GPIO_INT {
    /// `R16_PA_INT_EDGE_TYPE`，为 [`GPIO_INT::MODE`] 的高 16 位
    ///
    /// 每一位对应一个引脚：0=按 POR 触发，1=不按 POR 触发
    pub const fn EDGE_TYPE(&self) -> &Reg<PA_INT_EDGE> {
        unsafe { Reg::from_ptr((self.MODE.as_ptr() as *mut u8).add(2) as *mut PA_INT_EDGE) }
    }
}

/// GPIO PA 寄存器组，基地址 `0x400010A0`
#[derive(Debug)]
#[repr(C)]
pub struct GPIO {
    /// PA 输入输出方向寄存器：0=输入，1=输出
    pub DIR: Reg<PA_PINS>, // 0x00

    /// PA 输入寄存器
    pub PIN: Reg<PA_PINS>, // 0x04

    /// PA 输出寄存器
    pub OUT: Reg<PA_PINS>, // 0x08

    /// PA 输出清零寄存器：0=保持，1=清零
    pub CLR: Reg<PA_PINS>, // 0x0C

    /// PA 上拉电阻使能寄存器
    pub PU: Reg<PA_PINS>, // 0x10

    /// PA 输入下拉或输出驱动能力寄存器
    pub PD_DRV: Reg<PA_PINS>, // 0x14

    /// PA 输出置位寄存器：1=输出高，0=保持
    pub SET: Reg<PA_PINS>, // 0x18
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PA_PINS: u32 {
        const PIN11 = 1 << 11;
        const PIN10 = 1 << 10;
        const PIN9 = 1 << 9;
        const PIN8 = 1 << 8;
        const PIN7 = 1 << 7;
        const PIN6 = 1 << 6;
        const PIN5 = 1 << 5;
        const PIN4 = 1 << 4;
        const PIN3 = 1 << 3;
        const PIN2 = 1 << 2;
        const PIN1 = 1 << 1;
        const PIN0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PA_INT_EDGE: u16 {
        const PIN11 = 1 << 11;
        const PIN10 = 1 << 10;
        const PIN9 = 1 << 9;
        const PIN8 = 1 << 8;
        const PIN7 = 1 << 7;
        const PIN6 = 1 << 6;
        const PIN5 = 1 << 5;
        const PIN4 = 1 << 4;
        const PIN3 = 1 << 3;
        const PIN2 = 1 << 2;
        const PIN1 = 1 << 1;
        const PIN0 = 1 << 0;
    }
}

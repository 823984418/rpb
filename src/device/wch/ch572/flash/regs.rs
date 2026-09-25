use bitflags::bitflags;

use crate::register::{Reg, Reg8, Reg32};

/// Flash ROM 控制寄存器组，基地址 `0x40001800`
#[derive(Debug)]
#[repr(C)]
pub struct FLASH {
    /// Flash ROM 数据寄存器
    pub DATA: Reg32, // 0x00

    /// Flash ROM 数据缓冲寄存器
    pub CTRL_DATA: Reg8, // 0x04

    /// Flash ROM SCK 时序配置寄存器
    pub SCK: Reg<SCK>, // 0x05

    /// Flash ROM 访问控制寄存器
    pub CTRL: Reg<CTRL>, // 0x06

    /// Flash ROM 访问配置寄存器
    pub CFG: Reg<CFG>, // 0x07
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SCK: u8 {
        /// Flash ROM SCK 时序配置
        const SCK_CFG_MASK = 0x3F;
        /// 50MHz 系统时钟下的 SCK 时序配置
        const SCK_CFG_50M = 1 << 4;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL: u8 {
        /// 关闭 Flash ROM
        const ROM_OFF = 0x04;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFG: u8 {
        /// PLL 分频模式下的取值
        const MODE_PLL_DIV = 0x01;
        /// 32MHz 分频模式下的取值
        const MODE_CK32M_DIV = 0x02;
        /// 32MHz 直连模式下的取值
        const MODE_CK32M_DIRECT = 0x07;
    }
}

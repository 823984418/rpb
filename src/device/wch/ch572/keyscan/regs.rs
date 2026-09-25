use bitflags::bitflags;

use crate::register::Reg;

/// 键盘扫描寄存器组，基地址 `0x40001064`
#[derive(Debug)]
#[repr(C)]
pub struct KEYSCAN {
    /// 键盘扫描控制寄存器
    pub CTRL: Reg<CTRL>, // 0x00

    /// 键盘扫描中断使能寄存器
    pub INT_EN: Reg<INT_EN>, // 0x02

    /// 键盘扫描中断标志寄存器
    pub INT_FLAG: Reg<INT_FLAG>, // 0x03

    /// 当前扫描键值寄存器
    pub NUMB: Reg<NUMB>, // 0x04
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL: u16 {
        /// 芯片唤醒后清除唤醒信号
        const CLR_WAKEUP_EN = 1 << 14;
        /// 一轮键盘扫描结束后唤醒芯片
        const SCAN_1END_WAKE_EN = 1 << 13;
        /// 选择可被扫描的引脚
        const PIN_SCAN_EN_MASK = 0x1F00;
        /// 扫描时钟分频值
        const SCAN_CLK_DIV_MASK = 0x00F0;
        /// 相同键值的确认次数
        const SCAN_CNT_END_MASK = 0x000E;
        /// 启动键盘扫描
        const SCAN_START_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_EN: u8 {
        /// 一轮键盘扫描结束中断使能
        const SCAN_1END_IE = 1 << 1;
        /// 检测到按键按下中断使能
        const KEY_PRESSED_IE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FLAG: u8 {
        /// 一轮键盘扫描结束标志，写 1 清除
        const SCAN_1END_IF = 1 << 1;
        /// 检测到按键按下标志，写 1 清除
        const KEY_PRESSED_IF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NUMB: u32 {
        /// 当前扫描按键的次数
        const KEY_SCAN_CNT_MASK = 0x700000;
        /// 当前扫描键值
        const KEY_SCAN_NUMB_MASK = 0x0FFFFF;
    }
}

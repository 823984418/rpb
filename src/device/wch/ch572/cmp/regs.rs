use bitflags::bitflags;

use crate::register::Reg;

/// 比较器寄存器组，基地址 `0x40001054`
#[derive(Debug)]
#[repr(C)]
pub struct CMP {
    /// 比较器 0 配置寄存器
    pub CTRL_0: Reg<CTRL_0>, // 0x00

    /// 比较器 1 配置寄存器
    pub CTRL_1: Reg<CTRL_1>, // 0x01

    /// 比较器 2 配置寄存器
    pub CTRL_2: Reg<CTRL_2>, // 0x02

    /// 比较器 3 配置寄存器
    pub CTRL_3: Reg<CTRL_3>, // 0x03
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL_0: u8 {
        /// 负端 Vref 选择：1111=800mV，0000=50mV
        const NREF_LEVEL_MASK = 0xF0;
        /// 比较器输入选择：[1]=负端，1=PA7，0=PA3；[0]=正端，1=COMP_VREF，0=PA2
        const SW_MASK = 0x0C;
        /// 将比较器输出连接到 TIM 捕获输入
        const CAP = 1 << 1;
        /// 比较器使能
        const EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL_1: u8 {
        /// 比较器输出选择：11=上升沿，10=下降沿，01=低电平，00=高电平
        const OUT_SEL_MASK = 0x0C;
        /// 比较器中断使能
        const IE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL_2: u8 {
        /// 比较器中断标志，写 1 清除
        const IF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL_3: u8 {
        /// 比较器当前真实信号
        const REAL_SIG = 1 << 0;
        /// 比较器当前输出信号
        const APR_OUT_CMP = 1 << 1;
    }
}

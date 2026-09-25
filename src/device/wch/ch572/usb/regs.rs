use bitflags::bitflags;

use crate::register::{Reg, Reg16, Reg32, Reg8};

/// USB 寄存器组，基地址 `0x40008000`
#[derive(Debug)]
#[repr(C)]
pub struct USB {
    /// USB 基础控制寄存器
    pub CTRL: Reg<CTRL>, // 0x00

    /// USB 设备物理端口控制寄存器，主机模式下为 `UHOST_CTRL`
    pub DEV_CTRL: Reg<DEV_CTRL>, // 0x01

    /// USB 中断使能寄存器
    pub INT_EN: Reg<INT_EN>, // 0x02

    /// USB 设备地址寄存器
    pub DEV_AD: Reg<DEV_AD>, // 0x03

    RESERVED0: Reg8, // 0x04

    /// USB 杂项状态寄存器
    pub MIS_ST: Reg<MIS_ST>, // 0x05

    /// USB 中断标志寄存器，写 1 清除
    pub INT_FG: Reg<INT_FG>, // 0x06

    /// USB 中断状态寄存器
    pub INT_ST: Reg<INT_ST>, // 0x07

    /// USB 接收长度寄存器
    pub RX_LEN: Reg8, // 0x08

    RESERVED1: [Reg8; 3], // 0x09 ~ 0x0B

    /// 端点 4/1 模式寄存器
    pub UEP4_1_MOD: Reg<UEP4_1_MOD>, // 0x0C

    /// 端点 2/3 模式寄存器，主机模式下见 [`USB::UH_EP_MOD`]
    pub UEP2_3_MOD: Reg<UEP2_3_MOD>, // 0x0D

    /// 端点 5/6/7 模式寄存器
    pub UEP567_MOD: Reg<UEP567_MOD>, // 0x0E

    RESERVED2: Reg8, // 0x0F

    /// 端点 0 DMA 缓冲地址
    pub UEP0_DMA: Reg16, // 0x10

    RESERVED3: Reg16, // 0x12

    /// 端点 1 DMA 缓冲地址
    pub UEP1_DMA: Reg16, // 0x14

    RESERVED4: Reg16, // 0x16

    /// 端点 2 DMA 缓冲地址，主机模式下为 `UH_RX_DMA`
    pub UEP2_DMA: Reg16, // 0x18

    RESERVED5: Reg16, // 0x1A

    /// 端点 3 DMA 缓冲地址，主机模式下为 `UH_TX_DMA`
    pub UEP3_DMA: Reg16, // 0x1C

    RESERVED6: Reg16, // 0x1E

    /// 端点 0 发送长度寄存器
    pub UEP0_T_LEN: Reg8, // 0x20

    RESERVED7: Reg8, // 0x21

    /// 端点 0 控制寄存器
    pub UEP0_CTRL: Reg<UEP_CTRL>, // 0x22

    RESERVED8: Reg8, // 0x23

    /// 端点 1 发送长度寄存器
    pub UEP1_T_LEN: Reg8, // 0x24

    RESERVED9: Reg8, // 0x25

    /// 端点 1 控制寄存器，主机模式下见 [`USB::UH_SETUP`]
    pub UEP1_CTRL: Reg<UEP_CTRL>, // 0x26

    RESERVED10: Reg8, // 0x27

    /// 端点 2 发送长度寄存器，主机模式下为 `UH_EP_PID`
    pub UEP2_T_LEN: Reg8, // 0x28

    RESERVED11: Reg8, // 0x29

    /// 端点 2 控制寄存器，主机模式下见 [`USB::UH_RX_CTRL`]
    pub UEP2_CTRL: Reg<UEP_CTRL>, // 0x2A

    RESERVED12: Reg8, // 0x2B

    /// 端点 3 发送长度寄存器，主机模式下为 `UH_TX_LEN`
    pub UEP3_T_LEN: Reg8, // 0x2C

    RESERVED13: Reg8, // 0x2D

    /// 端点 3 控制寄存器，主机模式下见 [`USB::UH_TX_CTRL`]
    pub UEP3_CTRL: Reg<UEP_CTRL>, // 0x2E

    RESERVED14: Reg8, // 0x2F

    /// 端点 4 发送长度寄存器
    pub UEP4_T_LEN: Reg8, // 0x30

    RESERVED15: Reg8, // 0x31

    /// 端点 4 控制寄存器
    pub UEP4_CTRL: Reg<UEP_CTRL>, // 0x32

    RESERVED16: [Reg8; 0x21], // 0x33 ~ 0x53

    /// 端点 5 DMA 缓冲地址
    pub UEP5_DMA: Reg16, // 0x54

    RESERVED17: Reg16, // 0x56

    /// 端点 6 DMA 缓冲地址
    pub UEP6_DMA: Reg16, // 0x58

    RESERVED18: Reg16, // 0x5A

    /// 端点 7 DMA 缓冲地址
    pub UEP7_DMA: Reg16, // 0x5C

    RESERVED19: Reg16, // 0x5E

    RESERVED20: Reg32, // 0x60 ~ 0x63

    /// 端点 5 发送长度寄存器
    pub UEP5_T_LEN: Reg8, // 0x64

    RESERVED21: Reg8, // 0x65

    /// 端点 5 控制寄存器
    pub UEP5_CTRL: Reg<UEP_CTRL>, // 0x66

    RESERVED22: Reg8, // 0x67

    /// 端点 6 发送长度寄存器
    pub UEP6_T_LEN: Reg8, // 0x68

    RESERVED23: Reg8, // 0x69

    /// 端点 6 控制寄存器
    pub UEP6_CTRL: Reg<UEP_CTRL>, // 0x6A

    RESERVED24: Reg8, // 0x6B

    /// 端点 7 发送长度寄存器
    pub UEP7_T_LEN: Reg8, // 0x6C

    RESERVED25: Reg8, // 0x6D

    /// 端点 7 控制寄存器
    pub UEP7_CTRL: Reg<UEP_CTRL>, // 0x6E

    RESERVED26: Reg8, // 0x6F
}

impl USB {
    /// `R8_UHOST_CTRL`，主机物理端口控制，与 [`USB::DEV_CTRL`] 同地址
    pub const fn UHOST_CTRL(&self) -> &Reg<UHOST_CTRL> {
        unsafe { Reg::from_ptr(self.DEV_CTRL.as_ptr() as *mut UHOST_CTRL) }
    }

    /// `R8_UH_EP_MOD`，主机端点模式，与 [`USB::UEP2_3_MOD`] 同地址
    pub const fn UH_EP_MOD(&self) -> &Reg<UH_EP_MOD> {
        unsafe { Reg::from_ptr(self.UEP2_3_MOD.as_ptr() as *mut UH_EP_MOD) }
    }

    /// `R8_UH_SETUP`，主机辅助 SETUP，与 [`USB::UEP1_CTRL`] 同地址
    pub const fn UH_SETUP(&self) -> &Reg<UH_SETUP> {
        unsafe { Reg::from_ptr(self.UEP1_CTRL.as_ptr() as *mut UH_SETUP) }
    }

    /// `R8_UH_EP_PID`，主机端点与 PID，与 [`USB::UEP2_T_LEN`] 同地址
    pub const fn UH_EP_PID(&self) -> &Reg<UH_EP_PID> {
        unsafe { Reg::from_ptr(self.UEP2_T_LEN.as_ptr() as *mut UH_EP_PID) }
    }

    /// `R8_UH_RX_CTRL`，主机接收端点控制，与 [`USB::UEP2_CTRL`] 同地址
    pub const fn UH_RX_CTRL(&self) -> &Reg<UH_RX_CTRL> {
        unsafe { Reg::from_ptr(self.UEP2_CTRL.as_ptr() as *mut UH_RX_CTRL) }
    }

    /// `R8_UH_TX_CTRL`，主机发送端点控制，与 [`USB::UEP3_CTRL`] 同地址
    pub const fn UH_TX_CTRL(&self) -> &Reg<UH_TX_CTRL> {
        unsafe { Reg::from_ptr(self.UEP3_CTRL.as_ptr() as *mut UH_TX_CTRL) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL: u8 {
        /// 主机模式使能：0=设备模式，1=主机模式
        const HOST_MODE = 1 << 7;
        /// 低速使能：0=12Mbps，1=1.5Mbps
        const LOW_SPEED = 1 << 6;
        /// USB 设备使能与内部上拉电阻使能
        const DEV_PU_EN = 1 << 5;
        /// USB 系统控制位
        const SYS_CTRL_MASK = 0x30;
        /// USB 系统控制低位
        const SYS_CTRL0 = 1 << 4;
        /// 自动响应忙使能
        const INT_BUSY = 1 << 3;
        /// 强制复位 USB SIE，需软件清除
        const RESET_SIE = 1 << 2;
        /// 强制清除 USB FIFO 和计数
        const CLR_ALL = 1 << 1;
        /// USB DMA 使能和 DMA 中断使能
        const DMA_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEV_CTRL: u8 {
        /// 禁止 USB UDP/UDM 下拉电阻
        const PD_DIS = 1 << 7;
        /// 当前 UDP 引脚电平
        const DP_PIN = 1 << 5;
        /// 当前 UDM 引脚电平
        const DM_PIN = 1 << 4;
        /// USB 物理端口低速使能：0=全速，1=低速
        const LOW_SPEED = 1 << 2;
        /// 通用位
        const GP_BIT = 1 << 1;
        /// USB 物理端口 I/O 使能
        const PORT_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UHOST_CTRL: u8 {
        /// 禁止 USB UDP/UDM 下拉电阻
        const PD_DIS = 1 << 7;
        /// 当前 UDP 引脚电平
        const DP_PIN = 1 << 5;
        /// 当前 UDM 引脚电平
        const DM_PIN = 1 << 4;
        /// USB 端口低速使能：0=全速，1=低速
        const LOW_SPEED = 1 << 2;
        /// 控制 USB 总线复位：0=正常，1=强制复位
        const BUS_RESET = 1 << 1;
        /// USB 端口使能，设备拔出后自动关闭
        const PORT_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_EN: u8 {
        /// 设备模式 SOF 接收中断使能
        const DEV_SOF = 1 << 7;
        /// 设备模式 NAK 应答中断使能
        const DEV_NAK = 1 << 6;
        /// 单线模式使能
        const MOD_1_WIRE = 1 << 5;
        /// FIFO 溢出中断使能
        const FIFO_OV = 1 << 4;
        /// 主机模式 SOF 定时器动作中断使能
        const HST_SOF = 1 << 3;
        /// USB 挂起或恢复事件中断使能
        const SUSPEND = 1 << 2;
        /// USB 传输完成中断使能
        const TRANSFER = 1 << 1;
        /// 主机模式设备检测、设备模式总线复位中断使能
        const DETECT = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DEV_AD: u8 {
        /// 通用位
        const GP_BIT = 1 << 7;
        /// USB 设备地址
        const ADDR_MASK = 0x7F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MIS_ST: u8 {
        /// 主机 SOF 定时器预置状态
        const SOF_PRES = 1 << 7;
        /// 主机 SOF 定时器动作状态
        const SOF_ACT = 1 << 6;
        /// USB SIE 空闲状态
        const SIE_FREE = 1 << 5;
        /// 接收 FIFO 就绪（非空）状态
        const R_FIFO_RDY = 1 << 4;
        /// USB 总线复位状态
        const BUS_RESET = 1 << 3;
        /// USB 挂起状态
        const SUSPEND = 1 << 2;
        /// 设备连接到主机时保存的 UDM 电平
        const DM_LEVEL = 1 << 1;
        /// 主机模式下设备连接状态
        const DEV_ATTACH = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_FG: u8 {
        /// 当前 USB 传输收到 NAK
        const IS_NAK = 1 << 7;
        /// 当前 USB 传输 toggle 正确
        const TOG_OK = 1 << 6;
        /// USB SIE 空闲状态
        const SIE_FREE = 1 << 5;
        /// USB FIFO 溢出，写 1 清除
        const FIFO_OV = 1 << 4;
        /// 主机 SOF 定时器中断，写 1 清除
        const HST_SOF = 1 << 3;
        /// USB 挂起或恢复事件中断，写 1 清除
        const SUSPEND = 1 << 2;
        /// USB 传输完成中断，写 1 清除
        const TRANSFER = 1 << 1;
        /// 主机模式设备检测、设备模式总线复位中断，写 1 清除
        const DETECT = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_ST: u8 {
        /// 设备模式收到 SETUP 令牌及 8 字节 SETUP 请求
        const SETUP_ACT = 1 << 7;
        /// 当前 USB 传输 toggle 正确
        const TOG_OK = 1 << 6;
        /// 当前令牌 PID bit1
        const TOKEN1 = 1 << 5;
        /// 当前令牌 PID bit0
        const TOKEN0 = 1 << 4;
        /// 当前令牌 PID 掩码：00=OUT，01=SOF，10=IN，11=空闲
        const TOKEN_MASK = 0x30;
        /// 设备模式当前传输端点号；主机模式下为握手响应掩码
        const ENDP_MASK = 0x0F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UEP4_1_MOD: u8 {
        /// 端点 1 接收（OUT）使能
        const UEP1_RX_EN = 1 << 7;
        /// 端点 1 发送（IN）使能
        const UEP1_TX_EN = 1 << 6;
        /// 端点 1 缓冲模式
        const UEP1_BUF_MOD = 1 << 4;
        /// 端点 4 接收（OUT）使能
        const UEP4_RX_EN = 1 << 3;
        /// 端点 4 发送（IN）使能
        const UEP4_TX_EN = 1 << 2;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UEP2_3_MOD: u8 {
        /// 端点 3 接收（OUT）使能
        const UEP3_RX_EN = 1 << 7;
        /// 端点 3 发送（IN）使能
        const UEP3_TX_EN = 1 << 6;
        /// 端点 3 缓冲模式
        const UEP3_BUF_MOD = 1 << 4;
        /// 端点 2 接收（OUT）使能
        const UEP2_RX_EN = 1 << 3;
        /// 端点 2 发送（IN）使能
        const UEP2_TX_EN = 1 << 2;
        /// 端点 2 缓冲模式
        const UEP2_BUF_MOD = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UEP567_MOD: u8 {
        /// 端点 7 接收（OUT）使能
        const UEP7_RX_EN = 1 << 5;
        /// 端点 7 发送（IN）使能
        const UEP7_TX_EN = 1 << 4;
        /// 端点 6 接收（OUT）使能
        const UEP6_RX_EN = 1 << 3;
        /// 端点 6 发送（IN）使能
        const UEP6_TX_EN = 1 << 2;
        /// 端点 5 接收（OUT）使能
        const UEP5_RX_EN = 1 << 1;
        /// 端点 5 发送（IN）使能
        const UEP5_TX_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UEP_CTRL: u8 {
        /// 接收（OUT）期望数据 toggle：0=DATA0，1=DATA1
        const R_TOG = 1 << 7;
        /// 发送（IN）准备数据 toggle：0=DATA0，1=DATA1
        const T_TOG = 1 << 6;
        /// 端点 1/2/3 传输完成后自动翻转 toggle
        const AUTO_TOG = 1 << 4;
        /// 接收（OUT）握手响应高位
        const R_RES1 = 1 << 3;
        /// 接收（OUT）握手响应低位
        const R_RES0 = 1 << 2;
        /// 接收（OUT）握手响应掩码：00=ACK，01=无响应，10=NAK，11=STALL
        const R_RES_MASK = 0x0C;
        /// 发送（IN）握手响应高位
        const T_RES1 = 1 << 1;
        /// 发送（IN）握手响应低位
        const T_RES0 = 1 << 0;
        /// 发送（IN）握手响应掩码：00=ACK，01=无响应，10=NAK，11=STALL
        const T_RES_MASK = 0x03;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UH_EP_MOD: u8 {
        /// 主机 OUT 端点发送使能
        const EP_TX_EN = 1 << 6;
        /// 主机 OUT 端点缓冲模式
        const EP_TBUF_MOD = 1 << 4;
        /// 主机 IN 端点接收使能
        const EP_RX_EN = 1 << 3;
        /// 主机 IN 端点缓冲模式
        const EP_RBUF_MOD = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UH_SETUP: u8 {
        /// 低速设备经 hub 的 PRE PID 使能
        const PRE_PID_EN = 1 << 7;
        /// 主机自动 SOF 使能
        const SOF_EN = 1 << 6;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UH_EP_PID: u8 {
        /// 主机传输令牌 PID 掩码
        const TOKEN_MASK = 0xF0;
        /// 主机传输端点号掩码
        const ENDP_MASK = 0x0F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UH_RX_CTRL: u8 {
        /// 主机接收（IN）期望数据 toggle：0=DATA0，1=DATA1
        const R_TOG = 1 << 7;
        /// 传输完成后自动翻转 toggle
        const R_AUTO_TOG = 1 << 4;
        /// 主机接收（IN）握手响应：0=ACK，1=无响应
        const R_RES = 1 << 2;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct UH_TX_CTRL: u8 {
        /// 主机发送（SETUP/OUT）准备数据 toggle：0=DATA0，1=DATA1
        const T_TOG = 1 << 6;
        /// 传输完成后自动翻转 toggle
        const T_AUTO_TOG = 1 << 4;
        /// 主机发送（SETUP/OUT）期望握手响应：0=ACK，1=无响应
        const T_RES = 1 << 0;
    }
}

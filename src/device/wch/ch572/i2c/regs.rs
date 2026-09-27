use bitflags::bitflags;

use crate::register::{Reg, Reg8, Reg16, Reserved};

/// I2C 寄存器组，基地址 `0x40004800`
#[derive(Debug)]
#[repr(C)]
pub struct I2C {
    /// 控制寄存器 1
    pub CTRL1: Reg<CTRL1>, // 0x00

    RESERVED0: Reserved<u16>, // 0x02

    /// 控制寄存器 2
    pub CTRL2: Reg<CTRL2>, // 0x04

    RESERVED1: Reserved<u16>, // 0x06

    /// 自身地址寄存器 1
    pub OADDR1: Reg<OADDR1>, // 0x08

    RESERVED2: Reserved<u16>, // 0x0A

    /// 自身地址寄存器 2
    pub OADDR2: Reg<OADDR2>, // 0x0C

    RESERVED3: Reserved<u16>, // 0x0E

    /// 数据寄存器
    pub DATAR: Reg16, // 0x10

    RESERVED4: Reserved<u16>, // 0x12

    /// 状态寄存器 1
    pub STAR1: Reg<STAR1>, // 0x14

    RESERVED5: Reserved<u16>, // 0x16

    /// 状态寄存器 2，高字节为 PEC 寄存器
    pub STAR2: Reg<STAR2>, // 0x18

    RESERVED6: Reserved<u16>, // 0x1A

    /// 时钟控制寄存器
    pub CKCFGR: Reg<CKCFGR>, // 0x1C

    RESERVED7: Reserved<u16>, // 0x1E

    /// 上升时间寄存器
    pub RTR: Reg<RTR>, // 0x20

    RESERVED8: Reserved<u16>, // 0x22
}

impl I2C {
    /// `R8_I2C_PEC`，为 [`I2C::STAR2`] 的高字节
    pub const fn PEC(&self) -> &Reg8 {
        unsafe { Reg::from_ptr((self.STAR2.as_ptr() as *mut u8).add(1)) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL1: u16 {
        /// 软件复位
        const SWRST = 0x8000;
        /// SMBus 报警：0=释放 SMBA 引脚为高，1=拉低 SMBA 引脚
        const ALERT = 0x2000;
        /// 数据包错误校验：0=不传输 PEC，1=传输 PEC
        const PEC = 0x1000;
        /// 应答/PEC 位置
        const POS = 0x0800;
        /// 应答使能
        const ACK = 0x0400;
        /// 停止条件产生
        const STOP = 0x0200;
        /// 起始条件产生
        const START = 0x0100;
        /// 从机模式禁止时钟延展
        const NOSTRETCH = 0x0080;
        /// 通用呼叫使能
        const ENGC = 0x0040;
        /// PEC 使能
        const ENPEC = 0x0020;
        /// ARP 使能
        const EBARP = 0x0010;
        /// SMBus 类型：0=Device，1=Host
        const SMBTYPE = 0x0008;
        /// SMBUS 模式：0=I2C，1=SMBUS
        const SMBUS = 0x0002;
        /// 外设使能
        const PE = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTRL2: u16 {
        /// 缓冲中断使能
        const ITBUFEN = 0x0400;
        /// 事件中断使能
        const ITEVTEN = 0x0200;
        /// 错误中断使能
        const ITERREN = 0x0100;
        /// 外设时钟频率，允许 2MHz ~ 36MHz
        const FREQ_MASK = 0x003F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OADDR1: u16 {
        /// 从机地址模式：0=7 位，1=10 位
        const ADDMODE = 0x8000;
        /// 10 位地址模式下的地址 bit[9:8]
        const ADD9_8_MASK = 0x0300;
        /// 地址 bit[7:1]
        const ADD7_1_MASK = 0x00FE;
        /// 10 位地址模式下的地址 bit0
        const ADD0 = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OADDR2: u16 {
        /// 地址 2 的 bit[7:1]
        const ADD2_MASK = 0x00FE;
        /// 双地址模式使能
        const ENDUAL = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STAR1: u16 {
        /// SMBus 报警标志
        const SMBALERT = 0x8000;
        /// 超时或 Tlow 错误标志
        const TIMEOUT = 0x4000;
        /// 接收 PEC 错误标志
        const PECERR = 0x1000;
        /// 溢出/下溢标志
        const OVR = 0x0800;
        /// 应答失败标志
        const AF = 0x0400;
        /// 主机模式仲裁丢失标志
        const ARLO = 0x0200;
        /// 总线错误标志
        const BERR = 0x0100;
        /// 数据寄存器空标志
        const TxE = 0x0080;
        /// 数据寄存器非空标志
        const RxNE = 0x0040;
        /// 从机模式停止位检测标志
        const STOPF = 0x0010;
        /// 主机模式 10 位头已发送标志
        const ADD10 = 0x0008;
        /// 字节传输完成标志
        const BTF = 0x0004;
        /// 地址已发送/已匹配标志
        const ADDR = 0x0002;
        /// 起始位标志
        const SB = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STAR2: u16 {
        /// 数据包错误校验寄存器
        const PECX_MASK = 0xFF00;
        /// 双地址标志：0=匹配 OAR1，1=匹配 OAR2
        const DUALF = 0x0080;
        /// 收到 SMBus 主机头标志
        const SMBHOST = 0x0040;
        /// 收到 SMBus 默认地址标志
        const SMBDEFAULT = 0x0020;
        /// 收到通用呼叫地址标志
        const GENCALL = 0x0010;
        /// 传输方向：0=接收，1=发送
        const TRA = 0x0004;
        /// 总线忙标志
        const BUSY = 0x0002;
        /// 模式状态：0=从机，1=主机
        const MSL = 0x0001;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CKCFGR: u16 {
        /// 主机模式选择：0=标准模式，1=快速模式
        const F_S = 0x8000;
        /// 快速模式占空比：0=L/H=2，1=L/H=16/9
        const DUTY = 0x4000;
        /// 主机模式下控制 SCL 时钟
        const CCR_MASK = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTR: u16 {
        /// 最大上升时间
        const TRISE_MASK = 0x003F;
    }
}

use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

/// FLASH register block.
#[derive(Debug)]
#[repr(C)]
pub struct FLASH {
    pub ACR: Reg<ACR>,
    pub MKYR: Reg32,
    pub E2KYR: Reg32,
    pub SR: Reg<SR>,
    pub CR: Reg<CR>,
    pub CR1: Reg<CR1>,
    pub OPR: Reg<OPR>,
    pub RPR: Reg<RPR>,
    pub WRPR: Reg32,
    pub WRPR1: Reg32,
    pub CNTR: Reg32,
    pub UPCNTR: Reg32,
    pub CNTCR: Reg<CNTCR>,
    pub IKYR: Reg32,
    pub DATA0: Reg32,
    pub DATA1: Reg32,
    pub DATA2: Reg32,
    /// Reserved (0x44..=0xff).
    RESERVED0: Reserved<[u32; 47]>,
    pub MEMRMP: Reg<MEMRMP>,
    /// Reserved (0x104..=0x1ff).
    RESERVED1: Reserved<[u32; 63]>,
    pub OPR_CUST1: Reg<OPR_CUST1>,
    pub OPR_DESI0: Reg<OPR_DESI0>,
    pub OPR_DESI1: Reg<OPR_DESI1>,
    pub OPR_DESI2: Reg<OPR_DESI2>,
    pub OPR_DESI3: Reg<OPR_DESI3>,
}

bitflags! {
    /// `ACR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ACR: u32 {
        const LOCK_VALUE = 0x5AA5 << 16;
        /// 寄存器锁定位
        const LOCK_MASK = 0xFFFF << 16;

        /// 预取缓冲区使能
        const PRFTEN = 1 << 12;

        /// 预取区复位
        const CRST = 1 << 11;

        /// 预取数据BUFF使能
        const DBEN = 1 << 10;

        /// 预取指令BUFF使能
        const IBEN = 1 << 9;

        /// 预取CACHE缓冲区使能
        const CACHEN = 1 << 8;

        const LATENCY_7 = 0b111 << 0;
        const LATENCY_6 = 0b110 << 0;
        const LATENCY_5 = 0b101 << 0;
        const LATENCY_4 = 0b100 << 0;
        const LATENCY_3 = 0b011 << 0;
        const LATENCY_2 = 0b010 << 0;
        const LATENCY_1 = 0b001 << 0;
        const LATENCY_0 = 0b000 << 0;
        /// FLASH访问等待
        const LATENCY_MASK = 0x7 << 0;
    }
}

bitflags! {
    /// `SR` (0xc).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        /// FLASH状态错误标志清除位
        const STAERRC = 1 << 23;

        /// 编程窗口错误标志清除位
        const PGWERRC = 1 << 22;

        /// 编程错误标志清除位
        const PGPERRC = 1 << 21;

        /// 写保护错误标志清除位
        const WRPRTERRC = 1 << 20;

        /// FLASH硬件错误状态清除位
        const FLSERRC = 1 << 19;

        /// FLASH操作错误标志清除位
        const OPERRC = 1 << 17;

        /// 操作执行标志位
        const EOPC = 1 << 16;

        /// 忙标志位
        const BSY = 1 << 15;

        /// FLASH状态错误标志位
        const STAERR = 1 << 7;

        /// 编程窗口错误标志位
        const PGWERR = 1 << 6;

        /// 编程错误标志位
        const PGPERR = 1 << 5;

        /// 写保护错误标志位
        const WRPRTERR = 1 << 4;

        /// FLASH硬件错误
        const FLSERR = 1 << 3;

        /// FLASH操作错误标志位
        const OPERR = 1 << 1;

        /// 操作执行标志位
        const EOP = 1 << 0;
    }
}

bitflags! {
    /// `CR` (0x10).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        /// 主程序区整体擦除
        const CMD_ME0 = 0xAA55 << 16;
        /// 主程序取扇区擦除
        const CMD_MSE = 0xE619 << 16;
        /// 主程序区存储单元编程
        const CMD_MPG = 0x6E91 << 16;
        /// 特殊信息区E2Prom区编程
        const CMD_IPG = 0xB44B << 16;
        /// 特殊信息区E2Prom区按扇区擦除
        const CMD_E2SE = 0x4BB4 << 16;
        /// 特殊信息区Customer Block区编程
        const CMD_CBPG = 0xC33C << 16;
        /// 特殊信息区Customer Block区按扇区擦除
        const CMD_CBSE = 0x3CC3 << 16;
        /// 需要调试模式
        /// 特殊信息区BootRom区编程
        const CMD_BRPG = 0xE11E << 16;
        /// 特殊信息区Protect区编程
        const CMD_SPG = 0xD22D << 16;
        /// 特殊信息区OTP区编程
        const CMD_OPG = 0xF00F << 16;
        /// 需要调试模式
        /// Flash整体擦除（包括代码区，Protect Block,Customer Block, 引导区）
        const CMD_ME1 = 0xBB44 << 16;
        const CMD_MASK = 0xFFFF << 16;

        /// 主程序区锁定标志
        const MNLCK = 1 << 15;

        /// 类EEPROM存储区锁定标志位
        const E2LCK = 1 << 14;

        /// 特殊信息区锁定标志位(代码保护区、客户信息区、OTP区和引导区)
        const INFLCK = 1 << 12;

        /// 烧写操作位宽选择位(仅main区烧写有效)
        const PSIZE_MASK = 0x7 << 9;

        /// 开始标志位
        const STRT = 1 << 8;
    }
}

bitflags! {
    /// `CR1` (0x14).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR1: u32 {
        const SNB_MASK = 0x3ffff << 0;
    }
}

bitflags! {
    /// `OPR` (0x18).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `RPR` (0x1c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RPR: u32 {
        const RDP_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `CNTCR` (0x30).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTCR: u32 {
        const CNTEN = 1 << 0;
    }
}

bitflags! {
    /// `MEMRMP` (0x100).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MEMRMP: u32 {
        const MEMMODE_MASK = 0xffff << 0;
    }
}

bitflags! {
    /// `OPR_CUST1` (0x200).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_CUST1: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `OPR_DESI0` (0x204).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_DESI0: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `OPR_DESI1` (0x208).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_DESI1: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `OPR_DESI2` (0x20c).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_DESI2: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

bitflags! {
    /// `OPR_DESI3` (0x210).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPR_DESI3: u32 {
        const OPT3_MASK = 0xff << 24;

        const OPT2_MASK = 0xff << 16;

        const OPT1_MASK = 0xff << 8;

        const OPT0_MASK = 0xff << 0;
    }
}

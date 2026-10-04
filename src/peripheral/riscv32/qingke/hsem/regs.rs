use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

/// 硬件信号量
/// 这是 WCH 的专用外设
#[derive(Debug)]
#[repr(C)]
pub struct HSEM {
    pub RX: [Reg<RX>; 32],

    RESERVED0: Reserved<[u8; 0x80]>,

    pub RLRX: [Reg<RLRX>; 32],

    RESERVED1: Reserved<[u8; 0x80]>,

    pub LSE: Reg32,

    RESERVED2: Reserved<[u8; 4]>,

    pub CLR: Reg<CLR>,

    pub KEY: Reg<KEY>,

    RESERVED3: Reserved<[u8; 0xF0]>,

    pub IER: Reg32,

    RESERVED4: Reserved<[u8; 4]>,

    pub ISR: Reg32,

    RESERVED5: Reserved<[u8; 4]>,

    pub ISM: Reg32,

    RESERVED6: Reserved<[u8; 4]>,

    pub LSM: Reg32,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RX: u32 {
        const LOCK = 1 << 31;
        const CID = 0x3 << 8;
        const PID = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RLRX: u32 {
        const LOCK = 1 << 31;
        const CID_MASK = 0x3 << 8;
        const PID_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CLR: u32 {
        const CLR_KEY_VALUE = 0x5AA5 << 16;
        const CLR_KEY_MASK = 0xFFFF << 16;

        /// 注意 MASK 并不表示此字段是掩膜，而是字段的固有名称
        /// CID 匹配屏蔽：
        /// 1：屏蔽 CID 匹配，所有 CID 值均解锁；
        /// 0：不屏蔽 CID 匹配，锁定的通道 CID 值与 match_CID 值相同时解锁
        const CID_MASK = 1 << 13;

        /// 注意 MASK 并不表示此字段是掩膜，而是字段的固有名称
        /// PID 匹配屏蔽：
        /// 1：屏蔽 PID 匹配，所有 PID 值均解锁；
        /// 0：不屏蔽 PID 匹配，锁定的通道 PID 值与 match_PID 值相同时解锁
        const PID_MASK = 1 << 12;
        const MATCH_CID_MASK = 0x3 << 8;
        const MATCH_PID_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct KEY: u32 {
        const KEY_VALUE = 0x5AA5 << 16;
        const KEY_VALUE_MASK = 0xFFFF << 16;

        const AUTO_2STEP_CLR = 1 << 1;
        const AUTO_1STEP_CLR = 1 << 0;
    }
}

/// 信号量编号
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HSEM_ID {
    HSEM_ID0 = 0,
    HSEM_ID1,
    HSEM_ID2,
    HSEM_ID3,
    HSEM_ID4,
    HSEM_ID5,
    HSEM_ID6,
    HSEM_ID7,
    HSEM_ID8,
    HSEM_ID9,
    HSEM_ID10,
    HSEM_ID11,
    HSEM_ID12,
    HSEM_ID13,
    HSEM_ID14,
    HSEM_ID15,
    HSEM_ID16,
    HSEM_ID17,
    HSEM_ID18,
    HSEM_ID19,
    HSEM_ID20,
    HSEM_ID21,
    HSEM_ID22,
    HSEM_ID23,
    HSEM_ID24,
    HSEM_ID25,
    HSEM_ID26,
    HSEM_ID27,
    HSEM_ID28,
    HSEM_ID29,
    HSEM_ID30,
    HSEM_ID31,
}

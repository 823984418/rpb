mod regs;

pub use regs::*;

use crate::peripheral::riscv32::csr::csrr;
use crate::peripheral::riscv32::csr::mhartid::MHARTID;

/// 获取当前核 ID（CSR mhartid，0xF14）
///
/// - 0 - V3F
/// - 1 - V5F
#[inline(always)]
fn GetCurrentCoreID() -> MHARTID {
    unsafe { csrr::<0xF14>() }
}

impl HSEM {
    /// 以两步模式获取一个信号量。
    ///
    /// - `HSEM_ID`：信号量编号
    /// - `ProcessID`：进程 ID，0~255
    ///
    /// 返回 `true` 表示获取成功，`false` 表示获取失败。
    pub fn Take(&self, HSEM_ID: HSEM_ID, ProcessID: u32) -> bool {
        let CoreID = GetCurrentCoreID() as u32;
        let value = RX::LOCK
            | RX::from_bits_retain((CoreID << 8) & RX::CID.bits())
            | RX::from_bits_retain(ProcessID & RX::PID.bits());

        self.RX[HSEM_ID as usize].set(value);
        self.RX[HSEM_ID as usize].get() == value
    }

    /// 以一步模式快速获取一个信号量。
    ///
    /// - `HSEM_ID`：信号量编号
    ///
    /// 返回 `true` 表示获取成功，`false` 表示获取失败。
    pub fn FastTake(&self, HSEM_ID: HSEM_ID) -> bool {
        let CoreID = GetCurrentCoreID() as u32;

        self.RLRX[HSEM_ID as usize].get()
            == (RLRX::LOCK | RLRX::from_bits_retain((CoreID << 8) & RLRX::CID_MASK.bits()))
    }

    /// 获取指定信号量是否已被获取。
    ///
    /// - `HSEM_ID`：信号量编号
    ///
    /// 返回 `true` 表示已获取，`false` 表示未获取。
    pub fn GetOneSemTakenState(&self, HSEM_ID: HSEM_ID) -> bool {
        self.RX[HSEM_ID as usize].get().contains(RX::LOCK)
    }

    /// 获取所有信号量是否已被获取。
    pub fn GetAllSemTakenState(&self) -> u32 {
        self.LSE.get()
    }

    /// 获取当前核（V3F 或 V5F）所有信号量是否已被获取。
    pub fn OwnCoreGetAllSemTakenState(&self) -> u32 {
        self.LSM.get()
    }

    /// 释放一个信号量。
    ///
    /// - `HSEM_ID`：信号量编号
    /// - `ProcessID`：进程 ID，0~255
    pub fn ReleaseOneSem(&self, HSEM_ID: HSEM_ID, ProcessID: u32) {
        let CoreID = GetCurrentCoreID() as u32;

        self.RX[HSEM_ID as usize].set(
            RX::from_bits_retain((CoreID << 8) & RX::CID.bits())
                | RX::from_bits_retain(ProcessID & RX::PID.bits()),
        );
    }

    /// 释放所有信号量。
    ///
    /// 同时屏蔽 CID 和 PID 匹配，所有 CID/PID 值均解锁。
    pub fn ReleaseAllSem(&self) {
        self.CLR
            .set(CLR::CLR_KEY_VALUE | CLR::CID_MASK | CLR::PID_MASK);
    }

    /// 释放与 CoreID 和 ProcessID 匹配的信号量。
    ///
    /// 不屏蔽 CID/PID 匹配，锁定通道的 CID 和 PID 分别与 `CoreID`、`ProcessID` 相同时才解锁。
    ///
    /// - `CoreID`：核 ID（`match_CID` 值）
    /// - `ProcessID`：进程 ID，0~255（`match_PID` 值）
    pub fn ReleaseSem_MatchCID_PID(&self, CoreID: u32, ProcessID: u32) {
        self.CLR.set(
            CLR::CLR_KEY_VALUE
                | CLR::from_bits_retain(CoreID & CLR::MATCH_CID_MASK.bits())
                | CLR::from_bits_retain(ProcessID & CLR::MATCH_PID_MASK.bits()),
        );
    }

    /// 释放与 CoreID 匹配的信号量。
    ///
    /// 屏蔽 PID 匹配，所有 PID 值均解锁；CID 与 `CoreID` 相同时解锁。
    ///
    /// - `CoreID`：核 ID（`match_CID` 值）
    pub fn ReleaseSem_MatchCID(&self, CoreID: u32) {
        self.CLR.set(
            CLR::CLR_KEY_VALUE
                | CLR::PID_MASK
                | CLR::from_bits_retain(CoreID & CLR::MATCH_CID_MASK.bits()),
        );
    }

    /// 释放与 ProcessID 匹配的信号量。
    ///
    /// 屏蔽 CID 匹配，所有 CID 值均解锁；PID 与 `ProcessID` 相同时解锁。
    ///
    /// - `ProcessID`：进程 ID，0~255（`match_PID` 值）
    pub fn ReleaseSem_MatchPID(&self, ProcessID: u32) {
        self.CLR.set(
            CLR::CLR_KEY_VALUE
                | CLR::CID_MASK
                | CLR::from_bits_retain(ProcessID & CLR::MATCH_PID_MASK.bits()),
        );
    }

    /// 设置信号量 Key。
    ///
    /// - `Key`：信号量 Key，0~0xFFFF
    pub fn SetClearKey(&self, Key: u16) {
        self.CLR.set(CLR::from_bits_retain((Key as u32) << 16));
    }

    /// 获取信号量 Key。
    ///
    /// 返回 0~0xFFFF 的信号量 Key。
    pub fn GetClearKey(&self) -> u16 {
        (self.KEY.get().bits() >> 16) as u16
    }

    /// 使能或失能指定信号量的中断。
    ///
    /// - `HSEM_ID`：信号量编号
    /// - `NewState`：`true` 使能，`false` 失能
    pub fn ITConfig(&self, HSEM_ID: HSEM_ID, NewState: bool) {
        self.IER.bit_set(1 << HSEM_ID as usize, NewState);
    }

    /// 检查指定信号量标志是否置位。
    ///
    /// - `HSEM_ID`：信号量编号
    ///
    /// 返回 `true` 表示置位，`false` 表示复位。
    pub fn GetFlagStatus(&self, HSEM_ID: HSEM_ID) -> bool {
        (self.ISR.get() & (1 << HSEM_ID as usize)) != 0
    }

    /// 清除指定信号量的挂起标志。
    ///
    /// - `HSEM_ID`：信号量编号
    pub fn ClearFlag(&self, HSEM_ID: HSEM_ID) {
        self.ISR.set(1 << HSEM_ID as usize);
    }

    /// 检查指定信号量的中断是否发生。
    ///
    /// - `HSEM_ID`：信号量编号
    ///
    /// 返回 `true` 表示发生，`false` 表示未发生。
    pub fn GetITStatus(&self, HSEM_ID: HSEM_ID) -> bool {
        (self.ISM.get() & (1 << HSEM_ID as usize)) != 0
    }

    /// 清除指定信号量的中断挂起位。
    ///
    /// - `HSEM_ID`：信号量编号
    pub fn ClearITPendingBit(&self, HSEM_ID: HSEM_ID) {
        self.ISR.set(1 << HSEM_ID as usize);
    }
}

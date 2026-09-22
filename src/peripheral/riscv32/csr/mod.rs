#![allow(unused_variables)]

pub mod marchid;
pub mod mhartid;
pub mod mimpid;
pub mod misa;
pub mod mstatus;
pub mod mvendorid;

use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use marchid::MARCHID;
use mhartid::MHARTID;
use mstatus::MSTATUS;
use mvendorid::MVENDORID;

#[derive(Debug)]
pub struct RiscvCSR {
    pub mstatus: CSR<MSTATUS, 0x300>,

    pub mvendorid: CSR<MVENDORID, 0xF11>,
    pub marchid: CSR<MARCHID, 0xF12>,
    pub mhartid: CSR<MHARTID, 0xF14>,
}

#[derive(Copy, Clone)]
pub struct CSR<T, const CSR_ADDR: usize>(PhantomData<T>);

unsafe fn transmute<Src, Dst>(src: Src) -> Dst {
    const { assert!(size_of::<Src>() == size_of::<Dst>()) };
    unsafe { core::mem::transmute_copy(&ManuallyDrop::new(src)) }
}

impl<T: Debug + Copy, const CSR_ADDR: usize> Debug for CSR<T, CSR_ADDR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.get(), f)
    }
}

impl<T: Copy, const CSR_ADDR: usize> CSR<T, CSR_ADDR> {
    pub fn get(self) -> T {
        const { assert!(size_of::<T>() == size_of::<usize>()) };
        unsafe { transmute(csrr::<CSR_ADDR>()) }
    }

    pub fn set(self, value: T) {
        const { assert!(size_of::<T>() == size_of::<usize>()) };
        unsafe { csrw::<CSR_ADDR>(transmute(value)) }
    }

    pub fn bit_or(self, value: T) {
        const { assert!(size_of::<T>() == size_of::<usize>()) };
        unsafe { csrs::<CSR_ADDR>(transmute(value)) };
    }

    pub fn bit_clear(self, value: T) {
        const { assert!(size_of::<T>() == size_of::<usize>()) };
        unsafe { csrc::<CSR_ADDR>(transmute(value)) };
    }

    pub fn swap(self, value: T) -> T {
        const { assert!(size_of::<T>() == size_of::<usize>()) };
        unsafe { transmute(csrrw::<CSR_ADDR>(transmute(value))) }
    }

    pub fn fetch_or(self, value: T) -> T {
        const { assert!(size_of::<T>() == size_of::<usize>()) };
        unsafe { transmute(csrrs::<CSR_ADDR>(transmute(value))) }
    }

    pub fn fetch_clear(self, value: T) -> T {
        const { assert!(size_of::<T>() == size_of::<usize>()) };
        unsafe { transmute(csrrc::<CSR_ADDR>(transmute(value))) }
    }
}

/// 读取 `CSR` 指向的 CSR，返回其旧值。
///
/// - `CSR`：CSR 地址，需为编译期常量。
#[inline(always)]
pub unsafe fn csrr<const CSR: usize>() -> usize {
    cfg_select! {
        any(target_arch = "riscv32", target_arch = "riscv64") => {
            let val: usize;
            unsafe { core::arch::asm!("csrr {}, {}", out(reg) val, const CSR) };
            val
        }
        _ => unimplemented!(),
    }
}

/// 将 `val` 写入 `CSR` 指向的 CSR，不返回旧值。
///
/// - `CSR`：CSR 地址，需为编译期常量。
/// - `val`：待写入的值。
#[inline(always)]
pub unsafe fn csrw<const CSR: usize>(val: usize) {
    cfg_select! {
        any(target_arch = "riscv32", target_arch = "riscv64") => {
            unsafe { core::arch::asm!("csrw {}, {}", const CSR, in(reg) val) };
        }
        _ => unimplemented!(),
    }
}

/// 将 `mask` 中置位的比特在 `CSR` 指向的 CSR 中置位，不返回旧值。
///
/// - `CSR`：CSR 地址，需为编译期常量。
/// - `mask`：置位掩码。
#[inline(always)]
pub unsafe fn csrs<const CSR: usize>(mask: usize) {
    cfg_select! {
        any(target_arch = "riscv32", target_arch = "riscv64") => {
            unsafe { core::arch::asm!("csrs {}, {}", const CSR, in(reg) mask) };
        }
        _ => unimplemented!(),
    }
}

/// 将 `mask` 中置位的比特在 `CSR` 指向的 CSR 中清零，不返回旧值。
///
/// - `CSR`：CSR 地址，需为编译期常量。
/// - `mask`：清零掩码。
#[inline(always)]
pub unsafe fn csrc<const CSR: usize>(mask: usize) {
    cfg_select! {
        any(target_arch = "riscv32", target_arch = "riscv64") => {
            unsafe { core::arch::asm!("csrc {}, {}", const CSR, in(reg) mask) };
        }
        _ => unimplemented!(),
    }
}

/// 原子地将 `val` 写入 `CSR` 指向的 CSR，返回其旧值。
///
/// - `CSR`：CSR 地址，需为编译期常量。
/// - `val`：待写入的值。
#[inline(always)]
pub unsafe fn csrrw<const CSR: usize>(val: usize) -> usize {
    cfg_select! {
        any(target_arch = "riscv32", target_arch = "riscv64") => {
            let old: usize;
            unsafe { core::arch::asm!("csrrw {}, {}, {}", out(reg) old, const CSR, in(reg) val) };
            old
        }
        _ => unimplemented!(),
    }
}

/// 原子地将 `mask` 中置位的比特在 `CSR` 指向的 CSR 中置位，返回其旧值。
///
/// - `CSR`：CSR 地址，需为编译期常量。
/// - `mask`：置位掩码。
#[inline(always)]
pub unsafe fn csrrs<const CSR: usize>(mask: usize) -> usize {
    cfg_select! {
        any(target_arch = "riscv32", target_arch = "riscv64") => {
            let old: usize;
            unsafe { core::arch::asm!("csrrs {}, {}, {}", out(reg) old, const CSR, in(reg) mask) };
            old
        }
        _ => unimplemented!(),
    }
}

/// 原子地将 `mask` 中置位的比特在 `CSR` 指向的 CSR 中清零，返回其旧值。
///
/// - `CSR`：CSR 地址，需为编译期常量。
/// - `mask`：清零掩码。
#[inline(always)]
pub unsafe fn csrrc<const CSR: usize>(mask: usize) -> usize {
    cfg_select! {
        any(target_arch = "riscv32", target_arch = "riscv64") => {
            let old: usize;
            unsafe { core::arch::asm!("csrrc {}, {}, {}", out(reg) old, const CSR, in(reg) mask) };
            old
        }
        _ => unimplemented!(),
    }
}

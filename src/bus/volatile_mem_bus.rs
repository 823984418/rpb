use core::convert::Infallible;
use core::num::Wrapping;
use core::ops::{Add, BitAnd, BitOr, BitXor, Not, Sub};

use crate::bus::Bus;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VolatileMemBus(());

impl VolatileMemBus {
    pub const unsafe fn new() -> Self {
        Self(())
    }
}

unsafe fn load<T>(addr: usize) -> T {
    let addr = addr & const { !(align_of::<T>() - 1) };
    unsafe { core::ptr::read_volatile(addr as _) }
}

unsafe fn store<T>(addr: usize, val: T) {
    let addr = addr & const { !(align_of::<T>() - 1) };
    unsafe { core::ptr::write_volatile(addr as _, val) }
}

unsafe fn fetch_and<T: BitAnd<Output = T> + Copy>(addr: usize, val: T) -> T {
    unsafe {
        let v: T = load(addr);
        store(addr, v & val);
        v
    }
}

unsafe fn fetch_or<T: BitOr<Output = T> + Copy>(addr: usize, val: T) -> T {
    unsafe {
        let v: T = load(addr);
        store(addr, v | val);
        v
    }
}

unsafe fn fetch_xor<T: BitXor<Output = T> + Copy>(addr: usize, val: T) -> T {
    unsafe {
        let v: T = load(addr);
        store(addr, v ^ val);
        v
    }
}

unsafe fn fetch_nand<T: BitAnd<Output = T> + Not<Output = T> + Copy>(addr: usize, val: T) -> T {
    unsafe {
        let v: T = load(addr);
        store(addr, !(v & val));
        v
    }
}

unsafe fn fetch_add<T: Copy>(addr: usize, val: T) -> T
where
    Wrapping<T>: Add<Output = Wrapping<T>>,
{
    unsafe {
        let v: T = load(addr);
        store(addr, (Wrapping(v) + Wrapping(val)).0);
        v
    }
}

unsafe fn fetch_sub<T: Copy>(addr: usize, val: T) -> T
where
    Wrapping<T>: Sub<Output = Wrapping<T>>,
{
    unsafe {
        let v: T = load(addr);
        store(addr, (Wrapping(v) - Wrapping(val)).0);
        v
    }
}

unsafe fn fetch_max<T: Ord + Copy>(addr: usize, val: T) -> T {
    unsafe {
        let v: T = load(addr);
        store(addr, Ord::max(v, val));
        v
    }
}

unsafe fn fetch_min<T: Ord + Copy>(addr: usize, val: T) -> T {
    unsafe {
        let v: T = load(addr);
        store(addr, Ord::min(v, val));
        v
    }
}

unsafe fn swap<T: Copy>(addr: usize, val: T) -> T {
    unsafe {
        let v: T = load(addr);
        store(addr, val);
        v
    }
}

impl Bus for VolatileMemBus {
    type Error = Infallible;

    fn load_8(self, addr: usize) -> Result<u8, Self::Error> {
        unsafe { Ok(load(addr)) }
    }

    fn load_16(self, addr: usize) -> Result<u16, Self::Error> {
        unsafe { Ok(load(addr)) }
    }

    fn load_32(self, addr: usize) -> Result<u32, Self::Error> {
        unsafe { Ok(load(addr)) }
    }

    fn load_64(self, addr: usize) -> Result<u64, Self::Error> {
        unsafe { Ok(load(addr)) }
    }

    fn load_128(self, addr: usize) -> Result<u128, Self::Error> {
        unsafe { Ok(load(addr)) }
    }

    fn store_8(self, addr: usize, val: u8) -> Result<(), Self::Error> {
        unsafe { Ok(store(addr, val)) }
    }

    fn store_16(self, addr: usize, val: u16) -> Result<(), Self::Error> {
        unsafe { Ok(store(addr, val)) }
    }

    fn store_32(self, addr: usize, val: u32) -> Result<(), Self::Error> {
        unsafe { Ok(store(addr, val)) }
    }

    fn store_64(self, addr: usize, val: u64) -> Result<(), Self::Error> {
        unsafe { Ok(store(addr, val)) }
    }

    fn store_128(self, addr: usize, val: u128) -> Result<(), Self::Error> {
        unsafe { Ok(store(addr, val)) }
    }

    fn fetch_add_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(fetch_add(addr, val)) }
    }

    fn fetch_add_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(fetch_add(addr, val)) }
    }

    fn fetch_add_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(fetch_add(addr, val)) }
    }

    fn fetch_add_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(fetch_add(addr, val)) }
    }

    fn fetch_add_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(fetch_add(addr, val)) }
    }

    fn fetch_and_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(fetch_and(addr, val)) }
    }

    fn fetch_and_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(fetch_and(addr, val)) }
    }

    fn fetch_and_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(fetch_and(addr, val)) }
    }

    fn fetch_and_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(fetch_and(addr, val)) }
    }

    fn fetch_and_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(fetch_and(addr, val)) }
    }

    fn fetch_max_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(fetch_max(addr, val)) }
    }

    fn fetch_max_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(fetch_max(addr, val)) }
    }

    fn fetch_max_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(fetch_max(addr, val)) }
    }

    fn fetch_max_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(fetch_max(addr, val)) }
    }

    fn fetch_max_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(fetch_max(addr, val)) }
    }

    fn fetch_min_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(fetch_min(addr, val)) }
    }

    fn fetch_min_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(fetch_min(addr, val)) }
    }

    fn fetch_min_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(fetch_min(addr, val)) }
    }

    fn fetch_min_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(fetch_min(addr, val)) }
    }

    fn fetch_min_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(fetch_min(addr, val)) }
    }

    fn fetch_nand_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(fetch_nand(addr, val)) }
    }

    fn fetch_nand_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(fetch_nand(addr, val)) }
    }

    fn fetch_nand_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(fetch_nand(addr, val)) }
    }

    fn fetch_nand_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(fetch_nand(addr, val)) }
    }

    fn fetch_nand_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(fetch_nand(addr, val)) }
    }

    fn fetch_or_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(fetch_or(addr, val)) }
    }

    fn fetch_or_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(fetch_or(addr, val)) }
    }

    fn fetch_or_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(fetch_or(addr, val)) }
    }

    fn fetch_or_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(fetch_or(addr, val)) }
    }

    fn fetch_or_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(fetch_or(addr, val)) }
    }

    fn fetch_sub_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(fetch_sub(addr, val)) }
    }

    fn fetch_sub_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(fetch_sub(addr, val)) }
    }

    fn fetch_sub_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(fetch_sub(addr, val)) }
    }

    fn fetch_sub_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(fetch_sub(addr, val)) }
    }

    fn fetch_sub_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(fetch_sub(addr, val)) }
    }

    fn fetch_xor_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(fetch_xor(addr, val)) }
    }

    fn fetch_xor_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(fetch_xor(addr, val)) }
    }

    fn fetch_xor_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(fetch_xor(addr, val)) }
    }

    fn fetch_xor_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(fetch_xor(addr, val)) }
    }

    fn fetch_xor_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(fetch_xor(addr, val)) }
    }

    fn swap_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        unsafe { Ok(swap(addr, val)) }
    }

    fn swap_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        unsafe { Ok(swap(addr, val)) }
    }

    fn swap_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        unsafe { Ok(swap(addr, val)) }
    }

    fn swap_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        unsafe { Ok(swap(addr, val)) }
    }

    fn swap_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        unsafe { Ok(swap(addr, val)) }
    }
}

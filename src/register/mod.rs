use core::cell::UnsafeCell;

pub struct Reg<T: ?Sized>(UnsafeCell<T>);

pub struct Reg8(Reg<u8>);

pub struct Reg16(Reg<u16>);

pub struct Reg32(Reg<u32>);

pub struct Reg64(Reg<u64>);

pub struct Reg128(Reg<u128>);

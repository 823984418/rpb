use crate::bus::Bus;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AtomicMemBus();

pub struct UnsupportedAtomicError;

impl Bus for AtomicMemBus {
    type Error = UnsupportedAtomicError;

    fn load_8(self, addr: usize) -> Result<u8, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn load_16(self, addr: usize) -> Result<u16, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn load_32(self, addr: usize) -> Result<u32, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn load_64(self, addr: usize) -> Result<u64, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn load_128(self, addr: usize) -> Result<u128, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn store_8(self, addr: usize, val: u8) -> Result<(), Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn store_16(self, addr: usize, val: u16) -> Result<(), Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn store_32(self, addr: usize, val: u32) -> Result<(), Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn store_64(self, addr: usize, val: u64) -> Result<(), Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn store_128(self, addr: usize, val: u128) -> Result<(), Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_add_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_add_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_add_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_add_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_add_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_and_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_and_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_and_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_and_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_and_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_max_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_max_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_max_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_max_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_max_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_min_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_min_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_min_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_min_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_min_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_nand_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_nand_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_nand_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_nand_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_nand_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_or_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_or_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_or_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_or_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_or_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_sub_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_sub_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_sub_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        Err(UnsupportedAtomicError)
    }

    fn fetch_sub_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        todo!()
    }

    fn fetch_sub_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        todo!()
    }

    fn fetch_xor_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        todo!()
    }

    fn fetch_xor_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        todo!()
    }

    fn fetch_xor_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        todo!()
    }

    fn fetch_xor_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        todo!()
    }

    fn fetch_xor_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        todo!()
    }

    fn swap_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        todo!()
    }

    fn swap_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        todo!()
    }

    fn swap_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        todo!()
    }

    fn swap_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        todo!()
    }

    fn swap_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        todo!()
    }
}

use crate::bus::Bus;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OffsetBus<B: Bus> {
    offset: isize,
    bus: B,
}

fn bus_offset(offset: isize, addr: usize) -> usize {
    addr.wrapping_add_signed(offset)
}

impl<B: Bus> Bus for OffsetBus<B> {
    type Error = B::Error;

    fn load_8(self, addr: usize) -> Result<u8, Self::Error> {
        self.bus.load_8(bus_offset(self.offset, addr))
    }

    fn load_16(self, addr: usize) -> Result<u16, Self::Error> {
        self.bus.load_16(bus_offset(self.offset, addr))
    }

    fn load_32(self, addr: usize) -> Result<u32, Self::Error> {
        self.bus.load_32(bus_offset(self.offset, addr))
    }

    fn load_64(self, addr: usize) -> Result<u64, Self::Error> {
        self.bus.load_64(bus_offset(self.offset, addr))
    }

    fn load_128(self, addr: usize) -> Result<u128, Self::Error> {
        self.bus.load_128(bus_offset(self.offset, addr))
    }

    fn store_8(self, addr: usize, val: u8) -> Result<(), Self::Error> {
        self.bus.store_8(bus_offset(self.offset, addr), val)
    }

    fn store_16(self, addr: usize, val: u16) -> Result<(), Self::Error> {
        self.bus.store_16(bus_offset(self.offset, addr), val)
    }

    fn store_32(self, addr: usize, val: u32) -> Result<(), Self::Error> {
        self.bus.store_32(bus_offset(self.offset, addr), val)
    }

    fn store_64(self, addr: usize, val: u64) -> Result<(), Self::Error> {
        self.bus.store_64(bus_offset(self.offset, addr), val)
    }

    fn store_128(self, addr: usize, val: u128) -> Result<(), Self::Error> {
        self.bus.store_128(bus_offset(self.offset, addr), val)
    }

    fn fetch_add_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.fetch_add_8(bus_offset(self.offset, addr), val)
    }

    fn fetch_add_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.fetch_add_16(bus_offset(self.offset, addr), val)
    }

    fn fetch_add_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.fetch_add_32(bus_offset(self.offset, addr), val)
    }

    fn fetch_add_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.fetch_add_64(bus_offset(self.offset, addr), val)
    }

    fn fetch_add_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.fetch_add_128(bus_offset(self.offset, addr), val)
    }

    fn fetch_and_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.fetch_and_8(bus_offset(self.offset, addr), val)
    }

    fn fetch_and_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.fetch_and_16(bus_offset(self.offset, addr), val)
    }

    fn fetch_and_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.fetch_and_32(bus_offset(self.offset, addr), val)
    }

    fn fetch_and_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.fetch_and_64(bus_offset(self.offset, addr), val)
    }

    fn fetch_and_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.fetch_and_128(bus_offset(self.offset, addr), val)
    }

    fn fetch_max_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.fetch_max_8(bus_offset(self.offset, addr), val)
    }

    fn fetch_max_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.fetch_max_16(bus_offset(self.offset, addr), val)
    }

    fn fetch_max_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.fetch_max_32(bus_offset(self.offset, addr), val)
    }

    fn fetch_max_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.fetch_max_64(bus_offset(self.offset, addr), val)
    }

    fn fetch_max_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.fetch_max_128(bus_offset(self.offset, addr), val)
    }

    fn fetch_min_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.fetch_min_8(bus_offset(self.offset, addr), val)
    }

    fn fetch_min_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.fetch_min_16(bus_offset(self.offset, addr), val)
    }

    fn fetch_min_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.fetch_min_32(bus_offset(self.offset, addr), val)
    }

    fn fetch_min_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.fetch_min_64(bus_offset(self.offset, addr), val)
    }

    fn fetch_min_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.fetch_min_128(bus_offset(self.offset, addr), val)
    }

    fn fetch_nand_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.fetch_nand_8(bus_offset(self.offset, addr), val)
    }

    fn fetch_nand_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.fetch_nand_16(bus_offset(self.offset, addr), val)
    }

    fn fetch_nand_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.fetch_nand_32(bus_offset(self.offset, addr), val)
    }

    fn fetch_nand_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.fetch_nand_64(bus_offset(self.offset, addr), val)
    }

    fn fetch_nand_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.fetch_nand_128(bus_offset(self.offset, addr), val)
    }

    fn fetch_or_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.fetch_or_8(bus_offset(self.offset, addr), val)
    }

    fn fetch_or_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.fetch_or_16(bus_offset(self.offset, addr), val)
    }

    fn fetch_or_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.fetch_or_32(bus_offset(self.offset, addr), val)
    }

    fn fetch_or_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.fetch_or_64(bus_offset(self.offset, addr), val)
    }

    fn fetch_or_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.fetch_or_128(bus_offset(self.offset, addr), val)
    }

    fn fetch_sub_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.fetch_sub_8(bus_offset(self.offset, addr), val)
    }

    fn fetch_sub_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.fetch_sub_16(bus_offset(self.offset, addr), val)
    }

    fn fetch_sub_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.fetch_sub_32(bus_offset(self.offset, addr), val)
    }

    fn fetch_sub_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.fetch_sub_64(bus_offset(self.offset, addr), val)
    }

    fn fetch_sub_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.fetch_sub_128(bus_offset(self.offset, addr), val)
    }

    fn fetch_xor_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.fetch_xor_8(bus_offset(self.offset, addr), val)
    }

    fn fetch_xor_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.fetch_xor_16(bus_offset(self.offset, addr), val)
    }

    fn fetch_xor_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.fetch_xor_32(bus_offset(self.offset, addr), val)
    }

    fn fetch_xor_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.fetch_xor_64(bus_offset(self.offset, addr), val)
    }

    fn fetch_xor_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.fetch_xor_128(bus_offset(self.offset, addr), val)
    }

    fn swap_8(self, addr: usize, val: u8) -> Result<u8, Self::Error> {
        self.bus.swap_8(bus_offset(self.offset, addr), val)
    }

    fn swap_16(self, addr: usize, val: u16) -> Result<u16, Self::Error> {
        self.bus.swap_16(bus_offset(self.offset, addr), val)
    }

    fn swap_32(self, addr: usize, val: u32) -> Result<u32, Self::Error> {
        self.bus.swap_32(bus_offset(self.offset, addr), val)
    }

    fn swap_64(self, addr: usize, val: u64) -> Result<u64, Self::Error> {
        self.bus.swap_64(bus_offset(self.offset, addr), val)
    }

    fn swap_128(self, addr: usize, val: u128) -> Result<u128, Self::Error> {
        self.bus.swap_128(bus_offset(self.offset, addr), val)
    }
}

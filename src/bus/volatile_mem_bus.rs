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

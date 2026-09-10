pub mod atomic_mem_bus;
pub mod offset_bus;
pub mod volatile_mem_bus;

pub trait BusBase {
    type Error;
}

pub trait BusLoad<T>: BusBase {
    fn load(self, addr: usize) -> Result<T, Self::Error>;
}

pub trait BusAsyncLoad<T>: BusBase {
    fn async_load(self, addr: usize) -> impl Future<Output = Result<T, Self::Error>>;
}

pub trait BusWrite<T>: BusBase {
    fn write(self, addr: usize, val: T) -> Result<(), Self::Error>;
}

pub trait BusAsyncWrite<T>: BusBase {
    fn write(self, addr: usize, val: T) -> impl Future<Output = Result<(), Self::Error>>;
}

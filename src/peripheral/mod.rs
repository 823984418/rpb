use core::ops::{Deref, DerefMut};

pub mod cortex_m;
pub mod riscv32;

pub struct Block<T: ?Sized>(*mut T);

impl<T> Block<T> {
    pub const unsafe fn from_addr(addr: usize) -> Self {
        Self(addr as *mut T)
    }
}

impl<T: ?Sized> Deref for Block<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T: ?Sized> DerefMut for Block<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

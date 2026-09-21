use core::cell::UnsafeCell;
use core::fmt::{Debug, Formatter};
use core::ops::{BitAnd, BitOr, Deref, DerefMut, Not};

#[repr(transparent)]
pub struct Reg<T>(UnsafeCell<T>);

impl<T> Reg<T> {
    pub const unsafe fn from_ptr<'s>(addr: *mut T) -> &'s Self {
        unsafe { &*(addr as *mut Self) }
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.0.get()
    }

    pub fn set(&self, value: T) {
        unsafe { core::ptr::write_volatile(self.0.get(), value) }
    }

    pub fn get(&self) -> T {
        unsafe { core::ptr::read_volatile(self.0.get()) }
    }

    pub fn bit_or(&self, rhs: T)
    where
        T: BitOr<T, Output = T>,
    {
        self.set(self.get() | rhs);
    }

    pub fn bit_and(&self, rhs: T)
    where
        T: BitAnd<T, Output = T>,
    {
        self.set(self.get() & rhs);
    }

    pub fn bit_clear(&self, rhs: T)
    where
        T: Not<Output = T> + BitAnd<T, Output = T>,
    {
        self.set(self.get() & !rhs);
    }

    pub fn bit_clear_or(&self, mask: T, value: T)
    where
        T: Not<Output = T> + BitAnd<T, Output = T> + BitOr<T, Output = T>,
    {
        self.set((self.get() & !mask) | value);
    }

    pub fn bit_set(&self, mask: T, value: bool)
    where
        T: Not<Output = T> + BitAnd<T, Output = T> + BitOr<T, Output = T>,
    {
        if value {
            self.bit_or(mask);
        } else {
            self.bit_clear(mask);
        }
    }
}

impl<T: Debug> Debug for Reg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let value = self.get();
        Debug::fmt(&value, f)
    }
}

impl<T: bitflags::Flags> Deref for Reg<T> {
    type Target = Reg<T::Bits>;

    fn deref(&self) -> &Self::Target {
        const {
            assert!(size_of::<Self>() == size_of::<Self::Target>());
            assert!(align_of::<Self>() == align_of::<Self::Target>());
        }
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl<T: bitflags::Flags> DerefMut for Reg<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        const {
            assert!(size_of::<Self>() == size_of::<Self::Target>());
            assert!(align_of::<Self>() == align_of::<Self::Target>());
        }
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

pub type Reg8 = Reg<u8>;
pub type Reg16 = Reg<u16>;
pub type Reg32 = Reg<u32>;
pub type Reg64 = Reg<u64>;
pub type Reg128 = Reg<u128>;

mod regs;

pub use regs::*;

use crate::register::{Reg, Reg8};

impl PFIC {
    pub const BASE: usize = 0xE000E000;

    pub fn SystemReset(&self) -> ! {
        self.CFGR.set(CFGR::KEYCODE | CFGR::SYSRST);
        loop {}
    }

    pub fn HART_ID(&self) -> &Reg8 {
        unsafe { Reg::from_ptr((self.SCTLR.as_ptr() as *mut u8).add(2)) }
    }

    pub fn GetCurrentCoreID(&self) -> u8 {
        self.HART_ID().get()
    }

    pub fn EnableIRQ(&self, IRQn: usize) {
        self.IENR[IRQn >> 5].set(1 << (IRQn & 0x1F));
    }

    pub fn DisableIRQ(&self, IRQn: usize) {
        self.IRER[IRQn >> 5].set(1 << (IRQn & 0x1F));
        crate::peripheral::riscv32::asm::fence_i();
    }
}

mod regs;

pub use regs::*;

impl PFIC {
    pub const BASE: usize = 0xE000E000;

    pub fn SystemReset(&self) {
        self.CFGR.set(CFGR::KEYCODE | CFGR::SYSRST);
    }

    pub fn GetCurrentCoreID(&self) -> u8 {
        self.HART_ID().get()
    }
}

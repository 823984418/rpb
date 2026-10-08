mod regs;

pub use regs::*;

impl IWDT {
    pub fn clear(&self) {
        self.CLR.set(CLR::IWDTCLR_VALUE);
    }
}

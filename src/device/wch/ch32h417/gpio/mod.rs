mod regs;

pub use regs::*;

impl GPIO {}

impl AFIO {
    pub fn PinAFConfig(&self, port: u8, pin: u8, af: u8) {
        let index = port * 16 + pin;
        let base = self.GPIOA_AFLR.as_ptr() as *mut u8;
        unsafe {
            let reg = base.add(index as usize >> 1);
            if index & 1 == 0 {
                let val = core::ptr::read_volatile(reg) & 0xF0;
                core::ptr::write_volatile(reg, val | af);
            } else {
                let val = core::ptr::read_volatile(reg) & 0x0F;
                core::ptr::write_volatile(reg, val | (af << 4));
            }
        }
    }

    pub fn EXTILineConfig(&self, port: u8, pin: u8) {
        let base = self.EXTICR1.as_ptr() as *mut u8;
        unsafe {
            let reg = base.add(pin as usize >> 1);
            if pin & 1 == 0 {
                let val = core::ptr::read_volatile(reg) & 0xF0;
                core::ptr::write_volatile(reg, val | port);
            } else {
                let val = core::ptr::read_volatile(reg) & 0x0F;
                core::ptr::write_volatile(reg, val | (port << 4));
            }
        }
    }
}

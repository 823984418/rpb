mod regs;

pub use regs::*;

pub struct GPIO_Init {
    pub Pin: u16,
    pub Speed: SPEED_PIN,
    pub Mode: CFGR_PIN,
}

impl GPIO {
    pub fn Init(&self, InitStruct: &GPIO_Init) {
        let mut speed_clear = SPEED::empty();
        let mut speed_or = SPEED::empty();
        let mut cfglr_clear = CFGR::empty();
        let mut cfglr_or = CFGR::empty();
        let mut cfghr_clear = CFGR::empty();
        let mut cfghr_or = CFGR::empty();
        for i in 0..8 {
            if InitStruct.Pin & (1 << i) != 0 {
                speed_clear |= SPEED_PIN::MASK.pin(i);
                speed_or |= InitStruct.Speed.pin(i);
                cfglr_clear |= CFGR_PIN::MASK.pin(i);
                cfglr_or |= InitStruct.Mode.pin(i);
            }
        }
        for i in 8..16 {
            if InitStruct.Pin & (1 << i) != 0 {
                speed_clear |= SPEED_PIN::MASK.pin(i);
                speed_or |= InitStruct.Speed.pin(i);
                cfghr_clear |= CFGR_PIN::MASK.pin(i - 8);
                cfghr_or |= InitStruct.Mode.pin(i - 8);
            }
        }

        self.SPEED.bit_clear_or(speed_clear, speed_or);
        self.CFGLR.bit_clear_or(cfglr_clear, cfglr_or);
        self.CFGHR.bit_clear_or(cfghr_clear, cfghr_or);
    }
}

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

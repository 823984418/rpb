mod regs;

use bitflags::bitflags;
pub use regs::*;

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GPIO_Ports: u16 {
        const PF = 1 << 5;
        const PE = 1 << 4;
        const PD = 1 << 3;
        const PC = 1 << 2;
        const PB = 1 << 1;
        const PA = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GPIO_Pins: u16 {
        const PIN15 = 1 << 15;
        const PIN14 = 1 << 14;
        const PIN13 = 1 << 13;
        const PIN12 = 1 << 12;
        const PIN11 = 1 << 11;
        const PIN10 = 1 << 10;
        const PIN9 = 1 << 9;
        const PIN8 = 1 << 8;
        const PIN7 = 1 << 7;
        const PIN6 = 1 << 6;
        const PIN5 = 1 << 5;
        const PIN4 = 1 << 4;
        const PIN3 = 1 << 3;
        const PIN2 = 1 << 2;
        const PIN1 = 1 << 1;
        const PIN0 = 1 << 0;
    }
}

pub struct GPIO_Init {
    pub Pin: GPIO_Pins,
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
            if InitStruct.Pin.bits() & (1 << i) != 0 {
                speed_clear |= SPEED_PIN::MASK.pin(i);
                speed_or |= InitStruct.Speed.pin(i);
                cfglr_clear |= CFGR_PIN::MASK.pin(i);
                cfglr_or |= InitStruct.Mode.pin(i);
            }
        }
        for i in 8..16 {
            if InitStruct.Pin.bits() & (1 << i) != 0 {
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

    pub fn ReadInputDataBit(&self, Pin: GPIO_Pins) -> bool {
        GPIO_Pins::from_bits_retain(self.INDR.get() as u16).intersects(Pin)
    }

    pub fn ReadInputData(&self) -> GPIO_Pins {
        GPIO_Pins::from_bits_retain(self.INDR.get() as u16)
    }

    pub fn ReadOutputDataBit(&self, Pin: GPIO_Pins) -> bool {
        GPIO_Pins::from_bits_retain(self.OUTDR.get() as u16).intersects(Pin)
    }

    pub fn ReadOutputData(&self) -> GPIO_Pins {
        GPIO_Pins::from_bits_retain(self.OUTDR.get() as u16)
    }

    pub fn SetBits(&self, Pin: GPIO_Pins) {
        self.BSHR.set(Pin.bits() as u32);
    }

    pub fn ResetBits(&self, Pin: GPIO_Pins) {
        self.BCR.set(Pin.bits() as u32);
    }

    pub fn WriteBit(&self, Pin: GPIO_Pins, BitVal: bool) {
        if BitVal {
            self.BSHR.set(Pin.bits() as u32);
        } else {
            self.BCR.set(Pin.bits() as u32);
        }
    }

    pub fn Write(&self, PortVal: GPIO_Pins) {
        self.OUTDR.set(PortVal.bits() as u32);
    }

    pub fn PinLockConfig(&self, Pin: GPIO_Pins) {
        let lock1 = 0x00010000 | Pin.bits() as u32;
        let lock0 = Pin.bits() as u32;
        self.LCKR.set(lock1);
        self.LCKR.set(lock0);
        self.LCKR.set(lock1);
        let _ = self.LCKR.get();
        let _ = self.LCKR.get();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GPIO_Port {
    PA = 0,
    PB = 1,
    PC = 2,
    PD = 3,
    PE = 4,
    PF = 5,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GPIO_Pin {
    PIN0,
    PIN1,
    PIN2,
    PIN3,
    PIN4,
    PIN5,
    PIN6,
    PIN7,
    PIN8,
    PIN9,
    PIN10,
    PIN11,
    PIN12,
    PIN13,
    PIN14,
    PIN15,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GPIO_Af {
    AF0,
    AF1,
    AF2,
    AF3,
    AF4,
    AF5,
    AF6,
    AF7,
}

impl AFIO {
    pub fn PinAFConfig(&self, port: GPIO_Port, pin: GPIO_Pin, af: GPIO_Af) {
        let index = port as usize * 16 + pin as usize;
        let base = self.GPIOA_AFLR.as_ptr() as *mut u8;
        unsafe {
            let reg = base.add(index >> 1);
            if index & 1 == 0 {
                let val = core::ptr::read_volatile(reg) & 0xF0;
                core::ptr::write_volatile(reg, val | af as u8);
            } else {
                let val = core::ptr::read_volatile(reg) & 0x0F;
                core::ptr::write_volatile(reg, val | ((af as u8) << 4));
            }
        }
    }

    pub fn EXTILineConfig(&self, port: GPIO_Port, pin: GPIO_Pin) {
        let base = self.EXTICR1.as_ptr() as *mut u8;
        unsafe {
            let reg = base.add(pin as usize >> 1);
            if pin as usize & 1 == 0 {
                let val = core::ptr::read_volatile(reg) & 0xF0;
                core::ptr::write_volatile(reg, val | port as u8);
            } else {
                let val = core::ptr::read_volatile(reg) & 0x0F;
                core::ptr::write_volatile(reg, val | ((port as u8) << 4));
            }
        }
    }
}

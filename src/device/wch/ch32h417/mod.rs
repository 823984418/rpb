pub mod eth;
pub mod flash;
pub mod gpio;
pub mod pwr;
pub mod rcc;
pub mod usart;

use flash::FLASH;
use gpio::{AFIO, GPIO};
use pwr::PWR;
use rcc::RCC;
use usart::USART;

use crate::peripheral::Block;
use crate::peripheral::riscv32::qingke::csr::QingKeCSR;
use crate::peripheral::riscv32::qingke::hsem::HSEM;
use crate::peripheral::riscv32::qingke::ipc::IPC;
use crate::peripheral::riscv32::qingke::pfic::PFIC;
use crate::peripheral::riscv32::qingke::systick::SysTick;
use crate::register::Reg32;

#[derive(Debug)]
pub struct Device {
    pub CSR: QingKeCSR,

    pub CHIP_INFO: Block<Reg32, 0x1FFFF704>,

    pub PWR: Block<PWR, 0x40007000>,

    pub USART2: Block<USART, 0x40004400>,
    pub USART3: Block<USART, 0x40004800>,
    pub USART4: Block<USART, 0x40004C00>,
    pub USART5: Block<USART, 0x40005000>,
    pub USART6: Block<USART, 0x40001800>,
    pub USART7: Block<USART, 0x40001C00>,
    pub USART8: Block<USART, 0x40002000>,

    pub AFIO: Block<AFIO, 0x40010000>,
    pub GPIOA: Block<GPIO, 0x40010800>,
    pub GPIOB: Block<GPIO, 0x40010C00>,
    pub GPIOC: Block<GPIO, 0x40011000>,
    pub GPIOD: Block<GPIO, 0x40011400>,
    pub GPIOE: Block<GPIO, 0x40011800>,
    pub GPIOF: Block<GPIO, 0x40011C00>,

    pub USART1: Block<USART, 0x40013800>,

    pub FLASH: Block<FLASH, 0x40022000>,

    pub RCC: Block<RCC, 0x40021000>,

    pub HSEM: Block<HSEM, 0xE000C000>,
    pub PFIC: Block<PFIC, 0xE000E000>,
    pub IPC: Block<IPC, 0xE000D000>,
    pub SysTick0: Block<SysTick, 0xE000F000>,
    pub SysTick1: Block<SysTick, 0xE000F080>,

    pub SYS_CFGR0: Block<Reg32, 0x5003C000>,
}

impl Device {
    pub fn SetSYSCLK_400M_CoreCLK_V5F_400M_V3F_100M_HSE(&self) {
        self.RCC.CTLR.bit_or(rcc::CTLR::HSEON);
        for _ in 0..0x1000000 {
            if self.RCC.CTLR.get().intersects(rcc::CTLR::HSERDY) {
                break;
            }
        }
        let HSEStatus = self.RCC.CTLR.get() & rcc::CTLR::HSERDY;
        if !HSEStatus.is_empty() {
            self.RCC.PLLCFGR.bit_clear_or(
                rcc::PLLCFGR::PLLMUL_MASK
                    | rcc::PLLCFGR::PLL_SRC_DIV_MASK
                    | rcc::PLLCFGR::PLLSRC_MASK,
                rcc::PLLCFGR::PLLMUL_16 | rcc::PLLCFGR::PLL_SRC_DIV(0) | rcc::PLLCFGR::PLLSRC_HSE,
            );

            while self.RCC.PLLCFGR.get() & rcc::PLLCFGR::PLLSRC_MASK != rcc::PLLCFGR::PLLSRC_HSE {}

            self.RCC.CTLR.bit_or(rcc::CTLR::PLLON);

            while self.RCC.CTLR.get().intersects(rcc::CTLR::PLLRDY) {}

            self.RCC
                .PLLCFGR
                .bit_clear(rcc::PLLCFGR::SYSPLL_GATE | rcc::PLLCFGR::SYSPLL_SEL_MASK);

            while self.RCC.PLLCFGR.get() & rcc::PLLCFGR::SYSPLL_SEL_MASK
                != rcc::PLLCFGR::SYSPLL_SEL_PLL_CLK
            {}

            self.RCC.CFGR0.bit_clear_or(
                rcc::CFGR0::FPRE_MASK | rcc::CFGR0::HPRE_MASK,
                rcc::CFGR0::FPRE_4 | rcc::CFGR0::HPRE_1,
            );

            self.FLASH
                .ACTLR
                .bit_clear_or(flash::ACTLR::SCK_CFG, flash::ACTLR::SCK_CFG_2);

            self.RCC.PLLCFGR.bit_or(rcc::PLLCFGR::SYSPLL_GATE);

            self.RCC
                .CFGR0
                .bit_clear_or(rcc::CFGR0::SW_MASK, rcc::CFGR0::SW_PLL);

            while self.RCC.CFGR0.get() & rcc::CFGR0::SWS_MASK != rcc::CFGR0::SWS_PLL {}
        }
    }
}

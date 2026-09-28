pub mod adc;
pub mod can;
pub mod crc;
pub mod dac;
pub mod dfsdm;
pub mod dma;
pub mod dvp;
pub mod ecdc;
pub mod eth;
pub mod exti;
pub mod flash;
pub mod fmc;
pub mod gpha;
pub mod gpio;
pub mod hsadc;
pub mod i2c;
pub mod i3c;
pub mod iwdg;
pub mod lptim;
pub mod ltdc;
pub mod opa;
pub mod pwr;
pub mod qspi;
pub mod rcc;
pub mod rng;
pub mod rtc;
pub mod sai;
pub mod sdio;
pub mod sdmmc;
pub mod serdes;
pub mod spi;
pub mod swpmi;
pub mod tim;
pub mod usart;
pub mod usbfs;
pub mod usbhs;
pub mod usbpd;
pub mod usbss;
pub mod wwdg;

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
    pub OB: Block<flash::OB, 0x1FFFF800>,

    pub TIM2: Block<tim::TIM, 0x40000000>,
    pub TIM3: Block<tim::TIM, 0x40000400>,
    pub TIM4: Block<tim::TIM, 0x40000800>,
    pub TIM5: Block<tim::TIM, 0x40000C00>,
    pub TIM6: Block<tim::TIM, 0x40001000>,
    pub TIM7: Block<tim::TIM, 0x40001400>,
    pub USART6: Block<usart::USART, 0x40001800>,
    pub USART7: Block<usart::USART, 0x40001C00>,
    pub USART8: Block<usart::USART, 0x40002000>,
    pub LPTIM1: Block<lptim::LPTIM, 0x40002400>,
    pub RTC: Block<rtc::RTC, 0x40002800>,
    pub WWDG: Block<wwdg::WWDG, 0x40002C00>,
    pub IWDG: Block<iwdg::IWDG, 0x40003000>,
    pub LPTIM2: Block<lptim::LPTIM, 0x40003400>,
    pub SPI2: Block<spi::SPI, 0x40003800>,
    pub SPI3: Block<spi::SPI, 0x40003C00>,
    pub SPI4: Block<spi::SPI, 0x40004000>,
    pub USART2: Block<usart::USART, 0x40004400>,
    pub USART3: Block<usart::USART, 0x40004800>,
    pub USART4: Block<usart::USART, 0x40004C00>,
    pub USART5: Block<usart::USART, 0x40005000>,
    pub I2C1: Block<i2c::I2C, 0x40005400>,
    pub I2C2: Block<i2c::I2C, 0x40005800>,
    pub I2C3: Block<i2c::I2C, 0x40005C00>,
    pub CAN1: Block<can::CAN, 0x40006400>,
    pub CAN2: Block<can::CAN, 0x40006800>,
    pub PWR: Block<pwr::PWR, 0x40007000>,
    pub DAC: Block<dac::DAC, 0x40007400>,
    pub CAN3: Block<can::CAN, 0x40007800>,
    pub SWPMI: Block<swpmi::SWPMI, 0x40008400>,
    pub AFIO: Block<gpio::AFIO, 0x40010000>,
    pub EXTI: Block<exti::EXTI, 0x40010400>,
    pub GPIOA: Block<gpio::GPIO, 0x40010800>,
    pub GPIOB: Block<gpio::GPIO, 0x40010C00>,
    pub GPIOC: Block<gpio::GPIO, 0x40011000>,
    pub GPIOD: Block<gpio::GPIO, 0x40011400>,
    pub GPIOE: Block<gpio::GPIO, 0x40011800>,
    pub GPIOF: Block<gpio::GPIO, 0x40011C00>,
    pub ADC1: Block<adc::ADC, 0x40012400>,
    pub ADC2: Block<adc::ADC, 0x40012800>,
    pub TIM1: Block<tim::TIM, 0x40012C00>,
    pub SPI1: Block<spi::SPI, 0x40013000>,
    pub TIM8: Block<tim::TIM, 0x40013400>,
    pub USART1: Block<usart::USART, 0x40013800>,
    pub TIM12: Block<tim::TIM, 0x40013C00>,
    pub I2C4: Block<i2c::I2C, 0x40014000>,
    pub I3C: Block<i3c::I3C, 0x40014400>,
    pub LTDC: Block<ltdc::LTDC, 0x40014800>,
    pub LTDC_Layer1: Block<ltdc::LTDC_Layer, 0x40014834>,
    pub LTDC_Layer2: Block<ltdc::LTDC_Layer, 0x40014864>,
    pub TIM9: Block<tim::TIM, 0x40014C00>,
    pub TIM10: Block<tim::TIM, 0x40015000>,
    pub TIM11: Block<tim::TIM, 0x40015400>,
    pub SAI_Block_A: Block<sai::SAI_BLOCK, 0x40015804>,
    pub SAI_Block_B: Block<sai::SAI_BLOCK, 0x40015824>,
    pub GPHA: Block<gpha::GPHA, 0x40016800>,
    pub ECDC: Block<ecdc::ECDC, 0x40016C00>,
    pub DFSDM_Channel0: Block<dfsdm::DFSDM_Channel, 0x40017000>,
    pub DFSDM_Channel1: Block<dfsdm::DFSDM_Channel, 0x40017004>,
    pub DFSDM_FLT0: Block<dfsdm::DFSDM_FLT, 0x40017028>,
    pub DFSDM_FLT1: Block<dfsdm::DFSDM_FLT, 0x4001702C>,
    pub HSADC: Block<hsadc::HSADC, 0x40017400>,
    pub OPA: Block<opa::OPA, 0x40017800>,
    pub SDIO: Block<sdio::SDIO, 0x40018000>,
    pub DMA1: Block<dma::DMA, 0x40020000>,
    pub DMA1_Channel1: Block<dma::DMA_Channel, 0x40020008>,
    pub DMA1_Channel2: Block<dma::DMA_Channel, 0x4002001C>,
    pub DMA1_Channel3: Block<dma::DMA_Channel, 0x40020030>,
    pub DMA1_Channel4: Block<dma::DMA_Channel, 0x40020044>,
    pub DMA1_Channel5: Block<dma::DMA_Channel, 0x40020058>,
    pub DMA1_Channel6: Block<dma::DMA_Channel, 0x4002006C>,
    pub DMA1_Channel7: Block<dma::DMA_Channel, 0x40020080>,
    pub DMA1_Channel8: Block<dma::DMA_Channel, 0x40020094>,
    pub DMA2: Block<dma::DMA, 0x40020400>,
    pub DMA2_Channel1: Block<dma::DMA_Channel, 0x40020408>,
    pub DMA2_Channel2: Block<dma::DMA_Channel, 0x4002041C>,
    pub DMA2_Channel3: Block<dma::DMA_Channel, 0x40020430>,
    pub DMA2_Channel4: Block<dma::DMA_Channel, 0x40020444>,
    pub DMA2_Channel5: Block<dma::DMA_Channel, 0x40020458>,
    pub DMA2_Channel6: Block<dma::DMA_Channel, 0x4002046C>,
    pub DMA2_Channel7: Block<dma::DMA_Channel, 0x40020480>,
    pub DMA2_Channel8: Block<dma::DMA_Channel, 0x40020494>,
    pub DMAMUX: Block<dma::DMAMUX, 0x40020800>,
    pub RCC: Block<rcc::RCC, 0x40021000>,
    pub FLASH: Block<flash::FLASH, 0x40022000>,
    pub CRC: Block<crc::CRC, 0x40023000>,
    pub USBFSD: Block<usbfs::USBFSD, 0x40023400>,
    pub USBFSH: Block<usbfs::USBFSH, 0x40023400>,
    pub RNG: Block<rng::RNG, 0x40023C00>,
    pub SDMMC: Block<sdmmc::SDMMC, 0x40024000>,
    pub USBPD: Block<usbpd::USBPD, 0x40024400>,
    pub QSPI1: Block<qspi::QSPI, 0x40024C00>,
    pub QSPI2: Block<qspi::QSPI, 0x40025000>,
    pub FMC_Bank1: Block<fmc::FMC_BANK1, 0x40025400>,
    pub FMC_Bank3: Block<fmc::FMC_BANK3, 0x40025480>,
    pub FMC_Bank1E: Block<fmc::FMC_BANK1E, 0x40025504>,
    pub FMC_Bank5_6: Block<fmc::FMC_BANK5_6, 0x40025540>,
    pub DVP: Block<dvp::DVP, 0x40025800>,
    pub SDS1: Block<serdes::SDS, 0x40027C00>,
    pub SDS2: Block<serdes::SDS, 0x40027C40>,
    pub ETH: Block<eth::ETH, 0x40028000>,
    pub USBHSD: Block<usbhs::USBHSD, 0x40030000>,
    pub USBHSH: Block<usbhs::USBHSH, 0x40030100>,
    pub USBSSD: Block<usbss::USBSSD, 0x40034000>,
    pub USBSSH: Block<usbss::USBSSH, 0x40034000>,

    pub SYS_CFGR0: Block<Reg32, 0x5003C000>,
    pub SYS_CFGR4: Block<Reg32, 0x5003C010>,

    pub HSEM: Block<HSEM, 0xE000C000>,
    pub IPC: Block<IPC, 0xE000D000>,
    pub PFIC: Block<PFIC, 0xE000E000>,
    pub SysTick0: Block<SysTick, 0xE000F000>,
    pub SysTick1: Block<SysTick, 0xE000F080>,
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
                rcc::PLLCFGR::PLLMUL16 | rcc::PLLCFGR::PLL_SRC_DIV(0) | rcc::PLLCFGR::PLLSRC_HSE,
            );

            while self.RCC.PLLCFGR.get() & rcc::PLLCFGR::PLLSRC_MASK != rcc::PLLCFGR::PLLSRC_HSE {}

            self.RCC.CTLR.bit_or(rcc::CTLR::PLLON);

            while !self.RCC.CTLR.get().intersects(rcc::CTLR::PLLRDY) {}

            self.RCC
                .PLLCFGR
                .bit_clear(rcc::PLLCFGR::SYSPLL_GATE | rcc::PLLCFGR::SYSPLL_SEL_MASK);

            while self.RCC.PLLCFGR.get() & rcc::PLLCFGR::SYSPLL_SEL_MASK != rcc::PLLCFGR::SYSPLL_PLL
            {
            }

            self.RCC.CFGR0.bit_clear_or(
                rcc::CFGR0::FPRE_MASK | rcc::CFGR0::HPRE_MASK,
                rcc::CFGR0::FPRE_DIV1 | rcc::CFGR0::HPRE_DIV1,
            );

            self.FLASH
                .ACTLR
                .bit_clear_or(flash::ACTLR::SCK_CFG_MASK, flash::ACTLR::LATENCY_HCLK_DIV2);

            self.RCC.PLLCFGR.bit_or(rcc::PLLCFGR::SYSPLL_GATE);

            self.RCC
                .CFGR0
                .bit_clear_or(rcc::CFGR0::SW_MASK, rcc::CFGR0::SW_PLL);

            while self.RCC.CFGR0.get() & rcc::CFGR0::SWS_MASK != rcc::CFGR0::SWS_PLL {}
        }
    }
}

pub mod adc;
pub mod amoc;
pub mod exti;
pub mod flash;
pub mod gpio;
pub mod macp;
pub mod mcm;
pub mod pca;
pub mod pwm;
pub mod rcc;
pub mod spi;
pub mod syscfg;
pub mod tim;
pub mod uart;
pub mod wdt;

use crate::peripheral::Block;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Device {
    pub PCA0: Block<pca::PCA0, 0x40000400>,

    pub TIM7: Block<tim::tim7::TIM7, 0x40000C00>,
    pub TIM8: Block<tim::tim8::TIM8, 0x40001000>,

    pub UART0: Block<uart::uart0::UART0, 0x40001800>,
    pub UART1: Block<uart::uart1::UART1, 0x40001C00>,

    pub SPI0: Block<spi::SPI0, 0x40002400>,

    pub IWDT: Block<wdt::iwdt::IWDT, 0x40003000>,
    pub WWDT: Block<wdt::wwdt::WWDT, 0x40003400>,

    pub AMOC: Block<amoc::AMOC, 0x40003800>,

    pub EXTI: Block<exti::EXTI, 0x40004000>,

    pub PWM3: Block<pwm::pwm3::PWM3, 0x40005000>,
    pub PWM4: Block<pwm::pwm4::PWM4, 0x40005400>,

    pub GPIOA: Block<gpio::GPIO, 0x40040000>,
    pub GPIOB: Block<gpio::GPIO, 0x40040080>,
    pub GPIOC: Block<gpio::GPIO, 0x40040100>,

    pub MCM: Block<mcm::MCM, 0x40040400>,
    pub SYSCFG: Block<syscfg::SYSCFG, 0x40040800>,
    pub RCC: Block<rcc::RCC, 0x40040C00>,
    pub FLASH: Block<flash::FLASH, 0x40041000>,
    pub MACP: Block<macp::MACP, 0x40041400>,
    pub ADC: Block<adc::ADC, 0x40041800>,
}

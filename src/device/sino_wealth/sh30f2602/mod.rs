pub mod adc;
pub mod amoc;
pub mod exti;
pub mod flash;
pub mod gpio;
pub mod iwdt;
pub mod macp;
pub mod mcm;
pub mod n_test;
pub mod pca;
pub mod pwm;
pub mod rcc;
pub mod spi;
pub mod syscfg;
pub mod tim;
pub mod uart;
pub mod wwdt;

use crate::peripheral::Block;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 0 - Window WatchDog Interrupt
    WWDT = 0,
    /// 1 - PVD through EXTI Line detection Interrupt
    BOD = 1,
    /// 2 - RCC global Interrupt
    RCC = 2,
    /// 3 - EXTI Line0 Interrupt
    EXTI0 = 3,
    /// 4 - EXTI Line1 Interrupt
    EXTI1 = 4,
    /// 5 - EXTI Line2 Interrupt
    EXTI2 = 5,
    /// 6 - EXTI Line3 Interrupt
    EXTI3 = 6,
    /// 7 - MCM FAULT Interrupt
    MCM_FLT = 7,
    /// 8 - MCM PWM Interrupt
    MCM_PWM = 8,
    /// 9 - ADC Interrupt
    ADC = 9,
    /// 10 - PCA0 Interrupt
    PCA0 = 10,
    /// 12 - PCA0 Fault Interrupt
    PCA0_FLT = 12,
    /// 13 - CMP2 Interrupt
    CMP2 = 13,
    /// 14 - CMP0 Interrupt
    CMP0 = 14,
    /// 15 - CMP1 Interrupt
    CMP1 = 15,
    /// 16 - Reserved
    EXTI4 = 16,
    /// 17 - Reserved
    EXTI5 = 17,
    /// 18 - PWM3 Interrupt
    PWM3 = 18,
    /// 19 - SPI Interrupt
    SPI = 19,
    /// 20 - UART0 Interrupt
    UART0 = 20,
    /// 21 - UART1 Interrupt
    UART1 = 21,
    /// 22 - PWM4 Interrupt
    PWM4 = 22,
    /// 23 - TIM7 Interrupt
    TIM7 = 23,
    /// 24 - TIM8 Interrupt
    TIM8 = 24,
    /// 25 - EXTI Line6/7 Interrupt
    EXTI67 = 25,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Device {
    pub PCA0: Block<pca::PCA0, 0x40000400>,

    pub TIM7: Block<tim::TIM, 0x40000C00>,
    pub TIM8: Block<tim::TIM, 0x40001000>,

    pub UART0: Block<uart::UART, 0x40001800>,
    pub UART1: Block<uart::UART, 0x40001C00>,

    pub SPI0: Block<spi::SPI0, 0x40002400>,

    pub IWDT: Block<iwdt::IWDT, 0x40003000>,
    pub WWDT: Block<wwdt::WWDT, 0x40003400>,

    pub AMOC: Block<amoc::AMOC, 0x40003800>,

    pub EXTI: Block<exti::EXTI, 0x40004000>,

    pub PWM3: Block<pwm::PWM, 0x40005000>,
    pub PWM4: Block<pwm::PWM, 0x40005400>,

    pub N_TEST: Block<n_test::N_TEST, 0x4001F000>,

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

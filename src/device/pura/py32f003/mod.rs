pub mod adc;
pub mod flash;
pub mod gpio;
pub mod pwr;
pub mod rcc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    ///0 - Window WatchDog Interrupt
    WWDG = 0,
    ///1 - PVD Interrupt through EXTI Lines 16
    PVD = 1,
    ///2 - RTC Interrupt through EXTI Lines 19
    RTC = 2,
    ///3 - FLASH global Interrupt
    FLASH = 3,
    ///4 - RCC global Interrupt
    RCC = 4,
    ///5 - EXTI Line 0 and 1 Interrupt
    EXTI0_1 = 5,
    ///6 - EXTI Line 2 and 3 Interrupt
    EXTI2_3 = 6,
    ///7 - EXTI Line 4 to 15 Interrupt
    EXTI4_15 = 7,
    ///9 - DMA Channel 1 Interrupt
    DMA_CHANNEL1 = 9,
    ///10 - DMA Channel 2 and Channel 3 Interrupt
    DMA_CHANNEL2_3 = 10,
    ///12 - ADC and COMP Interrupt through EXTI Lines 17 and 18
    ADC_COMP = 12,
    ///13 - TIM1 Break, Update, Trigger and Commutation Interrupt
    TIM1_BRK_UP_TRG_COM = 13,
    ///14 - TIM1 Capture Compare Interrupt
    TIM1_CC = 14,
    ///16 - TIM3 global Interrupt
    TIM3 = 16,
    ///19 - TIM14 global Interrupt
    TIM14 = 19,
    ///21 - TIM16 global Interrupt
    TIM16 = 21,
    ///22 - TIM17 global Interrupt
    TIM17 = 22,
    ///23 - I2C1 global Interrupt
    I2C1 = 23,
    ///25 - SPI1 global Interrupt
    SPI1 = 25,
    ///27 - USART1 global Interrupt
    USART1 = 27,
    ///28 - USART2 global Interrupt
    USART2 = 28,
}

#[derive(Debug)]
pub struct Device {}

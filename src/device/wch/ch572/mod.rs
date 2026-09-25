pub mod cmp;
pub mod flash;
pub mod gpio;
pub mod i2c;
pub mod keyscan;
pub mod pwm;
pub mod spi;
pub mod sys;
pub mod tmr;
pub mod uart;
pub mod usb;

use crate::peripheral::Block;
use crate::peripheral::riscv32::qingke::csr::QingKeCSR;
use crate::peripheral::riscv32::qingke::pfic::PFIC;
use crate::peripheral::riscv32::qingke::systick::SysTick;

#[derive(Debug)]
pub struct Device {
    pub CSR: QingKeCSR,

    /// 系统配置寄存器组
    pub SYS: Block<sys::SYS, 0x40001000>,

    /// 安全访问、全局配置、振荡器校准寄存器组
    pub CFG: Block<sys::CFG, 0x40001040>,

    /// 比较器
    pub CMP: Block<cmp::CMP, 0x40001054>,

    /// 键盘扫描
    pub KEYSCAN: Block<keyscan::KEYSCAN, 0x40001064>,

    /// GPIO 中断控制
    pub GPIO_INT: Block<gpio::GPIO_INT, 0x40001090>,

    /// GPIO PA
    pub GPIOA: Block<gpio::GPIO, 0x400010A0>,

    /// Flash ROM 控制
    pub FLASH: Block<flash::FLASH, 0x40001800>,

    /// 定时器
    pub TMR: Block<tmr::TMR, 0x40002400>,

    /// 正交编码器
    pub ENC: Block<tmr::ENC, 0x40002420>,

    /// 串口
    pub UART: Block<uart::UART, 0x40003400>,

    /// SPI
    pub SPI: Block<spi::SPI, 0x40004000>,

    /// I2C
    pub I2C: Block<i2c::I2C, 0x40004800>,

    /// PWM1 ~ PWM5
    pub PWMX: Block<pwm::PWMX, 0x40005000>,

    /// USB 设备/主机
    pub USB: Block<usb::USB, 0x40008000>,

    pub PFIC: Block<PFIC, 0xE000E000>,
    pub SysTick: Block<SysTick, 0xE000F000>,
}

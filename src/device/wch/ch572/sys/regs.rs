use bitflags::bitflags;

use crate::register::{Reg, Reg8, Reg16, Reg32};

/// 系统配置寄存器组，基地址 `0x40001000`
#[derive(Debug)]
#[repr(C)]
pub struct SYS {
    /// 独立看门狗关键字寄存器
    pub IWDG_KR: Reg<IWDG_KR>, // 0x00

    /// 独立看门狗配置寄存器
    pub IWDG_CFG: Reg<IWDG_CFG>, // 0x04

    /// 系统时钟配置寄存器
    pub CLK_SYS_CFG: Reg<CLK_SYS_CFG>, // 0x08

    RESERVED0: Reg8, // 0x09

    /// 系统高频时钟电源配置寄存器
    pub HFCK_PWR_CTRL: Reg<HFCK_PWR_CTRL>, // 0x0A

    RESERVED1: Reg8, // 0x0B

    /// 睡眠时钟关闭控制字节 0
    pub SLP_CLK_OFF0: Reg<SLP_CLK_OFF0>, // 0x0C

    /// 睡眠时钟关闭控制字节 1
    pub SLP_CLK_OFF1: Reg<SLP_CLK_OFF1>, // 0x0D

    /// 睡眠唤醒控制寄存器
    pub SLP_WAKE_CTRL: Reg<SLP_WAKE_CTRL>, // 0x0E

    /// 外设掉电控制寄存器
    pub SLP_POWER_CTRL: Reg<SLP_POWER_CTRL>, // 0x0F

    RESERVED2: [Reg8; 8], // 0x10 ~ 0x17

    /// 功能引脚复用配置寄存器低位
    pub PIN_ALTERNATE: Reg<PIN_ALTERNATE>, // 0x18

    /// 功能引脚复用配置寄存器高位
    pub PIN_ALTERNATE_H: Reg<PIN_ALTERNATE_H>, // 0x1A

    /// 长复位配置寄存器
    pub LONG_RST_CFG: Reg<LONG_RST_CFG>, // 0x1C

    /// 睡眠时钟关闭控制字节 2
    pub SLP_CLK_OFF2: Reg<SLP_CLK_OFF2>, // 0x1D

    /// 睡眠辅助时钟配置寄存器
    pub SLP_WAKE_CFG: Reg<SLP_WAKE_CFG>, // 0x1E

    /// 睡眠前电源计划寄存器
    pub POWER_PLAN: Reg<POWER_PLAN>, // 0x20

    /// 辅助电源调整控制寄存器
    pub AUX_POWER_ADJ: Reg<AUX_POWER_ADJ>, // 0x22

    /// 电池电压检测控制寄存器
    pub BAT_DET_CTRL: Reg<BAT_DET_CTRL>, // 0x24

    /// 电池电压检测配置寄存器
    pub BAT_DET_CFG: Reg<BAT_DET_CFG>, // 0x25

    /// 电池电压检测状态寄存器
    pub BAT_STATUS: Reg<BAT_STATUS>, // 0x26

    RESERVED3: Reg8, // 0x27

    RESERVED4: [Reg8; 4], // 0x28 ~ 0x2B

    /// 内部 LSI 振荡器频率调整寄存器
    pub INT_LSI_TUNE: Reg<INT_LSI_TUNE>, // 0x2C

    RESERVED5: Reg8, // 0x2E

    /// LSI 振荡器配置寄存器
    pub LSI_CONFIG: Reg<LSI_CONFIG>, // 0x2F

    /// RTC 标志及清除控制寄存器
    pub RTC_FLAG_CTRL: Reg<RTC_FLAG_CTRL>, // 0x30

    /// RTC 模式控制寄存器
    pub RTC_MODE_CTRL: Reg<RTC_MODE_CTRL>, // 0x31

    RESERVED6: Reg16, // 0x32 ~ 0x33

    /// RTC 触发值寄存器
    pub RTC_TRIG: Reg32, // 0x34

    /// 基于 LSI 的 RTC 计数寄存器；高 16 位为 65536 分频周期
    pub RTC_CNT_LSI: Reg32, // 0x38

    /// 基于 2764800000 分频的 RTC 计数寄存器
    pub RTC_CNT_DIV2: Reg32, // 0x3C
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IWDG_KR: u32 {
        /// 解除写保护
        const KEY_UNPROTECT = 0x5555;
        /// 重装载计数值
        const KEY_RELOADING_COUNT = 0xAAAA;
        /// 启动看门狗
        const KEY_START_IWDG = 0xCCCC;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IWDG_CFG: u32 {
        /// 计数器重装载值
        const RLR_MASK = 0xFFF;
        /// 预分频系数
        const PR_MASK = 0x7 << 12;
        /// 配置寄存器更新标志
        const PVU = 1 << 15;
        /// 当前计数值
        const COUNT_MASK = 0xFFF << 16;
        /// 看门狗计数跟随内核停止
        const FOLLOW_CORE_STOP = 1 << 29;
        /// 写保护状态
        const WR_PROTECT = 1 << 30;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CLK_SYS_CFG: u8 {
        /// 系统时钟源模式
        const SYS_MOD_MASK = 0xC0;
        /// 由 32MHz 分频
        const SYS_MOD_CK32M = 0x00;
        /// 由 PLL-600MHz 分频
        const SYS_MOD_PLL = 0x40;
        /// 直接来自 LSI
        const SYS_MOD_LSI = 0xC0;
        /// PLL 或 CK32M 输出分频系数
        const PLL_DIV_MASK = 0x1F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct HFCK_PWR_CTRL: u8 {
        /// PLL 电源控制
        const CLK_PLL_PON = 1 << 4;
        /// 进入 halt 模式时禁止自动关闭外部 32MHz
        const CLK_XT32M_KEEP = 1 << 3;
        /// 外部 32MHz 电源控制
        const CLK_XT32M_PON = 1 << 2;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SLP_CLK_OFF0: u8 {
        /// 使能 KEYSCAN 唤醒
        const SLP_KEYSCAN_WAKE = 1 << 7;
        /// 置 1 关闭 UART 时钟
        const SLP_CLK_UART = 1 << 4;
        /// 置 1 关闭 CMP 时钟
        const SLP_CLK_CMP = 1 << 1;
        /// 置 1 关闭 TMR 时钟
        const SLP_CLK_TMR = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SLP_CLK_OFF1: u8 {
        /// 置 1 关闭 BLE 时钟
        const SLP_CLK_BLE = 1 << 7;
        /// 置 1 关闭 USB 时钟
        const SLP_CLK_USB = 1 << 4;
        /// 置 1 关闭 I2C 时钟
        const SLP_CLK_I2C = 1 << 3;
        /// 置 1 关闭 PWMx 时钟
        const SLP_CLK_PWMX = 1 << 2;
        /// 置 1 关闭 AES_CCM 时钟
        const CLK_OFF_AESCCM = 1 << 1;
        /// 置 1 关闭 SPI 时钟
        const SLP_CLK_SPI = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SLP_WAKE_CTRL: u8 {
        /// GPIO 唤醒模式
        const GPIO_WAKE_MODE = 1 << 7;
        /// 事件唤醒模式
        const WAKE_EV_MODE = 1 << 6;
        /// 使能 BAT 唤醒
        const SLP_BAT_WAKE = 1 << 5;
        /// 使能 GPIO 唤醒
        const SLP_GPIO_WAKE = 1 << 4;
        /// 使能 RTC 唤醒
        const SLP_RTC_WAKE = 1 << 3;
        /// GPIO 唤醒边沿选择
        const SLP_GPIO_EDGE_MODE = 1 << 2;
        /// 使能 ENCODER 唤醒
        const SLP_ENC_WAKE = 1 << 1;
        /// 使能 USB 唤醒
        const SLP_USB_WAKE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SLP_POWER_CTRL: u8 {
        /// SRAM 保持电压选择
        const RAM_RET_LV_MASK = 0xC0;
        /// 置 1 关闭主 SRAM 时钟
        const SLP_CLK_RAMX = 1 << 4;
        /// 唤醒等待时间选择
        const WAKE_DLY_MOD_MASK = 0x07;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PIN_ALTERNATE: u16 {
        /// 调试接口使能
        const PIN_DEBUG_EN = 1 << 14;
        /// USB 模拟 I/O 使能
        const PIN_USB_EN = 1 << 13;
        /// USB UDP 内部上拉电阻使能
        const UDP_PU_EN = 1 << 12;
        /// 置 1 关闭 PA 输入
        const PA_DI_DIS_MASK = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PIN_ALTERNATE_H: u16 {
        /// CLK25M 输出使能
        const EN_25M = 1 << 12;
        /// SPI CLK 复用引脚使能
        const SPI_CLK = 1 << 11;
        /// I2C 复用引脚使能
        const I2C_PIN_MASK = 0x0600;
        /// SPI CS 复用引脚使能
        const SPI_CS = 1 << 8;
        /// TIMER 复用引脚使能
        const TMR_PIN_MASK = 0x00C0;
        /// TXD 复用引脚使能
        const UART_TXD_MASK = 0x0038;
        /// RXD 复用引脚使能
        const UART_RXD_MASK = 0x0007;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SLP_WAKE_CFG: u16 {
        /// 使能安全访问自动关闭
        const ACAUTO_ENABLE = 1 << 8;
        /// 预时钟计数值选择
        const PRECLK_CNT_SEL_MASK = 0x0060;
        /// 唤醒后等待预时钟计数完成再释放 flck
        const PRECLK_CNT_EN = 1 << 4;
        /// halt 睡眠时强制 OSC READY
        const OSCCLK_RDY_KEEP = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SLP_CLK_OFF2: u8 {
        /// 置 1 关闭 XROM hclk
        const CLK_OFF_HCLK = 1 << 4;
        /// 置 1 关闭 2 线调试时钟
        const CLK_OFF_DEBUG = 1 << 2;
        /// 置 1 关闭 XROM 64M/600M 时钟
        const CLK_OFF_XROM = 1 << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LONG_RST_CFG: u8 {
        /// 长复位时间选择：11=32.768ms，10=25ms，01=20ms，00=15ms
        const LONG_TIM_SEL_MASK = 0x06;
        /// 长复位使能
        const LONG_RST_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct POWER_PLAN: u16 {
        /// 电源计划使能，睡眠执行后自动清除
        const PWR_PLAN_EN = 1 << 15;
        /// 外部 32MHz 提前唤醒使能
        const XT_PRE_EN = 1 << 14;
        /// 必须写 0
        const PWR_MUST_0 = 1 << 13;
        /// 内部 LDO5v 使能
        const PWR_LDO5V_EN = 1 << 8;
        /// 系统供电
        const PWR_SYS_EN = 1 << 7;
        /// 主电源选择
        const MAIN_ACT = 1 << 6;
        /// USB 和 BLE 单元保留区域供电
        const PWR_EXTEND = 1 << 3;
        /// 内核和基础外设保留区域供电
        const PWR_CORE = 1 << 2;
        /// 12KB 保持 SRAM 供电
        const PWR_RAM12K = 1 << 1;
        /// Flash ROM 供电
        const PWR_XROM = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AUX_POWER_ADJ: u16 {
        /// I/V 参考配置数据
        const CFG_IVREF_MASK = 0x0F00;
        /// 超低功耗 LDO 电压调整
        const ULPLDO_ADJ_MASK = 0x0007;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BAT_DET_CTRL: u8 {
        /// 电池低压中断使能
        const BAT_LOW_IE = 1 << 3;
        /// 睡眠模式下电池电压监控使能
        const BAT_MON_EN = 1 << 1;
        /// LDO 使能
        const PWR_LDO_EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BAT_DET_CFG: u8 {
        /// 电池低压检测/监控阈值电压选择
        const BAT_LOW_VTH_MASK = 0x03;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BAT_STATUS: u8 {
        /// 电池低压状态，高有效
        const BAT_STAT_LOW = 1 << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INT_LSI_TUNE: u16 {
        /// LSI 振荡器频率调整值
        const TUNE_MASK = 0x1FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LSI_CONFIG: u8 {
        /// LSI 振荡器时钟引脚状态
        const LSI_CLK_PIN = 1 << 7;
        /// LSI 振荡器上电
        const CLK_LSI_PON = 1 << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTC_FLAG_CTRL: u8 {
        /// RTC 触发动作标志
        const RTC_TRIG_FLAG = 1 << 7;
        /// RTC 定时动作标志
        const RTC_TMR_FLAG = 1 << 6;
        /// 置 1 清除 RTC 触发动作标志，自动清除
        const RTC_TRIG_CLR = 1 << 5;
        /// 置 1 清除 RTC 定时动作标志，自动清除
        const RTC_TMR_CLR = 1 << 4;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RTC_MODE_CTRL: u8 {
        /// 置 1 装载 RTC 计数高字，装载后自动清除
        const RTC_LOAD_HI = 1 << 7;
        /// 置 1 装载 RTC 计数低字，装载后自动清除
        const RTC_LOAD_LO = 1 << 6;
        /// RTC 触发模式使能
        const RTC_TRIG_EN = 1 << 5;
        /// RTC 定时模式使能
        const RTC_TMR_EN = 1 << 4;
        /// 触发模式忽略 bit0
        const RTC_IGNORE_B0 = 1 << 3;
        /// RTC 定时模式：000=1，001=4096，010=12288，011=28672，100=61440，101=122880，110=245760，111=491520
        const RTC_TMR_MODE_MASK = 0x07;
    }
}

/// 系统配置寄存器组，基地址 `0x40001040`
#[derive(Debug)]
#[repr(C)]
pub struct CFG {
    /// 安全访问签名寄存器
    pub SAFE_ACCESS_SIG: Reg<SAFE_ACCESS_SIG>, // 0x00

    /// 芯片 ID 寄存器，固定为 `ID_CH57*`
    pub CHIP_ID: Reg8, // 0x01

    /// 安全访问 ID 寄存器，固定为 `0x0C`
    pub SAFE_ACCESS_ID: Reg8, // 0x02

    /// 看门狗计数寄存器，计数频率 `Fsys/131072`
    pub WDOG_COUNT: Reg8, // 0x03

    /// 复位状态寄存器
    pub RESET_STATUS: Reg<RESET_STATUS>, // 0x04

    /// 全局配置信息及状态寄存器
    pub GLOB_CFG_INFO: Reg<GLOB_CFG_INFO>, // 0x05

    /// 复位及看门狗控制寄存器
    pub RST_WDOG_CTRL: Reg<RST_WDOG_CTRL>, // 0x06

    /// 全局复位时的值保持寄存器
    pub GLOB_RESET_KEEP: Reg8, // 0x07

    /// 杂项控制寄存器，高字节为 PLL 配置，见 [`CFG::PLL_CONFIG`]
    pub MISC_CTRL: Reg32, // 0x08

    RESERVED0: Reg16, // 0x0C ~ 0x0D

    /// 外部 32MHz 振荡器调整控制寄存器
    pub XT32M_TUNE: Reg<XT32M_TUNE>, // 0x0E

    RESERVED1: Reg8, // 0x0F

    /// 振荡器频率校准计数寄存器
    pub OSC_CAL_CNT: Reg<OSC_CAL_CNT>, // 0x10

    /// 振荡器频率校准溢出次数寄存器
    pub OSC_CAL_OV_CNT: Reg8, // 0x12

    /// 振荡器频率校准控制寄存器
    pub OSC_CAL_CTRL: Reg<OSC_CAL_CTRL>, // 0x13

    RESERVED2: Reg32, // 0x14 ~ 0x17，CMP 寄存器区

    /// 安全访问签名寄存器 2
    pub SAFE_ACCESS_SIG2: Reg<SAFE_ACCESS_SIG2>, // 0x18
}

impl CFG {
    /// `R8_GLOB_ROM_CFG`，与 [`CFG::RESET_STATUS`] 同地址
    pub const fn GLOB_ROM_CFG(&self) -> &Reg<GLOB_ROM_CFG> {
        unsafe { Reg::from_ptr(self.RESET_STATUS.as_ptr() as *mut GLOB_ROM_CFG) }
    }

    /// `R8_PLL_CONFIG`，与 [`CFG::MISC_CTRL`] 高字节同地址
    pub const fn PLL_CONFIG(&self) -> &Reg<PLL_CONFIG> {
        unsafe { Reg::from_ptr((self.MISC_CTRL.as_ptr() as *mut u8).add(3) as *mut PLL_CONFIG) }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SAFE_ACCESS_SIG: u8 {
        /// 当前安全访问模式：11=已解锁
        const SAFE_ACC_MODE_MASK = 0x03;
        /// 安全访问已开启
        const SAFE_ACC_ACT = 1 << 2;
        /// 安全访问模式关闭
        const SAFE_ACC_TIMER_MASK = 0xF8;
        /// 安全访问签名值第 1 步
        const SIG1 = 0x57;
        /// 安全访问签名值第 2 步
        const SIG2 = 0xA8;
        /// 关闭安全访问的签名值
        const SIG0 = 0x00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RESET_STATUS: u8 {
        /// 最近一次复位标志
        const RESET_FLAG_MASK = 0x07;
        /// 软件复位
        const RST_FLAG_SW = 0x00;
        /// 真实上电复位
        const RST_FLAG_RPOR = 0x01;
        /// 看门狗超时复位
        const RST_FLAG_WTR = 0x02;
        /// RST 引脚外部手动复位
        const RST_FLAG_MR = 0x03;
        /// 睡眠/关机期间全局复位
        const RST_FLAG_GPWSM = 0x05;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GLOB_ROM_CFG: u8 {
        /// 使能 Flash ROM 代码区擦写：X0=代码写保护，01=使能 129~240K 区，11=使能 0~240K 区
        const ROM_CODE_WE_MASK = 0xC0;
        /// 使能 Flash ROM 控制接口
        const ROM_CTRL_EN = 0x20;
        /// Flash ROM 代码偏移地址选择：0=0x000000，1=0x008000
        const ROM_CODE_OFS = 0x10;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GLOB_CFG_INFO: u8 {
        /// 引导程序状态：0=应用程序，1=BootLoader
        const BOOT_LOADER = 0x20;
        /// 复位引脚选择：1=PA7，0=PA8
        const CFG_RST_PIN = 0x10;
        /// BootLoader 使能状态
        const CFG_BOOT_EN = 0x08;
        /// 外部手动复位输入使能状态
        const CFG_RESET_EN = 0x04;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RST_WDOG_CTRL: u8 {
        /// 看门狗溢出中断标志，写 1 清除
        const WDOG_INT_FLAG = 1 << 4;
        /// 看门狗溢出中断使能
        const WDOG_INT_EN = 1 << 2;
        /// 看门狗溢出复位使能
        const WDOG_RST_EN = 1 << 1;
        /// 手动进入 BootLoader 标志
        const BOOT_LOAD_MAN = 1 << 0;
        /// 全局软件复位，高有效，自动清除
        const SOFTWARE_RESET = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PLL_CONFIG: u8 {
        /// PLL 配置控制数据
        const PLL_CFG_DAT_MASK = 0x7F;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct XT32M_TUNE: u8 {
        /// 外部 32MHz 负载电容调整：Cap = C_LOAD * 2 + 10pF
        const XT32M_C_LOAD_MASK = 0x70;
        /// 外部 32MHz 偏置电流调整：00=75%，01=标准，10=125%，11=150%
        const XT32M_I_BIAS_MASK = 0x03;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OSC_CAL_CNT: u16 {
        /// 振荡器捕获结束中断标志，写 1 清除
        const OSC_CAL_IF = 1 << 15;
        /// 溢出计数非零标志，写 1 清除
        const OSC_CAL_OV_CLR = 1 << 14;
        /// LSI 多个周期内的系统时钟计数值
        const OSC_CAL_CNT_MASK = 0x3FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OSC_CAL_CTRL: u8 {
        /// 复位振荡器校准计数值
        const CNT_CLR = 1 << 7;
        /// 振荡器捕获结束模式：0=正常，1=追加 2 个周期
        const OSC_CNT_END = 1 << 6;
        /// 校准计数器使能
        const OSC_CNT_EN = 1 << 5;
        /// 振荡器捕获结束中断使能
        const OSC_CAL_IE = 1 << 4;
        /// 校准计数器暂停状态
        const OSC_CNT_HALT = 1 << 3;
        /// 振荡器捕获总周期数：000=1，001=2，010=4，011=32，100=64，101=128，110=1024，111=2047
        const OSC_CNT_TOTAL_MASK = 0x07;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SAFE_ACCESS_SIG2: u32 {
        /// 功能使能
        const FUN_MODE_MASK = 0x07 << 24;
        /// 2 线 Flash 禁止操作标志
        const FLASH_HALTED = 1 << 23;
        /// 厂商配置字锁定标志
        const MANU_CFG_LOCK = 1 << 22;
        /// Flash 读保护标志
        const RD_PROTECT = 1 << 21;
        /// 安全寄存器自动关闭禁止标志
        const SAFE_AC_DIS = 1 << 20;
    }
}

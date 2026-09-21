mod regs;

pub use regs::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Clocks {
    pub SYSCLK: u32,
    pub HCLK: u32,
    pub Core: [u32; 2],
    pub ADCCLK: u32,
}

const PLLMULTable: [u8; 32] = [
    4, 6, 7, 8, 17, 9, 19, 10, 21, 11, 23, 12, 25, 13, 14, 15, 16, 17, 18, 19, 20, 22, 24, 26, 28,
    30, 32, 34, 36, 38, 40, 59,
];
const HBPrescTable: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9];
const SERDESPLLMULTable: [u8; 16] = [
    25, 28, 30, 32, 35, 38, 40, 45, 50, 56, 60, 64, 70, 76, 80, 90,
];
const FPRETable: [u8; 4] = [0, 1, 2, 2];
const PPRE2Table: [u8; 8] = [0, 0, 0, 0, 1, 2, 3, 4];
const ADCPRETable: [u8; 4] = [2, 4, 6, 8];

impl RCC {
    pub const HSI_VALUE: u32 = 25_000_000;
    pub const HSE_VALUE: u32 = 25_000_000;

    pub fn DeInit(&self) {
        self.CTLR.bit_or(CTLR::HSION);
        self.CFGR0.bit_and(CFGR0::from_bits_retain(0x305C0000));
        while self.CFGR0.get().intersects(CFGR0::SWS_MASK) {}
        self.CFGR0.bit_and(CFGR0::from_bits_retain(0xFFBFFFFF));

        self.PLLCFGR.bit_and(PLLCFGR::from_bits_retain(0x7FFFFFFF));

        self.CTLR.bit_and(CTLR::from_bits_retain(0x6AA6FFFF));
        self.CTLR.bit_and(CTLR::from_bits_retain(0xFFFBFFFF));

        self.PLLCFGR.bit_and(PLLCFGR::from_bits_retain(0x0FFFC000));
        self.PLLCFGR.bit_or(PLLCFGR::from_bits_retain(0x00000004));

        self.INTR.set(INTR::from_bits_retain(0x00FF0000));
        self.CFGR2.bit_and(CFGR2::from_bits_retain(0x0C600000));
        self.PLLCFGR2
            .bit_and(PLLCFGR2::from_bits_retain(0xFFF0E080));
        self.PLLCFGR2.bit_or(PLLCFGR2::from_bits_retain(0x00080020));
    }

    pub fn HSEConfig(&self, HSEON_HSEBYP: CTLR) {
        self.CTLR
            .bit_clear_or(CTLR::HSEON | CTLR::HSEBYP, HSEON_HSEBYP);
    }

    pub fn WaitForHSEStartUp(&self) -> bool {
        for _ in 0..0x1000000 {
            if self.CTLR.get().intersects(CTLR::HSERDY) {
                break;
            }
        }
        self.CTLR.get().intersects(CTLR::HSERDY)
    }

    pub fn AdjustHSICalibrationValue(&self, HSICalibrationValue: u8) {
        self.CTLR.bit_clear_or(
            CTLR::HSITRIM_MASK,
            CTLR::HSITRIM(HSICalibrationValue as u32),
        );
    }

    pub fn HSICmd(&self, NewState: bool) {
        self.CTLR.bit_set(CTLR::HSION, NewState);
    }

    pub fn PLLConfig(&self, PLL_SRC_DIV_MUL: PLLCFGR) {
        self.PLLCFGR.bit_clear_or(
            PLLCFGR::PLLSRC_MASK | PLLCFGR::PLL_SRC_DIV_MASK | PLLCFGR::PLLMUL_MASK,
            PLL_SRC_DIV_MUL,
        );
    }

    pub fn PLLCmd(&self, NewState: bool) {
        self.CTLR.bit_set(CTLR::PLLON, NewState);
    }

    pub fn SYSCLKConfig(&self, SW: CFGR0) {
        self.CFGR0.bit_clear_or(CFGR0::SW_MASK, SW);
    }

    pub fn GetSYSCLKSource(&self) -> CFGR0 {
        self.CFGR0.get() & CFGR0::SWS_MASK
    }

    pub fn HCLKSourceDivConfig(&self, HPRE_FPRE: CFGR0) {
        self.CFGR0
            .bit_clear_or(CFGR0::HPRE_MASK | CFGR0::FPRE_MASK, HPRE_FPRE);
    }

    pub fn ITConfig(&self, IT: INT, NewState: bool) {
        self.INTE().bit_set(IT, NewState);
    }

    pub fn ADCUSBHSPLLCLKAsSourceConfig(&self, PPRE2: CFGR0) {
        self.CFGR0
            .bit_clear_or(CFGR0::PPRE2_MASK | CFGR0::ADCPRE_MASK, PPRE2);
    }

    pub fn ADCHCLKCLKAsSourceConfig(&self, PPRE2_ADCDIV: CFGR0) {
        self.CFGR0
            .bit_clear_or(CFGR0::PPRE2_MASK | CFGR0::ADCPRE_MASK, PPRE2_ADCDIV);
    }

    pub fn LSEConfig(&self, LSEON_LSEBYP: BDCTLR) {
        self.BDCTLR.bit_clear(BDCTLR::LSEON);
        self.BDCTLR
            .bit_clear_or(BDCTLR::LSEON | BDCTLR::LSEBYP, LSEON_LSEBYP);
    }

    pub fn LSICmd(&self, NewState: bool) {
        self.RSTSCKR.bit_set(RSTSCKR::LSION, NewState);
    }

    pub fn RTCCLKConfig(&self, RTCSEL: BDCTLR) {
        self.BDCTLR.bit_clear_or(BDCTLR::RTCSEL_MASK, RTCSEL);
    }

    pub fn RTCCLKCmd(&self, NewState: bool) {
        self.BDCTLR.bit_set(BDCTLR::RTCEN, NewState);
    }

    pub fn GetClocksFreq(&self) -> Clocks {
        let mut clocks = Clocks::default();
        match self.CFGR0.get() & CFGR0::SWS_MASK {
            CFGR0::SWS_HSI => {
                clocks.SYSCLK = RCC::HSI_VALUE;
            }
            CFGR0::SWS_HSE => {
                clocks.SYSCLK = RCC::HSE_VALUE;
            }
            _ => match self.PLLCFGR.get() & PLLCFGR::SYSPLL_SEL_MASK {
                PLLCFGR::SYSPLL_SEL_PLL_CLK => {
                    let pllcfgr = self.PLLCFGR.get();
                    let pllmul = pllcfgr & PLLCFGR::PLLMUL_MASK;
                    let presc = ((pllcfgr & PLLCFGR::PLL_SRC_DIV_MASK).bits() >> 8) + 1;
                    let pllsrc;
                    match pllcfgr & PLLCFGR::PLLSRC_MASK {
                        PLLCFGR::PLLSRC_SERDES_PLL_DIV_2 => {
                            pllsrc = Self::HSE_VALUE
                                * SERDESPLLMULTable[(self.PLLCFGR2.get()
                                    & PLLCFGR2::SERDESPLL_MUL_MASK)
                                    .bits()
                                    as usize] as u32
                                / 2;
                        }
                        PLLCFGR::PLLSRC_USBSS_PLL => {
                            pllsrc = 125_000_000;
                        }
                        PLLCFGR::PLLSRC_ETH_PLL => {
                            pllsrc = 500_000_000;
                        }
                        PLLCFGR::PLLSRC_USBHS_PLL => {
                            pllsrc = 480_000_000;
                        }
                        PLLCFGR::PLLSRC_HSE => {
                            pllsrc = Self::HSE_VALUE;
                        }
                        PLLCFGR::PLLSRC_HSI => {
                            pllsrc = Self::HSI_VALUE;
                        }
                        _ => {
                            pllsrc = RCC::HSI_VALUE;
                        }
                    }
                    let pllsrc_tmp = pllsrc / presc * PLLMULTable[pllmul.bits() as usize] as u32;
                    match pllmul {
                        PLLCFGR::PLLMUL_12_5
                        | PLLCFGR::PLLMUL_11_5
                        | PLLCFGR::PLLMUL_10_5
                        | PLLCFGR::PLLMUL_9_5
                        | PLLCFGR::PLLMUL_8_5 => {
                            clocks.SYSCLK = pllsrc_tmp >> 1;
                        }
                        _ => {
                            clocks.SYSCLK = pllsrc_tmp;
                        }
                    }
                }
                PLLCFGR::SYSPLL_SEL_USBHS_PLL => {
                    clocks.SYSCLK = 480_000_000;
                }
                PLLCFGR::SYSPLL_SEL_ETH_PLL => {
                    clocks.SYSCLK = 500_000_000;
                }
                PLLCFGR::SYSPLL_SEL_SERDES_PLL_DIV_2 => {
                    clocks.SYSCLK = Self::HSE_VALUE
                        * SERDESPLLMULTable
                            [(self.PLLCFGR2.get() & PLLCFGR2::SERDESPLL_MUL_MASK).bits() as usize]
                            as u32;
                }
                PLLCFGR::SYSPLL_SEL_USBSS_PLL => {
                    clocks.SYSCLK = 125_000_000;
                }
                _ => {
                    clocks.SYSCLK = RCC::HSI_VALUE;
                }
            },
        }
        let cfgr0 = self.CFGR0.get();
        let high_bus =
            clocks.SYSCLK >> HBPrescTable[((cfgr0 & CFGR0::HPRE_MASK).bits() >> 4) as usize];
        clocks.HCLK = high_bus >> FPRETable[((cfgr0 & CFGR0::FPRE_MASK).bits() >> 16) as usize];
        clocks.Core[0] = clocks.HCLK;
        clocks.Core[1] = high_bus;
        if cfgr0.intersects(CFGR0::ADCSRC) {
            clocks.ADCCLK = 480_000_000 / (((cfgr0 & CFGR0::PPRE2_MASK).bits() >> 11) + 5);
        } else {
            clocks.ADCCLK = (clocks.HCLK
                >> (PPRE2Table[((cfgr0 & CFGR0::PPRE2_MASK).bits() >> 11) as usize]))
                / (ADCPRETable[((cfgr0 & CFGR0::ADCPRE_MASK).bits() >> 14) as usize] as u32);
        }
        clocks
    }

    pub fn HBPeriphClockCmd(&self, HBPeriph: HBPCENR, NewState: bool) {
        self.HBPCENR.bit_set(HBPeriph, NewState);
    }

    pub fn HB2PeriphClockCmd(&self, HB2Periph: HB2PCENR, NewState: bool) {
        self.HB2PCENR.bit_set(HB2Periph, NewState);
    }

    pub fn HB1PeriphClockCmd(&self, HB1Periph: HB1PCENR, NewState: bool) {
        self.HB1PCENR.bit_set(HB1Periph, NewState);
    }

    pub fn HBPeriphResetCmd(&self, HBPeriph: HBRSTR, NewState: bool) {
        self.HBRSTR.bit_set(HBPeriph, NewState);
    }

    pub fn HB2PeriphResetCmd(&self, HB2Periph: HB2PRSTR, NewState: bool) {
        self.HB2PRSTR.bit_set(HB2Periph, NewState);
    }

    pub fn HB1PeriphResetCmd(&self, HB1Periph: HB1PRSTR, NewState: bool) {
        self.HB1PRSTR.bit_set(HB1Periph, NewState);
    }

    pub fn BackupResetCmd(&self, NewState: bool) {
        self.BDCTLR.bit_set(BDCTLR::BDRST, NewState);
    }

    pub fn ClockSecuritySystemCmd(&self, CSSHSEDIS_CSSON: CTLR) {
        self.CTLR
            .bit_clear_or(CTLR::CSS_HSE_DIS | CTLR::CSSON, CSSHSEDIS_CSSON);
    }

    pub fn MCOConfig(&self, MCO: CFGR0) {
        self.CFGR0.bit_clear_or(CFGR0::MCO_MASK, MCO);
    }

    pub fn ClearResetFlag(&self) {
        self.RSTSCKR.bit_or(RSTSCKR::RMVF);
    }

    pub fn GetITStatus(&self) -> INT {
        self.INTF().get()
    }

    pub fn ClearITPendingBit(&self, IT: INT) {
        self.INTC().set(IT);
    }

    pub fn USBHS_PLLCmd(&self, NewState: bool) {
        self.CTLR.bit_set(CTLR::USBHS_PLLON, NewState);
    }

    pub fn USBSS_PLLCmd(&self, NewState: bool) {
        self.CTLR.bit_set(CTLR::USBSS_PLLON, NewState);
    }

    pub fn ETH_PLLCmd(&self, NewState: bool) {
        self.CTLR.bit_set(CTLR::ETH_PLLON, NewState);
    }

    pub fn SERDES_PLLCmd(&self, NewState: bool) {
        self.CTLR.bit_set(CTLR::SERDES_PLLON, NewState);
    }

    pub fn PIPECmd(&self, NewState: bool) {
        self.CFGR0.bit_set(CFGR0::PIPEON, NewState);
    }
}

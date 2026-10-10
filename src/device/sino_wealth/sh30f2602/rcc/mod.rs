mod regs;

pub use regs::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct RCC_Clocks {
    pub sys_clk: f32,
    pub ahb_clk: f32,
    pub apb_clk: f32,
}

impl RCC {
    #[inline(always)]
    pub fn WriteEnable(&self) {
        self.RCCLOCK.set(RCCLOCK::LOCK_VALUE);
    }

    #[inline(always)]
    pub fn WriteDisable(&self) {
        self.RCCLOCK.set(RCCLOCK::empty());
    }

    /// 6MHz内部时钟
    pub const HSI_CLK: f32 = 6_000_000.0;

    /// 128KHz内部时钟
    pub const LSI_CLK: f32 = 128_000.0;

    pub fn GetClocksFreq(&self) -> RCC_Clocks {
        let sys_clk;
        let ahb_clk;
        let apb_clk;

        let cfgr = self.CFGR.get();

        match self.CR.get() & CR::SWS_MASK {
            CR::SWS_HSI | CR::SWS_HSI_ => sys_clk = RCC::HSI_CLK,
            CR::SWS_PLL | CR::SWS_PLL_ => {
                let f = ((cfgr & CFGR::PLLF_MASK).bits() >> 12) + 15;
                let k = ((cfgr & CFGR::PLLK_MASK).bits() >> 9) + 1;
                sys_clk = RCC::HSI_CLK * f as f32 / k as f32 / 2.0;
            }
            _ => unreachable!(),
        }

        let ahb_div = 1 << ((cfgr & CFGR::HPRE_MASK).bits() >> 0).min(4);
        ahb_clk = sys_clk / ahb_div as f32;

        let apb_div = 1 << ((cfgr & CFGR::PPRE_MASK).bits() >> 3).min(5);
        apb_clk = ahb_clk / apb_div as f32;

        RCC_Clocks {
            sys_clk,
            ahb_clk,
            apb_clk,
        }
    }
}

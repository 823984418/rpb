use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct LTDC {
    pub SSCR: Reg<SSCR>,
    pub BPCR: Reg<BPCR>,
    pub AWCR: Reg<AWCR>,
    pub TWCR: Reg<TWCR>,
    pub GCR: Reg<GCR>,
    pub SRCR: Reg<SRCR>,
    pub BCCR: Reg<BCCR>,
    pub IER: Reg<IER>,
    pub ISR: Reg<ISR>,
    pub ICR: Reg<ICR>,
    pub LIPCR: Reg<LIPCR>,
    pub CPSR: Reg<CPSR>,
    pub CDSR: Reg<CDSR>,
}

#[derive(Debug)]
#[repr(C)]
pub struct LTDC_Layer {
    pub CR: Reg<CR>,
    pub WHPCR: Reg<WHPCR>,
    pub WVPCR: Reg<WVPCR>,
    pub CKCR: Reg<CKCR>,
    pub PFCR: Reg<PFCR>,
    pub CACR: Reg<CACR>,
    pub DCCR: Reg<DCCR>,
    pub BFCR: Reg<BFCR>,
    pub CFBAR: Reg<CFBAR>,
    pub CFBLR: Reg<CFBLR>,
    pub CFBLNR: Reg<CFBLNR>,
    pub CLUTWR: Reg<CLUTWR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SSCR: u32 {
        const HSW_MASK = 0xFFF << 16;
        const VSH_MASK = 0x7FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BPCR: u32 {
        const AHBP_MASK = 0xFFF << 16;
        const AVBP_MASK = 0x7FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWCR: u32 {
        const AAW_MASK = 0xFFF << 16;
        const AAH_MASK = 0x7FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TWCR: u32 {
        const TOTALW_MASK = 0xFFF << 16;
        const TOTALH_MASK = 0x7FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GCR: u32 {
        const HSPOL = 1 << 31;
        const VSPOL = 1 << 30;
        const DEPOL = 1 << 29;
        const PCPOL = 1 << 28;

        const DRW_2 = 1 << 14;
        const DRW_1 = 1 << 13;
        const DRW_0 = 1 << 12;
        const DRW_MASK = 0x7 << 12;

        const DGW_2 = 1 << 10;
        const DGW_1 = 1 << 9;
        const DGW_0 = 1 << 8;
        const DGW_MASK = 0x7 << 8;

        const DBW_2 = 1 << 6;
        const DBW_1 = 1 << 5;
        const DBW_0 = 1 << 4;
        const DBW_MASK = 0x7 << 4;

        const LTDCEN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SRCR: u32 {
        const VBR = 1 << 1;
        const IMR = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BCCR: u32 {
        const BCRED_MASK = 0xFF << 16;
        const BCGREEN_MASK = 0xFF << 8;
        const BCBLUE_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u32 {
        const RRIE = 1 << 3;
        const FUIE = 1 << 1;
        const LIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const RRIF = 1 << 3;
        const FUIF = 1 << 1;
        const LIF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ICR: u32 {
        const CRRIF = 1 << 3;
        const CFUIF = 1 << 1;
        const CLIF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LIPCR: u32 {
        const LIPOS_MASK = 0x7FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CPSR: u32 {
        const CXPOS_MASK = 0xFFFF << 16;
        const CYPOS_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CDSR: u32 {
        const HSYNCS = 1 << 3;
        const VSYNCS = 1 << 2;
        const HDES = 1 << 1;
        const VDES = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const CLUTEN = 1 << 4;

        const COLKEN = 1 << 1;
        const LEN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WHPCR: u32 {
        const WHSPPOS_MASK = 0xFFF << 16;
        const WHSTPOS_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WVPCR: u32 {
        const WVSPPOS_MASK = 0x7FF << 16;
        const WVSTPOS_MASK = 0x7FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CKCR: u32 {
        const CKRED_MASK = 0xFF << 16;
        const CKGREEN_MASK = 0xFF << 8;
        const CKBLUE_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PFCR: u32 {
        const PF_2 = 1 << 2;
        const PF_1 = 1 << 1;
        const PF_0 = 1 << 0;

        const PF_AL88 = 0b111 << 0;
        const PF_AL44 = 0b110 << 0;
        const PF_L8 = 0b101 << 0;
        const PF_ARGB4444 = 0b100 << 0;
        const PF_ARGB1555 = 0b011 << 0;
        const PF_RGB565 = 0b010 << 0;
        const PF_RGB888 = 0b001 << 0;
        const PF_ARGB8888 = 0b000 << 0;
        const PF_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CACR: u32 {
        const CONSTA_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DCCR: u32 {
        const DCALPHA_MASK = 0xFF << 24;
        const DCRED_MASK = 0xFF << 16;
        const DCGREEN_MASK = 0xFF << 8;
        const DCBLUE_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BFCR: u32 {
        const BF1_2 = 1 << 10;
        const BF1_1 = 1 << 9;
        const BF1_0 = 1 << 8;
        const BF1_MASK = 0x7 << 8;

        const BF2_2 = 1 << 2;
        const BF2_1 = 1 << 1;
        const BF2_0 = 1 << 0;
        const BF2_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFBAR: u32 {
        const CFBADD_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFBLR: u32 {
        const CFBP_MASK = 0x1FFF << 16;
        const CFBLL_MASK = 0x1FFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFBLNR: u32 {
        const CFBLNBR_MASK = 0x7FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CLUTWR: u32 {
        const CLUTADD_MASK = 0xFF << 24;
        const RED_MASK = 0xFF << 16;
        const GREEN_MASK = 0xFF << 8;
        const BLUE_MASK = 0xFF << 0;
    }
}

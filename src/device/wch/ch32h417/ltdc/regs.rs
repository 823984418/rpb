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
        const VSH = 0x000007FF;
        const HSW = 0x0FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BPCR: u32 {
        const AVBP = 0x000007FF;
        const AHBP = 0x0FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWCR: u32 {
        const AAH = 0x000007FF;
        const AAW = 0x0FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TWCR: u32 {
        const TOTALH = 0x000007FF;
        const TOTALW = 0x0FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct GCR: u32 {
        const LTDCEN = 0x00000001;

        const DBW = 0x00000070;
        const DBW_0 = 0x00000010;
        const DBW_1 = 0x00000020;
        const DBW_2 = 0x00000040;

        const DGW = 0x00000700;
        const DGW_0 = 0x00000100;
        const DGW_1 = 0x00000200;
        const DGW_2 = 0x00000400;

        const DRW = 0x00007000;
        const DRW_0 = 0x00001000;
        const DRW_1 = 0x00002000;
        const DRW_2 = 0x00004000;

        const PCPOL = 0x10000000;
        const DEPOL = 0x20000000;
        const VSPOL = 0x40000000;
        const HSPOL = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SRCR: u32 {
        const IMR = 0x00000001;
        const VBR = 0x00000002;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BCCR: u32 {
        const BCBLUE = 0x000000FF;
        const BCGREEN = 0x0000FF00;
        const BCRED = 0x00FF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IER: u32 {
        const LIE = 0x00000001;
        const FUIE = 0x00000002;

        const RRIE = 0x00000008;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const LIF = 0x00000001;
        const FUIF = 0x00000002;

        const RRIF = 0x00000008;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ICR: u32 {
        const CLIF = 0x00000001;
        const CFUIF = 0x00000002;

        const CRRIF = 0x00000008;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LIPCR: u32 {
        const LIPOS = 0x000007FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CPSR: u32 {
        const CYPOS = 0x0000FFFF;
        const CXPOS = 0xFFFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CDSR: u32 {
        const VDES = 0x00000001;
        const HDES = 0x00000002;
        const VSYNCS = 0x00000004;
        const HSYNCS = 0x00000008;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR: u32 {
        const LEN = 0x00000001;
        const COLKEN = 0x00000002;

        const CLUTEN = 0x00000010;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WHPCR: u32 {
        const WHSTPOS = 0x00000FFF;
        const WHSPPOS = 0x0FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WVPCR: u32 {
        const WVSTPOS = 0x000007FF;
        const WVSPPOS = 0x07FF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CKCR: u32 {
        const CKBLUE = 0x000000FF;
        const CKGREEN = 0x0000FF00;
        const CKRED = 0x00FF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PFCR: u32 {
        const PF = 0x00000007;
        const PF_0 = 0x00000001;
        const PF_1 = 0x00000002;
        const PF_2 = 0x00000004;

        const PF_ARGB8888 = 0x00000000;
        const PF_RGB888 = 0x00000001;
        const PF_RGB565 = 0x00000002;
        const PF_ARGB1555 = 0x00000003;
        const PF_ARGB4444 = 0x00000004;
        const PF_L8 = 0x00000005;
        const PF_AL44 = 0x00000006;
        const PF_AL88 = 0x00000007;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CACR: u32 {
        const CONSTA = 0x000000FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DCCR: u32 {
        const DCBLUE = 0x000000FF;
        const DCGREEN = 0x0000FF00;
        const DCRED = 0x00FF0000;
        const DCALPHA = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BFCR: u32 {
        const BF2 = 0x00000007;
        const BF2_0 = 0x00000001;
        const BF2_1 = 0x00000002;
        const BF2_2 = 0x00000004;

        const BF1 = 0x00000700;
        const BF1_0 = 0x00000100;
        const BF1_1 = 0x00000200;
        const BF1_2 = 0x00000400;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFBAR: u32 {
        const CFBADD = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFBLR: u32 {
        const CFBLL = 0x00001FFF;
        const CFBP = 0x1FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFBLNR: u32 {
        const CFBLNBR = 0x000007FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CLUTWR: u32 {
        const BLUE = 0x000000FF;
        const GREEN = 0x0000FF00;
        const RED = 0x00FF0000;
        const CLUTADD = 0xFF000000;
    }
}

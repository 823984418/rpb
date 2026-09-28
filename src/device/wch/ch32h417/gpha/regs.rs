use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct GPHA {
    pub CTLR: Reg<CTLR>,
    pub ISR: Reg<ISR>,
    pub IFCR: Reg<IFCR>,
    pub FGMAR: Reg<FGMAR>,
    pub FGOR: Reg<FGOR>,
    pub BGMAR: Reg<BGMAR>,
    pub BGOR: Reg<BGOR>,
    pub FGPFCCR: Reg<FGPFCCR>,
    pub FGCOLR: Reg<FGCOLR>,
    pub BGPFCCR: Reg<BGPFCCR>,
    pub BGCOLR: Reg<BGCOLR>,
    pub FGCMAR: Reg<FGCMAR>,
    pub BGCMAR: Reg<BGCMAR>,
    pub OPFCCR: Reg<OPFCCR>,
    pub OCOLR: Reg<OCOLR>,
    pub OMAR: Reg<OMAR>,
    pub OOR: Reg<OOR>,
    pub NLR: Reg<NLR>,
    pub LWR: Reg<LWR>,
    pub AMTCR: Reg<AMTCR>,
    pub FGCWRS: Reg<FGCWRS>,
    pub FGCDAT: Reg<FGCDAT>,
    pub BGCWRS: Reg<BGCWRS>,
    pub BGCDAT: Reg<BGCDAT>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const MODE_2 = 1 << 18;
        const MODE_1 = 1 << 17;
        const MODE_0 = 1 << 16;
        const MODE_MASK = 0x7 << 16;

        const CEIE = 1 << 13;
        const CTCIE = 1 << 12;
        const CAEIE = 1 << 11;
        const TWIE = 1 << 10;
        const TCIE = 1 << 9;

        const ABORT = 1 << 2;
        const SUSP = 1 << 1;
        const START = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const CEIF = 1 << 5;
        const CTCIF = 1 << 4;
        const CAEIF = 1 << 3;
        const TWIF = 1 << 2;
        const TCIF = 1 << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IFCR: u32 {
        const CCEIF = 1 << 5;
        const CCTCIF = 1 << 4;
        const CAECIF = 1 << 3;
        const CTWIF = 1 << 2;
        const CTCIF = 1 << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGMAR: u32 {
        const MA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGOR: u32 {
        const LO_MASK = 0x3FFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGMAR: u32 {
        const MA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGOR: u32 {
        const LO_MASK = 0x3FFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGPFCCR: u32 {
        const ALPHA_MASK = 0xFF << 24;

        const RBS = 1 << 21;
        const AI = 1 << 20;

        const CSS_1 = 1 << 19;
        const CSS_0 = 1 << 18;
        const CSS_MASK = 0x3 << 18;

        const AM_1 = 1 << 17;
        const AM_0 = 1 << 16;
        const AM_MASK = 0x3 << 16;

        const CS_MASK = 0xFF << 8;

        const START = 1 << 5;
        const CCM = 1 << 4;

        const CM_3 = 1 << 3;
        const CM_2 = 1 << 2;
        const CM_1 = 1 << 1;
        const CM_0 = 1 << 0;

        const CM_YcbCr = 0b1011 << 0;
        const CM_A4 = 0b1010 << 0;
        const CM_A8 = 0b1001 << 0;
        const CM_L4 = 0b1000 << 0;
        const CM_AL88 = 0b0111 << 0;
        const CM_AL44 = 0b0110 << 0;
        const CM_L8 = 0b0101 << 0;
        const CM_ARGB4444 = 0b0100 << 0;
        const CM_ARGB1555 = 0b0011 << 0;
        const CM_RGB565 = 0b0010 << 0;
        const CM_RGB888 = 0b0001 << 0;
        const CM_ARGB8888 = 0b0000 << 0;
        const CM_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGCOLR: u32 {
        const RED_MASK = 0xFF << 16;
        const GREEN_MASK = 0xFF << 8;
        const BLUE_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGPFCCR: u32 {
        const ALPHA_MASK = 0xFF << 24;

        const RBS = 1 << 21;
        const AI = 1 << 20;

        const AM_1 = 1 << 17;
        const AM_0 = 1 << 16;
        const AM_MASK = 0x3 << 16;

        const CS_MASK = 0xFF << 8;

        const START = 1 << 5;
        const CCM = 1 << 4;

        const CM_3 = 1 << 3;
        const CM_2 = 1 << 2;
        const CM_1 = 1 << 1;
        const CM_0 = 1 << 0;

        const CM_A4 = 0b1010 << 0;
        const CM_A8 = 0b1001 << 0;
        const CM_L4 = 0b1000 << 0;
        const CM_AL88 = 0b0111 << 0;
        const CM_AL44 = 0b0110 << 0;
        const CM_L8 = 0b0101 << 0;
        const CM_ARGB4444 = 0b0100 << 0;
        const CM_ARGB1555 = 0b0011 << 0;
        const CM_RGB565 = 0b0010 << 0;
        const CM_RGB888 = 0b0001 << 0;
        const CM_ARGB8888 = 0b0000 << 0;
        const CM_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGCOLR: u32 {
        const RED_MASK = 0xFF << 16;
        const GREEN_MASK = 0xFF << 8;
        const BLUE_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGCMAR: u32 {
        const MA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGCMAR: u32 {
        const MA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPFCCR: u32 {
        const RBS = 1 << 21;
        const AI = 1 << 20;

        const CM_2 = 1 << 2;
        const CM_1 = 1 << 1;
        const CM_0 = 1 << 0;

        const CM_ARGB4444 = 0b100 << 0;
        const CM_ARGB1555 = 0b011 << 0;
        const CM_RGB565 = 0b010 << 0;
        const CM_RGB888 = 0b001 << 0;
        const CM_ARGB8888 = 0b000 << 0;
        const CM_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OCOLR: u32 {
        const ALPHA_MASK = 0xFF << 24;
        const RED_MASK = 0xFF << 16;
        const GREEN_MASK = 0xFF << 8;
        const BLUE_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OMAR: u32 {
        const MA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OOR: u32 {
        const LO_MASK = 0x3FFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NLR: u32 {
        const PL_MASK = 0x3FFF << 16;
        const NL_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LWR: u32 {
        const LW_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AMTCR: u32 {
        const DT_MASK = 0xFF << 8;
        const EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGCWRS: u32 {
        const FG_CLUT_EN = 1 << 8;
        const FG_CLUT_INDEX_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGCDAT: u32 {
        const FG_CLUT_DATA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGCWRS: u32 {
        const BG_CLUT_EN = 1 << 8;
        const BG_CLUT_INDEX_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGCDAT: u32 {
        const BG_CLUT_DATA_MASK = 0xFFFFFFFF << 0;
    }
}

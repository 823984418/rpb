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
        const START = 0x00000001;
        const SUSP = 0x00000002;
        const ABORT = 0x00000004;

        const TCIE = 0x00000200;
        const TWIE = 0x00000400;
        const CAEIE = 0x00000800;
        const CTCIE = 0x00001000;
        const CEIE = 0x00002000;

        const MODE = 0x00070000;
        const MODE_0 = 0x00010000;
        const MODE_1 = 0x00020000;
        const MODE_2 = 0x00040000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const TCIF = 0x00000002;
        const TWIF = 0x00000004;
        const CAEIF = 0x00000008;
        const CTCIF = 0x00000010;
        const CEIF = 0x00000020;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IFCR: u32 {
        const CTCIF = 0x00000002;
        const CTWIF = 0x00000004;
        const CAECIF = 0x00000008;
        const CCTCIF = 0x00000010;
        const CCEIF = 0x00000020;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGMAR: u32 {
        const MA = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGOR: u32 {
        const LO = 0x00003FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGMAR: u32 {
        const MA = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGOR: u32 {
        const LO = 0x00003FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGPFCCR: u32 {
        const CM = 0x0000000F;
        const CM_0 = 0x00000001;
        const CM_1 = 0x00000002;
        const CM_2 = 0x00000004;
        const CM_3 = 0x00000008;

        const CM_ARGB8888 = 0x00000000;
        const CM_RGB888 = 0x00000001;
        const CM_RGB565 = 0x00000002;
        const CM_ARGB1555 = 0x00000003;
        const CM_ARGB4444 = 0x00000004;
        const CM_L8 = 0x00000005;
        const CM_AL44 = 0x00000006;
        const CM_AL88 = 0x00000007;
        const CM_L4 = 0x00000008;
        const CM_A8 = 0x00000009;
        const CM_A4 = 0x0000000A;
        const CM_YcbCr = 0x0000000B;

        const CCM = 0x00000010;
        const START = 0x00000020;

        const CS = 0x0000FF00;

        const AM = 0x00030000;
        const AM_0 = 0x00010000;
        const AM_1 = 0x00020000;

        const CSS = 0x000C0000;
        const CSS_0 = 0x00040000;
        const CSS_1 = 0x00080000;

        const AI = 0x00100000;
        const RBS = 0x00200000;

        const ALPHA = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGCOLR: u32 {
        const BLUE = 0x000000FF;
        const GREEN = 0x0000FF00;
        const RED = 0x00FF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGPFCCR: u32 {
        const CM = 0x0000000F;
        const CM_0 = 0x00000001;
        const CM_1 = 0x00000002;
        const CM_2 = 0x00000004;
        const CM_3 = 0x00000008;

        const CM_ARGB8888 = 0x00000000;
        const CM_RGB888 = 0x00000001;
        const CM_RGB565 = 0x00000002;
        const CM_ARGB1555 = 0x00000003;
        const CM_ARGB4444 = 0x00000004;
        const CM_L8 = 0x00000005;
        const CM_AL44 = 0x00000006;
        const CM_AL88 = 0x00000007;
        const CM_L4 = 0x00000008;
        const CM_A8 = 0x00000009;
        const CM_A4 = 0x0000000A;

        const CCM = 0x00000010;
        const START = 0x00000020;

        const CS = 0x0000FF00;

        const AM = 0x00030000;
        const AM_0 = 0x00010000;
        const AM_1 = 0x00020000;

        const AI = 0x00100000;
        const RBS = 0x00200000;

        const ALPHA = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGCOLR: u32 {
        const BLUE = 0x000000FF;
        const GREEN = 0x0000FF00;
        const RED = 0x00FF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGCMAR: u32 {
        const MA = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGCMAR: u32 {
        const MA = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OPFCCR: u32 {
        const CM = 0x00000007;
        const CM_0 = 0x00000001;
        const CM_1 = 0x00000002;
        const CM_2 = 0x00000004;

        const CM_ARGB8888 = 0x00000000;
        const CM_RGB888 = 0x00000001;
        const CM_RGB565 = 0x00000002;
        const CM_ARGB1555 = 0x00000003;
        const CM_ARGB4444 = 0x00000004;

        const AI = 0x00100000;
        const RBS = 0x00200000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OCOLR: u32 {
        const BLUE = 0x000000FF;
        const GREEN = 0x0000FF00;
        const RED = 0x00FF0000;
        const ALPHA = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OMAR: u32 {
        const MA = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OOR: u32 {
        const LO = 0x00003FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NLR: u32 {
        const NL = 0x0000FFFF;
        const PL = 0x3FFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LWR: u32 {
        const LW = 0x0000FFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AMTCR: u32 {
        const EN = 0x00000001;
        const DT = 0x0000FF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGCWRS: u32 {
        const FG_CLUT_INDEX = 0x000000FF;
        const FG_CLUT_EN = 0x00000100;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FGCDAT: u32 {
        const FG_CLUT_DATA = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGCWRS: u32 {
        const BG_CLUT_INDEX = 0x000000FF;
        const BG_CLUT_EN = 0x00000100;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BGCDAT: u32 {
        const BG_CLUT_DATA = 0xFFFFFFFF;
    }
}

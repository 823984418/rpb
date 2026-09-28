use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct DFSDM_FLT {
    pub CR1: Reg<CR1>,
    RESERVED0: Reserved<u32>,
    pub CR2: Reg<CR2>,
    RESERVED1: Reserved<u32>,
    pub ISR: Reg<ISR>,
    RESERVED2: Reserved<u32>,
    pub ICR: Reg<ICR>,
    RESERVED3: Reserved<u32>,
    pub JCHGR: Reg<JCHGR>,
    RESERVED4: Reserved<u32>,
    pub FCR3: Reg<FCR3>,
    RESERVED5: Reserved<u32>,
    pub JDATAR: Reg<JDATAR>,
    RESERVED6: Reserved<u32>,
    pub RDATAR: Reg<RDATAR>,
    RESERVED7: Reserved<u32>,
    pub AWHTR: Reg<AWHTR>,
    RESERVED8: Reserved<u32>,
    pub AWLTR: Reg<AWLTR>,
    RESERVED9: Reserved<u32>,
    pub AWSR: Reg<AWSR>,
    RESERVED10: Reserved<u32>,
    pub AWCFR: Reg<AWCFR>,
    RESERVED11: Reserved<u32>,
    pub EXMAX: Reg<EXMAX>,
    RESERVED12: Reserved<u32>,
    pub EXMIN: Reg<EXMIN>,
    RESERVED13: Reserved<u32>,
    pub NVTIMR: Reg<NVTIMR>,
    RESERVED14: Reserved<u32>,
}

#[derive(Debug)]
#[repr(C)]
pub struct DFSDM_Channel {
    pub CFGR1: Reg<CFGR1>,
    RESERVED0: Reserved<u32>,
    pub CFGR2: Reg<CFGR2>,
    RESERVED1: Reserved<u32>,
    pub AWSCDR: Reg<AWSCDR>,
    RESERVED2: Reserved<u32>,
    pub WDATR: Reg32,
    RESERVED3: Reserved<u32>,
    pub DATINR: Reg<DATINR>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR1: u32 {
        const DFEN = 0x00000001;
        const JSWSTART = 0x00000002;
        const JSYNC = 0x00000008;
        const JSCAN = 0x00000010;
        const JDMAEN = 0x00000020;

        const JEXTSEL = 0x00000F00;
        const JEXTSEL_0 = 0x00000100;
        const JEXTSEL_1 = 0x00000200;
        const JEXTSEL_2 = 0x00000400;
        const JEXTSEL_3 = 0x00000800;

        const JEXTEN = 0x00006000;
        const JEXTEN_0 = 0x00002000;
        const JEXTEN_1 = 0x00004000;

        const RSWSTART = 0x00020000;
        const RCONT = 0x00040000;
        const RSYNC = 0x00080000;

        const RDMAEN = 0x00200000;

        const RCH = 0x01000000;

        const FAST = 0x20000000;
        const AWFSEL = 0x40000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR2: u32 {
        const JEOCIE = 0x00000001;
        const REOCIE = 0x00000002;
        const JOVRIE = 0x00000004;
        const ROVRIE = 0x00000008;
        const AWDIE = 0x00000010;
        const SCDIE = 0x00000020;
        const CKABIE = 0x00000040;

        const EXCH = 0x00000300;
        const EXCH_0 = 0x00000100;
        const EXCH_1 = 0x00000200;

        const AWDCH = 0x00030000;
        const AWDCH_0 = 0x00010000;
        const AWDCH_1 = 0x00020000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const JEOCF = 0x00000001;
        const REOCF = 0x00000002;
        const JOVRF = 0x00000004;
        const ROVRF = 0x00000008;
        const AWDF = 0x00000010;

        const JCIP = 0x00002000;
        const RCIP = 0x00004000;

        const CKABF = 0x00030000;
        const CKABF_0 = 0x00010000;
        const CKABF_1 = 0x00020000;

        const SCDF = 0x03000000;
        const SCDF_0 = 0x01000000;
        const SCDF_1 = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ICR: u32 {
        const CLRJOVRF = 0x00000004;
        const CLRROVRF = 0x00000008;

        const CLRCKABF = 0x00030000;
        const CLRCKABF_0 = 0x00010000;
        const CLRCKABF_1 = 0x00020000;

        const CLRSCDF = 0x03000000;
        const CLRSCDF_0 = 0x01000000;
        const CLRSCDF_1 = 0x02000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct JCHGR: u32 {
        const JCHG = 0x00000003;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FCR3: u32 {
        const IOSR = 0x000000FF;
        const IOSR_0 = 0x00000001;
        const IOSR_1 = 0x00000002;
        const IOSR_2 = 0x00000004;
        const IOSR_3 = 0x00000008;
        const IOSR_4 = 0x00000010;
        const IOSR_5 = 0x00000020;
        const IOSR_6 = 0x00000040;
        const IOSR_7 = 0x00000080;

        const FOSR = 0x03FF0000;
        const FOSR_0 = 0x00010000;
        const FOSR_1 = 0x00020000;
        const FOSR_2 = 0x00040000;
        const FOSR_3 = 0x00080000;
        const FOSR_4 = 0x00100000;
        const FOSR_5 = 0x00200000;
        const FOSR_6 = 0x00400000;
        const FOSR_7 = 0x00800000;
        const FOSR_8 = 0x01000000;
        const FOSR_9 = 0x03FF0000;

        const FORD = 0xE0000000;
        const FORD_0 = 0x20000000;
        const FORD_1 = 0x40000000;
        const FORD_2 = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct JDATAR: u32 {
        const JDATACH = 0x00000001;
        const JDATA = 0xFFFFFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDATAR: u32 {
        const RDATACH = 0x00000001;
        const RPEND = 0x00000010;
        const RDATA = 0xFFFFFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWHTR: u32 {
        const BKAWH = 0x0000000F;
        const BKAWH_0 = 0x00000001;
        const BKAWH_1 = 0x00000002;
        const BKAWH_2 = 0x00000004;
        const BKAWH_3 = 0x00000008;

        const AWHT = 0xFFFFFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWLTR: u32 {
        const BKAWL = 0x0000000F;
        const BKAWL_0 = 0x00000001;
        const BKAWL_1 = 0x00000002;
        const BKAWL_2 = 0x00000004;
        const BKAWL_3 = 0x00000008;

        const AWLT = 0xFFFFFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWSR: u32 {
        const AWLTF = 0x00000003;
        const AWHTF = 0x00000300;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWCFR: u32 {
        const CLRAWLTF = 0x00000003;
        const CLRAWLTF_0 = 0x00000001;
        const CLRAWLTF_1 = 0x00000002;

        const CLRAWHTF = 0x00000300;
        const CLRAWHTF_0 = 0x00000100;
        const CLRAWHTF_1 = 0x00000200;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EXMAX: u32 {
        const EXMAXCH = 0x00000001;
        const EXMAX = 0xFFFFFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EXMIN: u32 {
        const EXMINCH = 0x00000001;
        const EXMIN = 0xFFFFFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NVTIMR: u32 {
        const CNVCNT = 0xFFFFFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR1: u32 {
        const SITP = 0x00000003;
        const SITP_0 = 0x00000001;
        const SITP_1 = 0x00000002;

        const SPICKSEL = 0x0000000C;
        const SPICKSEL_0 = 0x00000004;
        const SPICKSEL_1 = 0x00000008;

        const SCDEN = 0x00000020;
        const CKABEN = 0x00000040;
        const CHEN = 0x00000080;
        const CHINSEL = 0x00000100;

        const DATMPX = 0x00003000;
        const DATMPX_0 = 0x00001000;
        const DATMPX_1 = 0x00002000;

        const DATPACK = 0x0000C000;
        const DATPACK_0 = 0x00004000;
        const DATPACK_1 = 0x00008000;

        const CKOUTDIV = 0x00FF0000;

        const CKOUTSRC = 0x40000000;
        const DFSDMEN = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR2: u32 {
        const DTRBS = 0x000000F8;
        const OFFSET = 0xFFFFFF00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWSCDR: u32 {
        const SCDT = 0x000000FF;

        const BKSCD = 0x00003000;
        const BKSCD_0 = 0x00001000;
        const BKSCD_1 = 0x00002000;

        const AWFOSR = 0x001F0000;
        const AWFOSR_0 = 0x00010000;
        const AWFOSR_1 = 0x00020000;
        const AWFOSR_2 = 0x00040000;
        const AWFOSR_3 = 0x00080000;
        const AWFOSR_4 = 0x00100000;

        const AWFORD = 0x00C00000;
        const AWFORD_0 = 0x00400000;
        const AWFORD_1 = 0x00800000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATINR: u32 {
        const INDAT0 = 0x0000FFFF;
        const INDAT1 = 0xFFFF0000;
    }
}

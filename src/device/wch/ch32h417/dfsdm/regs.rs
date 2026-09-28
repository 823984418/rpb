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
        const AWFSEL = 1 << 30;
        const FAST = 1 << 29;
        const RCH = 1 << 24;
        const RDMAEN = 1 << 21;
        const RSYNC = 1 << 19;
        const RCONT = 1 << 18;
        const RSWSTART = 1 << 17;

        const JEXTEN_1 = 1 << 14;
        const JEXTEN_0 = 1 << 13;
        const JEXTEN_MASK = 0x3 << 13;

        const JEXTSEL_3 = 1 << 11;
        const JEXTSEL_2 = 1 << 10;
        const JEXTSEL_1 = 1 << 9;
        const JEXTSEL_0 = 1 << 8;
        const JEXTSEL_MASK = 0xF << 8;

        const JDMAEN = 1 << 5;
        const JSCAN = 1 << 4;
        const JSYNC = 1 << 3;
        const JSWSTART = 1 << 1;
        const DFEN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CR2: u32 {
        const AWDCH_1 = 1 << 17;
        const AWDCH_0 = 1 << 16;
        const AWDCH_MASK = 0x3 << 16;

        const EXCH_1 = 1 << 9;
        const EXCH_0 = 1 << 8;
        const EXCH_MASK = 0x3 << 8;

        const CKABIE = 1 << 6;
        const SCDIE = 1 << 5;
        const AWDIE = 1 << 4;
        const ROVRIE = 1 << 3;
        const JOVRIE = 1 << 2;
        const REOCIE = 1 << 1;
        const JEOCIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISR: u32 {
        const SCDF_1 = 1 << 25;
        const SCDF_0 = 1 << 24;
        const SCDF_MASK = 0x3 << 24;

        const CKABF_1 = 1 << 17;
        const CKABF_0 = 1 << 16;
        const CKABF_MASK = 0x3 << 16;

        const RCIP = 1 << 14;
        const JCIP = 1 << 13;
        const AWDF = 1 << 4;
        const ROVRF = 1 << 3;
        const JOVRF = 1 << 2;
        const REOCF = 1 << 1;
        const JEOCF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ICR: u32 {
        const CLRSCDF_1 = 1 << 25;
        const CLRSCDF_0 = 1 << 24;
        const CLRSCDF_MASK = 0x3 << 24;

        const CLRCKABF_1 = 1 << 17;
        const CLRCKABF_0 = 1 << 16;
        const CLRCKABF_MASK = 0x3 << 16;

        const CLRROVRF = 1 << 3;
        const CLRJOVRF = 1 << 2;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct JCHGR: u32 {
        const JCHG_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FCR3: u32 {
        const FORD_2 = 1 << 31;
        const FORD_1 = 1 << 30;
        const FORD_0 = 1 << 29;
        const FORD_MASK = 0x7 << 29;

        const FOSR_9 = 0x3FF << 16;
        const FOSR_8 = 1 << 24;
        const FOSR_7 = 1 << 23;
        const FOSR_6 = 1 << 22;
        const FOSR_5 = 1 << 21;
        const FOSR_4 = 1 << 20;
        const FOSR_3 = 1 << 19;
        const FOSR_2 = 1 << 18;
        const FOSR_1 = 1 << 17;
        const FOSR_0 = 1 << 16;
        const FOSR_MASK = 0x3FF << 16;

        const IOSR_7 = 1 << 7;
        const IOSR_6 = 1 << 6;
        const IOSR_5 = 1 << 5;
        const IOSR_4 = 1 << 4;
        const IOSR_3 = 1 << 3;
        const IOSR_2 = 1 << 2;
        const IOSR_1 = 1 << 1;
        const IOSR_0 = 1 << 0;
        const IOSR_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct JDATAR: u32 {
        const JDATA_MASK = 0xFFFFFF << 8;

        const JDATACH = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDATAR: u32 {
        const RDATA_MASK = 0xFFFFFF << 8;

        const RPEND = 1 << 4;
        const RDATACH = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWHTR: u32 {
        const AWHT_MASK = 0xFFFFFF << 8;

        const BKAWH_3 = 1 << 3;
        const BKAWH_2 = 1 << 2;
        const BKAWH_1 = 1 << 1;
        const BKAWH_0 = 1 << 0;
        const BKAWH_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWLTR: u32 {
        const AWLT_MASK = 0xFFFFFF << 8;

        const BKAWL_3 = 1 << 3;
        const BKAWL_2 = 1 << 2;
        const BKAWL_1 = 1 << 1;
        const BKAWL_0 = 1 << 0;
        const BKAWL_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWSR: u32 {
        const AWHTF_MASK = 0x3 << 8;

        const AWLTF_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWCFR: u32 {
        const CLRAWHTF_1 = 1 << 9;
        const CLRAWHTF_0 = 1 << 8;
        const CLRAWHTF_MASK = 0x3 << 8;

        const CLRAWLTF_1 = 1 << 1;
        const CLRAWLTF_0 = 1 << 0;
        const CLRAWLTF_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EXMAX: u32 {
        const EXMAX_MASK = 0xFFFFFF << 8;

        const EXMAXCH = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EXMIN: u32 {
        const EXMIN_MASK = 0xFFFFFF << 8;

        const EXMINCH = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct NVTIMR: u32 {
        const CNVCNT_MASK = 0xFFFFFF << 8;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR1: u32 {
        const DFSDMEN = 1 << 31;
        const CKOUTSRC = 1 << 30;

        const CKOUTDIV_MASK = 0xFF << 16;

        const DATPACK_1 = 1 << 15;
        const DATPACK_0 = 1 << 14;
        const DATPACK_MASK = 0x3 << 14;

        const DATMPX_1 = 1 << 13;
        const DATMPX_0 = 1 << 12;
        const DATMPX_MASK = 0x3 << 12;

        const CHINSEL = 1 << 8;
        const CHEN = 1 << 7;
        const CKABEN = 1 << 6;
        const SCDEN = 1 << 5;

        const SPICKSEL_1 = 1 << 3;
        const SPICKSEL_0 = 1 << 2;
        const SPICKSEL_MASK = 0x3 << 2;

        const SITP_1 = 1 << 1;
        const SITP_0 = 1 << 0;
        const SITP_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR2: u32 {
        const OFFSET_MASK = 0xFFFFFF << 8;

        const DTRBS_MASK = 0x1F << 3;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AWSCDR: u32 {
        const AWFORD_1 = 1 << 23;
        const AWFORD_0 = 1 << 22;
        const AWFORD_MASK = 0x3 << 22;

        const AWFOSR_4 = 1 << 20;
        const AWFOSR_3 = 1 << 19;
        const AWFOSR_2 = 1 << 18;
        const AWFOSR_1 = 1 << 17;
        const AWFOSR_0 = 1 << 16;
        const AWFOSR_MASK = 0x1F << 16;

        const BKSCD_1 = 1 << 13;
        const BKSCD_0 = 1 << 12;
        const BKSCD_MASK = 0x3 << 12;

        const SCDT_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DATINR: u32 {
        const INDAT1_MASK = 0xFFFF << 16;

        const INDAT0_MASK = 0xFFFF << 0;
    }
}

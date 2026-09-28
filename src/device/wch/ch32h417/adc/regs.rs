use bitflags::bitflags;

use crate::register::{Reg, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct ADC {
    pub STATR: Reg<STATR>,
    pub CTLR1: Reg<CTLR1>,
    pub CTLR2: Reg<CTLR2>,
    pub SAMPTR1: Reg<SAMPTR1>,
    pub SAMPTR2: Reg<SAMPTR2>,
    pub IOFR1: Reg<IOFR1>,
    pub IOFR2: Reg<IOFR2>,
    pub IOFR3: Reg<IOFR3>,
    pub IOFR4: Reg<IOFR4>,
    pub WDHTR: Reg<WDHTR>,
    pub WDLTR: Reg<WDLTR>,
    pub RSQR1: Reg<RSQR1>,
    pub RSQR2: Reg<RSQR2>,
    pub RSQR3: Reg<RSQR3>,
    pub ISQR: Reg<ISQR>,
    pub IDATAR1: Reg<IDATAR1>,
    pub IDATAR2: Reg<IDATAR2>,
    pub IDATAR3: Reg<IDATAR3>,
    pub IDATAR4: Reg<IDATAR4>,
    pub RDATAR: Reg<RDATAR>,
    RESERVED0: Reserved<u32>,
    pub AUX: Reg<AUX>,
    pub DRV: Reg<DRV>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const RSTF = 0x8000;
        const STRT = 0x10;
        const JSTRT = 0x08;
        const JEOC = 0x04;
        const EOC = 0x02;
        const AWD = 0x01;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u32 {
        const AWDCH = 0x0000001F;
        const AWDCH_0 = 0x00000001;
        const AWDCH_1 = 0x00000002;
        const AWDCH_2 = 0x00000004;
        const AWDCH_3 = 0x00000008;
        const AWDCH_4 = 0x00000010;

        const EOCIE = 0x00000020;
        const AWDIE = 0x00000040;
        const JEOCIE = 0x00000080;
        const SCAN = 0x00000100;
        const AWDSGL = 0x00000200;
        const JAUTO = 0x00000400;
        const DISCEN = 0x00000800;
        const JDISCEN = 0x00001000;

        const DISCNUM = 0x0000E000;
        const DISCNUM_0 = 0x00002000;
        const DISCNUM_1 = 0x00004000;
        const DISCNUM_2 = 0x00008000;

        const DUALMOD = 0x000F0000;
        const DUALMOD_0 = 0x00010000;
        const DUALMOD_1 = 0x00020000;
        const DUALMOD_2 = 0x00040000;
        const DUALMOD_3 = 0x00080000;

        const JAWDEN = 0x00400000;
        const AWDEN = 0x00800000;

        const TKENABLE = 0x01000000;
        const TKITUNE = 0x02000000;
        const BUFEN = 0x04000000;

        const PGA = 0x18000000;
        const PGA_0 = 0x08000000;
        const PGA_1 = 0x10000000;

        const ANA_RST = 0x40000000;
        const SW_PRE = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u32 {
        const ADON = 0x00000001;
        const CONT = 0x00000002;
        const CAL = 0x00000004;
        const RSTCAL = 0x00000008;
        const DMA = 0x00000100;
        const ALIGN = 0x00000800;

        const JEXTSEL = 0x00007000;
        const JEXTSEL_0 = 0x00001000;
        const JEXTSEL_1 = 0x00002000;
        const JEXTSEL_2 = 0x00004000;

        const JEXTTRIG = 0x00008000;

        const EXTSEL = 0x000E0000;
        const EXTSEL_0 = 0x00020000;
        const EXTSEL_1 = 0x00040000;
        const EXTSEL_2 = 0x00080000;

        const EXTTRIG = 0x00100000;
        const JSWSTART = 0x00200000;
        const SWSTART = 0x00400000;
        const TSVREFE = 0x00800000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SAMPTR1: u32 {
        const SMP10 = 0x00000007;
        const SMP10_0 = 0x00000001;
        const SMP10_1 = 0x00000002;
        const SMP10_2 = 0x00000004;

        const SMP11 = 0x00000038;
        const SMP11_0 = 0x00000008;
        const SMP11_1 = 0x00000010;
        const SMP11_2 = 0x00000020;

        const SMP12 = 0x000001C0;
        const SMP12_0 = 0x00000040;
        const SMP12_1 = 0x00000080;
        const SMP12_2 = 0x00000100;

        const SMP13 = 0x00000E00;
        const SMP13_0 = 0x00000200;
        const SMP13_1 = 0x00000400;
        const SMP13_2 = 0x00000800;

        const SMP14 = 0x00007000;
        const SMP14_0 = 0x00001000;
        const SMP14_1 = 0x00002000;
        const SMP14_2 = 0x00004000;

        const SMP15 = 0x00038000;
        const SMP15_0 = 0x00008000;
        const SMP15_1 = 0x00010000;
        const SMP15_2 = 0x00020000;

        const SMP16 = 0x001C0000;
        const SMP16_0 = 0x00040000;
        const SMP16_1 = 0x00080000;
        const SMP16_2 = 0x00100000;

        const SMP17 = 0x00E00000;
        const SMP17_0 = 0x00200000;
        const SMP17_1 = 0x00400000;
        const SMP17_2 = 0x00800000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SAMPTR2: u32 {
        const SMP0 = 0x00000007;
        const SMP0_0 = 0x00000001;
        const SMP0_1 = 0x00000002;
        const SMP0_2 = 0x00000004;

        const SMP1 = 0x00000038;
        const SMP1_0 = 0x00000008;
        const SMP1_1 = 0x00000010;
        const SMP1_2 = 0x00000020;

        const SMP2 = 0x000001C0;
        const SMP2_0 = 0x00000040;
        const SMP2_1 = 0x00000080;
        const SMP2_2 = 0x00000100;

        const SMP3 = 0x00000E00;
        const SMP3_0 = 0x00000200;
        const SMP3_1 = 0x00000400;
        const SMP3_2 = 0x00000800;

        const SMP4 = 0x00007000;
        const SMP4_0 = 0x00001000;
        const SMP4_1 = 0x00002000;
        const SMP4_2 = 0x00004000;

        const SMP5 = 0x00038000;
        const SMP5_0 = 0x00008000;
        const SMP5_1 = 0x00010000;
        const SMP5_2 = 0x00020000;

        const SMP6 = 0x001C0000;
        const SMP6_0 = 0x00040000;
        const SMP6_1 = 0x00080000;
        const SMP6_2 = 0x00100000;

        const SMP7 = 0x00E00000;
        const SMP7_0 = 0x00200000;
        const SMP7_1 = 0x00400000;
        const SMP7_2 = 0x00800000;

        const SMP8 = 0x07000000;
        const SMP8_0 = 0x01000000;
        const SMP8_1 = 0x02000000;
        const SMP8_2 = 0x04000000;

        const SMP9 = 0x38000000;
        const SMP9_0 = 0x08000000;
        const SMP9_1 = 0x10000000;
        const SMP9_2 = 0x20000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IOFR1: u32 {
        const JOFFSET1 = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IOFR2: u32 {
        const JOFFSET2 = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IOFR3: u32 {
        const JOFFSET3 = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IOFR4: u32 {
        const JOFFSET4 = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WDHTR: u32 {
        const HT = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WDLTR: u32 {
        const LT = 0x0FFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSQR1: u32 {
        const SQ13 = 0x0000001F;
        const SQ13_0 = 0x00000001;
        const SQ13_1 = 0x00000002;
        const SQ13_2 = 0x00000004;
        const SQ13_3 = 0x00000008;
        const SQ13_4 = 0x00000010;

        const SQ14 = 0x000003E0;
        const SQ14_0 = 0x00000020;
        const SQ14_1 = 0x00000040;
        const SQ14_2 = 0x00000080;
        const SQ14_3 = 0x00000100;
        const SQ14_4 = 0x00000200;

        const SQ15 = 0x00007C00;
        const SQ15_0 = 0x00000400;
        const SQ15_1 = 0x00000800;
        const SQ15_2 = 0x00001000;
        const SQ15_3 = 0x00002000;
        const SQ15_4 = 0x00004000;

        const SQ16 = 0x000F8000;
        const SQ16_0 = 0x00008000;
        const SQ16_1 = 0x00010000;
        const SQ16_2 = 0x00020000;
        const SQ16_3 = 0x00040000;
        const SQ16_4 = 0x00080000;

        const L = 0x00F00000;
        const L_0 = 0x00100000;
        const L_1 = 0x00200000;
        const L_2 = 0x00400000;
        const L_3 = 0x00800000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSQR2: u32 {
        const SQ7 = 0x0000001F;
        const SQ7_0 = 0x00000001;
        const SQ7_1 = 0x00000002;
        const SQ7_2 = 0x00000004;
        const SQ7_3 = 0x00000008;
        const SQ7_4 = 0x00000010;

        const SQ8 = 0x000003E0;
        const SQ8_0 = 0x00000020;
        const SQ8_1 = 0x00000040;
        const SQ8_2 = 0x00000080;
        const SQ8_3 = 0x00000100;
        const SQ8_4 = 0x00000200;

        const SQ9 = 0x00007C00;
        const SQ9_0 = 0x00000400;
        const SQ9_1 = 0x00000800;
        const SQ9_2 = 0x00001000;
        const SQ9_3 = 0x00002000;
        const SQ9_4 = 0x00004000;

        const SQ10 = 0x000F8000;
        const SQ10_0 = 0x00008000;
        const SQ10_1 = 0x00010000;
        const SQ10_2 = 0x00020000;
        const SQ10_3 = 0x00040000;
        const SQ10_4 = 0x00080000;

        const SQ11 = 0x01F00000;
        const SQ11_0 = 0x00100000;
        const SQ11_1 = 0x00200000;
        const SQ11_2 = 0x00400000;
        const SQ11_3 = 0x00800000;
        const SQ11_4 = 0x01000000;

        const SQ12 = 0x3E000000;
        const SQ12_0 = 0x02000000;
        const SQ12_1 = 0x04000000;
        const SQ12_2 = 0x08000000;
        const SQ12_3 = 0x10000000;
        const SQ12_4 = 0x20000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSQR3: u32 {
        const SQ1 = 0x0000001F;
        const SQ1_0 = 0x00000001;
        const SQ1_1 = 0x00000002;
        const SQ1_2 = 0x00000004;
        const SQ1_3 = 0x00000008;
        const SQ1_4 = 0x00000010;

        const SQ2 = 0x000003E0;
        const SQ2_0 = 0x00000020;
        const SQ2_1 = 0x00000040;
        const SQ2_2 = 0x00000080;
        const SQ2_3 = 0x00000100;
        const SQ2_4 = 0x00000200;

        const SQ3 = 0x00007C00;
        const SQ3_0 = 0x00000400;
        const SQ3_1 = 0x00000800;
        const SQ3_2 = 0x00001000;
        const SQ3_3 = 0x00002000;
        const SQ3_4 = 0x00004000;

        const SQ4 = 0x000F8000;
        const SQ4_0 = 0x00008000;
        const SQ4_1 = 0x00010000;
        const SQ4_2 = 0x00020000;
        const SQ4_3 = 0x00040000;
        const SQ4_4 = 0x00080000;

        const SQ5 = 0x01F00000;
        const SQ5_0 = 0x00100000;
        const SQ5_1 = 0x00200000;
        const SQ5_2 = 0x00400000;
        const SQ5_3 = 0x00800000;
        const SQ5_4 = 0x01000000;

        const SQ6 = 0x3E000000;
        const SQ6_0 = 0x02000000;
        const SQ6_1 = 0x04000000;
        const SQ6_2 = 0x08000000;
        const SQ6_3 = 0x10000000;
        const SQ6_4 = 0x20000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISQR: u32 {
        const JSQ1 = 0x0000001F;
        const JSQ1_0 = 0x00000001;
        const JSQ1_1 = 0x00000002;
        const JSQ1_2 = 0x00000004;
        const JSQ1_3 = 0x00000008;
        const JSQ1_4 = 0x00000010;

        const JSQ2 = 0x000003E0;
        const JSQ2_0 = 0x00000020;
        const JSQ2_1 = 0x00000040;
        const JSQ2_2 = 0x00000080;
        const JSQ2_3 = 0x00000100;
        const JSQ2_4 = 0x00000200;

        const JSQ3 = 0x00007C00;
        const JSQ3_0 = 0x00000400;
        const JSQ3_1 = 0x00000800;
        const JSQ3_2 = 0x00001000;
        const JSQ3_3 = 0x00002000;
        const JSQ3_4 = 0x00004000;

        const JSQ4 = 0x000F8000;
        const JSQ4_0 = 0x00008000;
        const JSQ4_1 = 0x00010000;
        const JSQ4_2 = 0x00020000;
        const JSQ4_3 = 0x00040000;
        const JSQ4_4 = 0x00080000;

        const JL = 0x00300000;
        const JL_0 = 0x00100000;
        const JL_1 = 0x00200000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR1: u32 {
        const JDATA = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR2: u32 {
        const JDATA = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR3: u32 {
        const JDATA = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR4: u32 {
        const JDATA = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDATAR: u32 {
        const DATA = 0x0000FFFF;
        const ADC2DATA = 0xFFFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AUX: u32 {
        const SMP_SEL_0 = 0x00000001;
        const SMP_SEL_1 = 0x00000002;
        const SMP_SEL_2 = 0x00000004;
        const SMP_SEL_3 = 0x00000008;
        const SMP_SEL_4 = 0x00000010;
        const SMP_SEL_5 = 0x00000020;
        const SMP_SEL_6 = 0x00000040;
        const SMP_SEL_7 = 0x00000080;
        const SMP_SEL_8 = 0x00000100;
        const SMP_SEL_9 = 0x00000200;
        const SMP_SEL_10 = 0x00000400;
        const SMP_SEL_11 = 0x00000800;
        const SMP_SEL_12 = 0x00001000;
        const SMP_SEL_13 = 0x00002000;
        const SMP_SEL_14 = 0x00004000;
        const SMP_SEL_15 = 0x00008000;
        const SMP_SEL_16 = 0x00010000;
        const SMP_SEL_17 = 0x00020000;

        const TO_DFSDM = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DRV: u32 {
        const TKEY_OUTEN = 0x0000FFFF;
        const TKEY_EN = 0x00010000;
    }
}

// TKEY_CHARGE1 bit definitions (header lines 8986-9028). These belong to the
// TKEY_CHARGE1 register of the TKEY peripheral, not to ADC_TypeDef, so no ADC
// struct field maps to them; they are carried here as requested. TKCG14/TKCG15
// have no enumerated value variants in the header.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CHARGE1: u32 {
        const TKCG10 = 0x0007;
        const TKCG10_1C5 = 0x0000;
        const TKCG10_7C5 = 0x0001;
        const TKCG10_13C5 = 0x0002;
        const TKCG10_28C5 = 0x0003;
        const TKCG10_41C5 = 0x0004;
        const TKCG10_55C5 = 0x0005;
        const TKCG10_71C5 = 0x0006;
        const TKCG10_239C5 = 0x0007;

        const TKCG11 = 0x0038;
        const TKCG11_1C5 = 0x0000;
        const TKCG11_7C5 = 0x0008;
        const TKCG11_13C5 = 0x0010;
        const TKCG11_28C5 = 0x0018;
        const TKCG11_41C5 = 0x0020;
        const TKCG11_55C5 = 0x0028;
        const TKCG11_71C5 = 0x0030;
        const TKCG11_239C5 = 0x0038;

        const TKCG12 = 0x01C0;
        const TKCG12_1C5 = 0x0000;
        const TKCG12_7C5 = 0x0040;
        const TKCG12_13C5 = 0x0080;
        const TKCG12_28C5 = 0x00C0;
        const TKCG12_41C5 = 0x0100;
        const TKCG12_55C5 = 0x0140;
        const TKCG12_71C5 = 0x0180;
        const TKCG12_239C5 = 0x01C0;

        const TKCG13 = 0x0E00;
        const TKCG13_1C5 = 0x0000;
        const TKCG13_7C5 = 0x0200;
        const TKCG13_13C5 = 0x0400;
        const TKCG13_28C5 = 0x0600;
        const TKCG13_41C5 = 0x0800;
        const TKCG13_55C5 = 0x0A00;
        const TKCG13_71C5 = 0x0C00;
        const TKCG13_239C5 = 0x0E00;

        const TKCG14 = 0x7000;
        const TKCG15 = 0x38000;
    }
}

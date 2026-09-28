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
        const RSTF = 1 << 15;
        const STRT = 1 << 4;
        const JSTRT = 1 << 3;
        const JEOC = 1 << 2;
        const EOC = 1 << 1;
        const AWD = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR1: u32 {
        const SW_PRE = 1 << 31;
        const ANA_RST = 1 << 30;

        const PGA_1 = 1 << 28;
        const PGA_0 = 1 << 27;
        const PGA_MASK = 0x3 << 27;

        const BUFEN = 1 << 26;
        const TKITUNE = 1 << 25;
        const TKENABLE = 1 << 24;
        const AWDEN = 1 << 23;
        const JAWDEN = 1 << 22;

        const DUALMOD_3 = 1 << 19;
        const DUALMOD_2 = 1 << 18;
        const DUALMOD_1 = 1 << 17;
        const DUALMOD_0 = 1 << 16;
        const DUALMOD_MASK = 0xF << 16;

        const DISCNUM_2 = 1 << 15;
        const DISCNUM_1 = 1 << 14;
        const DISCNUM_0 = 1 << 13;
        const DISCNUM_MASK = 0x7 << 13;

        const JDISCEN = 1 << 12;
        const DISCEN = 1 << 11;
        const JAUTO = 1 << 10;
        const AWDSGL = 1 << 9;
        const SCAN = 1 << 8;
        const JEOCIE = 1 << 7;
        const AWDIE = 1 << 6;
        const EOCIE = 1 << 5;

        const AWDCH_4 = 1 << 4;
        const AWDCH_3 = 1 << 3;
        const AWDCH_2 = 1 << 2;
        const AWDCH_1 = 1 << 1;
        const AWDCH_0 = 1 << 0;
        const AWDCH_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR2: u32 {
        const TSVREFE = 1 << 23;
        const SWSTART = 1 << 22;
        const JSWSTART = 1 << 21;
        const EXTTRIG = 1 << 20;

        const EXTSEL_2 = 1 << 19;
        const EXTSEL_1 = 1 << 18;
        const EXTSEL_0 = 1 << 17;
        const EXTSEL_MASK = 0x7 << 17;

        const JEXTTRIG = 1 << 15;

        const JEXTSEL_2 = 1 << 14;
        const JEXTSEL_1 = 1 << 13;
        const JEXTSEL_0 = 1 << 12;
        const JEXTSEL_MASK = 0x7 << 12;

        const ALIGN = 1 << 11;
        const DMA = 1 << 8;
        const RSTCAL = 1 << 3;
        const CAL = 1 << 2;
        const CONT = 1 << 1;
        const ADON = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SAMPTR1: u32 {
        const SMP17_2 = 1 << 23;
        const SMP17_1 = 1 << 22;
        const SMP17_0 = 1 << 21;
        const SMP17_MASK = 0x7 << 21;

        const SMP16_2 = 1 << 20;
        const SMP16_1 = 1 << 19;
        const SMP16_0 = 1 << 18;
        const SMP16_MASK = 0x7 << 18;

        const SMP15_2 = 1 << 17;
        const SMP15_1 = 1 << 16;
        const SMP15_0 = 1 << 15;
        const SMP15_MASK = 0x7 << 15;

        const SMP14_2 = 1 << 14;
        const SMP14_1 = 1 << 13;
        const SMP14_0 = 1 << 12;
        const SMP14_MASK = 0x7 << 12;

        const SMP13_2 = 1 << 11;
        const SMP13_1 = 1 << 10;
        const SMP13_0 = 1 << 9;
        const SMP13_MASK = 0x7 << 9;

        const SMP12_2 = 1 << 8;
        const SMP12_1 = 1 << 7;
        const SMP12_0 = 1 << 6;
        const SMP12_MASK = 0x7 << 6;

        const SMP11_2 = 1 << 5;
        const SMP11_1 = 1 << 4;
        const SMP11_0 = 1 << 3;
        const SMP11_MASK = 0x7 << 3;

        const SMP10_2 = 1 << 2;
        const SMP10_1 = 1 << 1;
        const SMP10_0 = 1 << 0;
        const SMP10_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SAMPTR2: u32 {
        const SMP9_2 = 1 << 29;
        const SMP9_1 = 1 << 28;
        const SMP9_0 = 1 << 27;
        const SMP9_MASK = 0x7 << 27;

        const SMP8_2 = 1 << 26;
        const SMP8_1 = 1 << 25;
        const SMP8_0 = 1 << 24;
        const SMP8_MASK = 0x7 << 24;

        const SMP7_2 = 1 << 23;
        const SMP7_1 = 1 << 22;
        const SMP7_0 = 1 << 21;
        const SMP7_MASK = 0x7 << 21;

        const SMP6_2 = 1 << 20;
        const SMP6_1 = 1 << 19;
        const SMP6_0 = 1 << 18;
        const SMP6_MASK = 0x7 << 18;

        const SMP5_2 = 1 << 17;
        const SMP5_1 = 1 << 16;
        const SMP5_0 = 1 << 15;
        const SMP5_MASK = 0x7 << 15;

        const SMP4_2 = 1 << 14;
        const SMP4_1 = 1 << 13;
        const SMP4_0 = 1 << 12;
        const SMP4_MASK = 0x7 << 12;

        const SMP3_2 = 1 << 11;
        const SMP3_1 = 1 << 10;
        const SMP3_0 = 1 << 9;
        const SMP3_MASK = 0x7 << 9;

        const SMP2_2 = 1 << 8;
        const SMP2_1 = 1 << 7;
        const SMP2_0 = 1 << 6;
        const SMP2_MASK = 0x7 << 6;

        const SMP1_2 = 1 << 5;
        const SMP1_1 = 1 << 4;
        const SMP1_0 = 1 << 3;
        const SMP1_MASK = 0x7 << 3;

        const SMP0_2 = 1 << 2;
        const SMP0_1 = 1 << 1;
        const SMP0_0 = 1 << 0;
        const SMP0_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IOFR1: u32 {
        const JOFFSET1_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IOFR2: u32 {
        const JOFFSET2_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IOFR3: u32 {
        const JOFFSET3_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IOFR4: u32 {
        const JOFFSET4_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WDHTR: u32 {
        const HT_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct WDLTR: u32 {
        const LT_MASK = 0xFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSQR1: u32 {
        const L_3 = 1 << 23;
        const L_2 = 1 << 22;
        const L_1 = 1 << 21;
        const L_0 = 1 << 20;
        const L_MASK = 0xF << 20;

        const SQ16_4 = 1 << 19;
        const SQ16_3 = 1 << 18;
        const SQ16_2 = 1 << 17;
        const SQ16_1 = 1 << 16;
        const SQ16_0 = 1 << 15;
        const SQ16_MASK = 0x1F << 15;

        const SQ15_4 = 1 << 14;
        const SQ15_3 = 1 << 13;
        const SQ15_2 = 1 << 12;
        const SQ15_1 = 1 << 11;
        const SQ15_0 = 1 << 10;
        const SQ15_MASK = 0x1F << 10;

        const SQ14_4 = 1 << 9;
        const SQ14_3 = 1 << 8;
        const SQ14_2 = 1 << 7;
        const SQ14_1 = 1 << 6;
        const SQ14_0 = 1 << 5;
        const SQ14_MASK = 0x1F << 5;

        const SQ13_4 = 1 << 4;
        const SQ13_3 = 1 << 3;
        const SQ13_2 = 1 << 2;
        const SQ13_1 = 1 << 1;
        const SQ13_0 = 1 << 0;
        const SQ13_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSQR2: u32 {
        const SQ12_4 = 1 << 29;
        const SQ12_3 = 1 << 28;
        const SQ12_2 = 1 << 27;
        const SQ12_1 = 1 << 26;
        const SQ12_0 = 1 << 25;
        const SQ12_MASK = 0x1F << 25;

        const SQ11_4 = 1 << 24;
        const SQ11_3 = 1 << 23;
        const SQ11_2 = 1 << 22;
        const SQ11_1 = 1 << 21;
        const SQ11_0 = 1 << 20;
        const SQ11_MASK = 0x1F << 20;

        const SQ10_4 = 1 << 19;
        const SQ10_3 = 1 << 18;
        const SQ10_2 = 1 << 17;
        const SQ10_1 = 1 << 16;
        const SQ10_0 = 1 << 15;
        const SQ10_MASK = 0x1F << 15;

        const SQ9_4 = 1 << 14;
        const SQ9_3 = 1 << 13;
        const SQ9_2 = 1 << 12;
        const SQ9_1 = 1 << 11;
        const SQ9_0 = 1 << 10;
        const SQ9_MASK = 0x1F << 10;

        const SQ8_4 = 1 << 9;
        const SQ8_3 = 1 << 8;
        const SQ8_2 = 1 << 7;
        const SQ8_1 = 1 << 6;
        const SQ8_0 = 1 << 5;
        const SQ8_MASK = 0x1F << 5;

        const SQ7_4 = 1 << 4;
        const SQ7_3 = 1 << 3;
        const SQ7_2 = 1 << 2;
        const SQ7_1 = 1 << 1;
        const SQ7_0 = 1 << 0;
        const SQ7_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RSQR3: u32 {
        const SQ6_4 = 1 << 29;
        const SQ6_3 = 1 << 28;
        const SQ6_2 = 1 << 27;
        const SQ6_1 = 1 << 26;
        const SQ6_0 = 1 << 25;
        const SQ6_MASK = 0x1F << 25;

        const SQ5_4 = 1 << 24;
        const SQ5_3 = 1 << 23;
        const SQ5_2 = 1 << 22;
        const SQ5_1 = 1 << 21;
        const SQ5_0 = 1 << 20;
        const SQ5_MASK = 0x1F << 20;

        const SQ4_4 = 1 << 19;
        const SQ4_3 = 1 << 18;
        const SQ4_2 = 1 << 17;
        const SQ4_1 = 1 << 16;
        const SQ4_0 = 1 << 15;
        const SQ4_MASK = 0x1F << 15;

        const SQ3_4 = 1 << 14;
        const SQ3_3 = 1 << 13;
        const SQ3_2 = 1 << 12;
        const SQ3_1 = 1 << 11;
        const SQ3_0 = 1 << 10;
        const SQ3_MASK = 0x1F << 10;

        const SQ2_4 = 1 << 9;
        const SQ2_3 = 1 << 8;
        const SQ2_2 = 1 << 7;
        const SQ2_1 = 1 << 6;
        const SQ2_0 = 1 << 5;
        const SQ2_MASK = 0x1F << 5;

        const SQ1_4 = 1 << 4;
        const SQ1_3 = 1 << 3;
        const SQ1_2 = 1 << 2;
        const SQ1_1 = 1 << 1;
        const SQ1_0 = 1 << 0;
        const SQ1_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ISQR: u32 {
        const JL_1 = 1 << 21;
        const JL_0 = 1 << 20;
        const JL_MASK = 0x3 << 20;

        const JSQ4_4 = 1 << 19;
        const JSQ4_3 = 1 << 18;
        const JSQ4_2 = 1 << 17;
        const JSQ4_1 = 1 << 16;
        const JSQ4_0 = 1 << 15;
        const JSQ4_MASK = 0x1F << 15;

        const JSQ3_4 = 1 << 14;
        const JSQ3_3 = 1 << 13;
        const JSQ3_2 = 1 << 12;
        const JSQ3_1 = 1 << 11;
        const JSQ3_0 = 1 << 10;
        const JSQ3_MASK = 0x1F << 10;

        const JSQ2_4 = 1 << 9;
        const JSQ2_3 = 1 << 8;
        const JSQ2_2 = 1 << 7;
        const JSQ2_1 = 1 << 6;
        const JSQ2_0 = 1 << 5;
        const JSQ2_MASK = 0x1F << 5;

        const JSQ1_4 = 1 << 4;
        const JSQ1_3 = 1 << 3;
        const JSQ1_2 = 1 << 2;
        const JSQ1_1 = 1 << 1;
        const JSQ1_0 = 1 << 0;
        const JSQ1_MASK = 0x1F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR1: u32 {
        const JDATA_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR2: u32 {
        const JDATA_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR3: u32 {
        const JDATA_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct IDATAR4: u32 {
        const JDATA_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RDATAR: u32 {
        const ADC2DATA_MASK = 0xFFFF << 16;

        const DATA_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AUX: u32 {
        const TO_DFSDM = 1 << 31;

        const SMP_SEL_17 = 1 << 17;
        const SMP_SEL_16 = 1 << 16;
        const SMP_SEL_15 = 1 << 15;
        const SMP_SEL_14 = 1 << 14;
        const SMP_SEL_13 = 1 << 13;
        const SMP_SEL_12 = 1 << 12;
        const SMP_SEL_11 = 1 << 11;
        const SMP_SEL_10 = 1 << 10;
        const SMP_SEL_9 = 1 << 9;
        const SMP_SEL_8 = 1 << 8;
        const SMP_SEL_7 = 1 << 7;
        const SMP_SEL_6 = 1 << 6;
        const SMP_SEL_5 = 1 << 5;
        const SMP_SEL_4 = 1 << 4;
        const SMP_SEL_3 = 1 << 3;
        const SMP_SEL_2 = 1 << 2;
        const SMP_SEL_1 = 1 << 1;
        const SMP_SEL_0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct DRV: u32 {
        const TKEY_EN = 1 << 16;

        const TKEY_OUTEN_MASK = 0xFFFF << 0;
    }
}

// TKEY_CHARGE1 bit definitions (header lines 8986-9028). These belong to the
// TKEY_CHARGE1 register of the TKEY peripheral, not to ADC_TypeDef, so no ADC
// struct field maps to them; they are carried here as requested. TKCG14/TKCG15
// have no enumerated value variants in the header.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CHARGE1: u32 {
        const TKCG15_MASK = 0x7 << 15;

        const TKCG14_MASK = 0x7 << 12;

        const TKCG13_239C5 = 0b111 << 9;
        const TKCG13_71C5 = 0b110 << 9;
        const TKCG13_55C5 = 0b101 << 9;
        const TKCG13_41C5 = 0b100 << 9;
        const TKCG13_28C5 = 0b011 << 9;
        const TKCG13_13C5 = 0b010 << 9;
        const TKCG13_7C5 = 0b001 << 9;
        const TKCG13_1C5 = 0b000 << 9;
        const TKCG13_MASK = 0x7 << 9;

        const TKCG12_239C5 = 0b111 << 6;
        const TKCG12_71C5 = 0b110 << 6;
        const TKCG12_55C5 = 0b101 << 6;
        const TKCG12_41C5 = 0b100 << 6;
        const TKCG12_28C5 = 0b011 << 6;
        const TKCG12_13C5 = 0b010 << 6;
        const TKCG12_7C5 = 0b001 << 6;
        const TKCG12_1C5 = 0b000 << 6;
        const TKCG12_MASK = 0x7 << 6;

        const TKCG11_239C5 = 0b111 << 3;
        const TKCG11_71C5 = 0b110 << 3;
        const TKCG11_55C5 = 0b101 << 3;
        const TKCG11_41C5 = 0b100 << 3;
        const TKCG11_28C5 = 0b011 << 3;
        const TKCG11_13C5 = 0b010 << 3;
        const TKCG11_7C5 = 0b001 << 3;
        const TKCG11_1C5 = 0b000 << 3;
        const TKCG11_MASK = 0x7 << 3;

        const TKCG10_239C5 = 0b111 << 0;
        const TKCG10_71C5 = 0b110 << 0;
        const TKCG10_55C5 = 0b101 << 0;
        const TKCG10_41C5 = 0b100 << 0;
        const TKCG10_28C5 = 0b011 << 0;
        const TKCG10_13C5 = 0b010 << 0;
        const TKCG10_7C5 = 0b001 << 0;
        const TKCG10_1C5 = 0b000 << 0;
        const TKCG10_MASK = 0x7 << 0;
    }
}

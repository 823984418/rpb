use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADCON1: u32 {
        const ADIE2 = 1 << 31;

        const ADSTRS2_MASK = 0x7F << 24;

        const ADIE1 = 1 << 23;

        const ADSTRS1_MASK = 0x7F << 16;

        const ADIE = 1 << 15;

        const ADSTRS_MASK = 0x7F << 8;

        const ADON = 1 << 7;

        const ADCTU_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADCON2: u32 {
        const ADOUT = 1 << 28;

        const ADMAXCH2_MASK = 0xF << 24;

        const ADMAXCH1_MASK = 0xF << 20;

        const TGAP_MASK = 0x7 << 17;

        const TS_MASK = 0xF << 8;

        const ADMAXCH_MASK = 0xF << 4;

        const TADC_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADCON3: u32 {
        const ADSOC = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADPCH: u32 {
        const ADPCH_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SEQCHSEL: u32 {}
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADGAPON: u32 {}
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ADINTF: u32 {
        const ADIF2C = 1 << 20;
        const ADIF1C = 1 << 19;
        const ADIFC = 1 << 18;

        const ADIF2 = 1 << 4;
        const ADIF1 = 1 << 3;
        const ADIF = 1 << 2;
    }
}

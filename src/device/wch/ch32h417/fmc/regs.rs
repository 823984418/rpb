use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct FMC_BANK1 {
    // BTCR[8] interleaves the bank control and timing registers:
    // BTCR[2n] is BCRn+1 and BTCR[2n+1] is BTRn+1 (n = 0..=3), all 32-bit.
    // Kept as Reg32 because a single array element type cannot alternate
    // between the standalone `BCR` and `BTR` flag types.
    pub BTCR: [Reg32; 8],
}

#[derive(Debug)]
#[repr(C)]
pub struct FMC_BANK1E {
    pub BWTR: [Reg<BWTR>; 7],
}

#[derive(Debug)]
#[repr(C)]
pub struct FMC_BANK3 {
    pub PCR: Reg<PCR>,
    pub SR: Reg<SR>,
    pub PMEM: Reg<PMEM>,
    pub PATT: Reg<PATT>,
    RESERVED0: Reserved<u32>,
    pub ECCR: Reg32,
}

#[derive(Debug)]
#[repr(C)]
pub struct FMC_BANK5_6 {
    pub SDCR: [Reg<SDCR>; 2],
    pub SDTR: [Reg<SDTR>; 2],
    pub SDCMR: Reg<SDCMR>,
    pub SDRTR: Reg<SDRTR>,
    pub SDSR: Reg<SDSR>,
    RESERVED0: Reserved<[u32; 9]>,
    pub MISC: Reg<MISC>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PCR: u32 {
        const ECCPS_MASK = 0x7 << 17;

        const TAR_MASK = 0xF << 13;

        const TCLR_MASK = 0xF << 9;

        const ECCEN = 1 << 10;

        const PWID_MASK = 0x3 << 8;

        const PTYP = 1 << 7;
        const PBKEN = 1 << 6;
        const PWAITEN = 1 << 5;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const FEMPT = 1 << 6;
        const IFEN = 1 << 5;
        const ILEN = 1 << 4;
        const IREN = 1 << 3;
        const IFS = 1 << 2;
        const ILS = 1 << 1;
        const IRS = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PMEM: u32 {
        const MEMHIZ_MASK = 0xFF << 24;
        const MEMHOLD_MASK = 0xFF << 16;
        const MEMWAIT_MASK = 0xFF << 8;
        const MEMSET_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PATT: u32 {
        const ATTHIZ_MASK = 0xFF << 24;
        const ATTHOLD_MASK = 0xFF << 16;
        const ATTWAIT_MASK = 0xFF << 8;
        const ATTSET_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDCMR: u32 {
        const MRD_MASK = 0x1FFF << 9;

        const NRFS_MASK = 0xF << 5;

        const CTB1 = 1 << 4;
        const CTB2 = 1 << 3;

        const MODE_MASK = 0x7 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDRTR: u32 {
        const REIE = 1 << 14;
        const COUNT_MASK = 0x1FFF << 1;
        const CRE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDSR: u32 {
        const BUSY = 1 << 5;

        const MODES2_MASK = 0x3 << 3;

        const MODES1_MASK = 0x3 << 1;

        const RE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MISC: u32 {
        const En_Bank2 = 1 << 17;
        const En_Bank1 = 1 << 16;
        const Enhance_read_mode = 1 << 12;

        const Phase_Sel_MASK = 0xF << 4;

        const NRFS_CNT_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ECCR: u32 {
        const ECC_MASK = 0xFFFFFFFF << 0;
    }
}

// BCR1..BCR4 share one layout. BCR1 names bit 6 MFACCEN
// while BCR2..BCR4 name it FACCEN, and BCR1 names bits 18:16 CPSIZE
// while BCR2..BCR4 name them CPSZIE; both spellings are provided.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BCR: u32 {
        const FMCEN = 1 << 31;

        const BMP_MASK = 0x3 << 24;

        const CBURSTRW = 1 << 19;

        const CPSIZE_MASK = 0x7 << 16;

        const CPSZIE_MASK = 0x7 << 16;

        const ASYNCWAIT = 1 << 15;
        const EXTMOD = 1 << 14;
        const WAITEN = 1 << 13;
        const WREN = 1 << 12;

        const WAITCFG = 1 << 11;
        const WAITPOL = 1 << 9;
        const BURSTEN = 1 << 8;

        const MFACCEN = 1 << 6;
        const FACCEN = 1 << 6;

        const MWID_MASK = 0x3 << 4;

        const MTYP_MASK = 0x3 << 2;

        const MUXEN = 1 << 1;
        const MBKEN = 1 << 0;
    }
}

// BTR1..BTR4 have identical layouts.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BTR: u32 {
        const ACCMOD_MASK = 0x3 << 28;

        const DATLAT_MASK = 0xF << 24;

        const CLKDIV_MASK = 0xF << 20;

        const BUSTURN_MASK = 0xF << 16;

        const DATAST_MASK = 0xFF << 8;

        const ADDHLD_MASK = 0xF << 4;

        const ADDSET_MASK = 0xF << 0;
    }
}

// BWTR1..BWTR4 have identical layouts; they carry no CLKDIV/DATLAT.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BWTR: u32 {
        const ACCMOD_MASK = 0x3 << 28;

        const BUSTURN_MASK = 0xF << 16;

        const DATAST_MASK = 0xFF << 8;

        const ADDHLD_MASK = 0xF << 4;

        const ADDSET_MASK = 0xF << 0;
    }
}

// SDCR1 and SDCR2 have identical layouts.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDCR: u32 {
        const RPIPE_MASK = 0x3 << 13;

        const RBURST = 1 << 12;

        const SDCLK_MASK = 0x3 << 10;

        const WP = 1 << 9;

        const CAS_MASK = 0x3 << 7;

        const NB = 1 << 6;

        const MWID_MASK = 0x3 << 4;

        const NR_MASK = 0x3 << 2;

        const NC_MASK = 0x3 << 0;
    }
}

// SDTR1 and SDTR2 have identical layouts. The TXSR and TRP bit suffixes
// are exactly as spelled in the header.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDTR: u32 {
        const TRCD_MASK = 0xF << 24;

        const TRP_MASK = 0xF << 20;

        const TWR_MASK = 0xF << 16;

        const TRC_MASK = 0xF << 12;

        const TRAS_MASK = 0xF << 8;

        const TXSR_MASK = 0xF << 4;

        const TMRD_MASK = 0xF << 0;
    }
}

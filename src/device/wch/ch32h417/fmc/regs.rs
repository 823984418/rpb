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
        const PWAITEN = 0x00000020;
        const PBKEN = 0x00000040;
        const PTYP = 0x00000080;
        const PWID = 0x00000300;
        const PWID_0 = 0x00000100;
        const PWID_1 = 0x00000200;

        const ECCEN = 0x00000400;

        const TCLR = 0x00001E00;
        const TCLR_0 = 0x00000200;
        const TCLR_1 = 0x00000400;
        const TCLR_2 = 0x00000800;
        const TCLR_3 = 0x00001000;

        const TAR = 0x0001E000;
        const TAR_0 = 0x00002000;
        const TAR_1 = 0x00004000;
        const TAR_2 = 0x00008000;
        const TAR_3 = 0x00010000;

        const ECCPS = 0x000E0000;
        const ECCPS_0 = 0x00020000;
        const ECCPS_1 = 0x00040000;
        const ECCPS_2 = 0x00080000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const IRS = 0x00000001;
        const ILS = 0x00000002;
        const IFS = 0x00000004;
        const IREN = 0x00000008;
        const ILEN = 0x00000010;
        const IFEN = 0x00000020;
        const FEMPT = 0x00000040;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PMEM: u32 {
        const MEMSET = 0x000000FF;
        const MEMWAIT = 0x0000FF00;
        const MEMHOLD = 0x00FF0000;
        const MEMHIZ = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PATT: u32 {
        const ATTSET = 0x000000FF;
        const ATTWAIT = 0x0000FF00;
        const ATTHOLD = 0x00FF0000;
        const ATTHIZ = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDCMR: u32 {
        const MODE = 0x00000007;
        const MODE_0 = 0x00000001;
        const MODE_1 = 0x00000002;
        const MODE_2 = 0x00000004;

        const CTB2 = 0x00000008;
        const CTB1 = 0x00000010;

        const NRFS = 0x000001E0;
        const NRFS_0 = 0x00000020;
        const NRFS_1 = 0x00000040;
        const NRFS_2 = 0x00000080;
        const NRFS_3 = 0x00000100;

        const MRD = 0x003FFE00;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDRTR: u32 {
        const CRE = 0x00000001;
        const COUNT = 0x00003FFE;
        const REIE = 0x00004000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDSR: u32 {
        const RE = 0x00000001;

        const MODES1 = 0x00000006;
        const MODES1_0 = 0x00000002;
        const MODES1_1 = 0x00000004;

        const MODES2 = 0x00000018;
        const MODES2_0 = 0x00000008;
        const MODES2_1 = 0x00000010;

        const BUSY = 0x00000020;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MISC: u32 {
        const NRFS_CNT = 0x0000000F;
        const NRFS_CNT_0 = 0x00000001;
        const NRFS_CNT_1 = 0x00000002;
        const NRFS_CNT_2 = 0x00000004;
        const NRFS_CNT_3 = 0x00000008;

        const Phase_Sel = 0x000000F0;
        const Phase_Sel_0 = 0x00000010;
        const Phase_Sel_1 = 0x00000020;
        const Phase_Sel_2 = 0x00000040;
        const Phase_Sel_3 = 0x00000080;

        const Enhance_read_mode = 0x00001000;
        const En_Bank1 = 0x00010000;
        const En_Bank2 = 0x00020000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ECCR: u32 {
        const ECC = 0xFFFFFFFF;
    }
}

// BCR1..BCR4 share one layout. BCR1 names bit 6 MFACCEN
// while BCR2..BCR4 name it FACCEN, and BCR1 names bits 18:16 CPSIZE
// while BCR2..BCR4 name them CPSZIE; both spellings are provided.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BCR: u32 {
        const MBKEN = 0x00000001;
        const MUXEN = 0x00000002;

        const MTYP = 0x0000000C;
        const MTYP_0 = 0x00000004;
        const MTYP_1 = 0x00000008;

        const MWID = 0x00000030;
        const MWID_0 = 0x00000010;
        const MWID_1 = 0x00000020;

        const MFACCEN = 0x00000040;
        const FACCEN = 0x00000040;

        const BURSTEN = 0x00000100;
        const WAITPOL = 0x00000200;
        const WAITCFG = 0x00000800;

        const WREN = 0x00001000;
        const WAITEN = 0x00002000;
        const EXTMOD = 0x00004000;
        const ASYNCWAIT = 0x00008000;

        const CPSIZE = 0x00070000;
        const CPSIZE_0 = 0x00010000;
        const CPSIZE_1 = 0x00020000;
        const CPSIZE_2 = 0x00040000;

        const CPSZIE = 0x00070000;
        const CPSZIE_0 = 0x00010000;
        const CPSZIE_1 = 0x00020000;
        const CPSZIE_2 = 0x00040000;

        const CBURSTRW = 0x00080000;

        const BMP = 0x03000000;
        const BMP_0 = 0x01000000;
        const BMP_1 = 0x02000000;

        const FMCEN = 0x80000000;
    }
}

// BTR1..BTR4 have identical layouts.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BTR: u32 {
        const ADDSET = 0x0000000F;
        const ADDSET_0 = 0x00000001;
        const ADDSET_1 = 0x00000002;
        const ADDSET_2 = 0x00000004;
        const ADDSET_3 = 0x00000008;

        const ADDHLD = 0x000000F0;
        const ADDHLD_0 = 0x00000010;
        const ADDHLD_1 = 0x00000020;
        const ADDHLD_2 = 0x00000040;
        const ADDHLD_3 = 0x00000080;

        const DATAST = 0x0000FF00;
        const DATAST_0 = 0x00000100;
        const DATAST_1 = 0x00000200;
        const DATAST_2 = 0x00000400;
        const DATAST_3 = 0x00000800;
        const DATAST_4 = 0x00001000;
        const DATAST_5 = 0x00002000;
        const DATAST_6 = 0x00004000;
        const DATAST_7 = 0x00008000;

        const BUSTURN = 0x000F0000;
        const BUSTURN_0 = 0x00010000;
        const BUSTURN_1 = 0x00020000;
        const BUSTURN_2 = 0x00040000;
        const BUSTURN_3 = 0x00080000;

        const CLKDIV = 0x00F00000;
        const CLKDIV_0 = 0x00100000;
        const CLKDIV_1 = 0x00200000;
        const CLKDIV_2 = 0x00400000;
        const CLKDIV_3 = 0x00800000;

        const DATLAT = 0x0F000000;
        const DATLAT_0 = 0x01000000;
        const DATLAT_1 = 0x02000000;
        const DATLAT_2 = 0x04000000;
        const DATLAT_3 = 0x08000000;

        const ACCMOD = 0x30000000;
        const ACCMOD_0 = 0x10000000;
        const ACCMOD_1 = 0x20000000;
    }
}

// BWTR1..BWTR4 have identical layouts; they carry no CLKDIV/DATLAT.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BWTR: u32 {
        const ADDSET = 0x0000000F;
        const ADDSET_0 = 0x00000001;
        const ADDSET_1 = 0x00000002;
        const ADDSET_2 = 0x00000004;
        const ADDSET_3 = 0x00000008;

        const ADDHLD = 0x000000F0;
        const ADDHLD_0 = 0x00000010;
        const ADDHLD_1 = 0x00000020;
        const ADDHLD_2 = 0x00000040;
        const ADDHLD_3 = 0x00000080;

        const DATAST = 0x0000FF00;
        const DATAST_0 = 0x00000100;
        const DATAST_1 = 0x00000200;
        const DATAST_2 = 0x00000400;
        const DATAST_3 = 0x00000800;
        const DATAST_4 = 0x00001000;
        const DATAST_5 = 0x00002000;
        const DATAST_6 = 0x00004000;
        const DATAST_7 = 0x00008000;

        const BUSTURN = 0x000F0000;
        const BUSTURN_0 = 0x00010000;
        const BUSTURN_1 = 0x00020000;
        const BUSTURN_2 = 0x00040000;
        const BUSTURN_3 = 0x00080000;

        const ACCMOD = 0x30000000;
        const ACCMOD_0 = 0x10000000;
        const ACCMOD_1 = 0x20000000;
    }
}

// SDCR1 and SDCR2 have identical layouts.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDCR: u32 {
        const NC = 0x00000003;
        const NC_0 = 0x00000001;
        const NC_1 = 0x00000002;

        const NR = 0x0000000C;
        const NR_0 = 0x00000004;
        const NR_1 = 0x00000008;

        const MWID = 0x00000030;
        const MWID_0 = 0x00000010;
        const MWID_1 = 0x00000020;

        const NB = 0x00000040;

        const CAS = 0x00000180;
        const CAS_0 = 0x00000080;
        const CAS_1 = 0x00000100;

        const WP = 0x00000200;

        const SDCLK = 0x00000C00;
        const SDCLK_0 = 0x00000400;
        const SDCLK_1 = 0x00000800;

        const RBURST = 0x00001000;

        const RPIPE = 0x00006000;
        const RPIPE_0 = 0x00002000;
        const RPIPE_1 = 0x00004000;
    }
}

// SDTR1 and SDTR2 have identical layouts. The TXSR and TRP bit suffixes
// are exactly as spelled in the header.
bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SDTR: u32 {
        const TMRD = 0x0000000F;
        const TMRD_0 = 0x00000001;
        const TMRD_1 = 0x00000002;
        const TMRD_2 = 0x00000004;
        const TMRD_3 = 0x00000008;

        const TXSR = 0x000000F0;
        const TXSR_0 = 0x00000010;
        const TXSR_2 = 0x00000020;
        const TXSR_4 = 0x00000040;
        const TXSR_8 = 0x00000080;

        const TRAS = 0x00000F00;
        const TRAS_0 = 0x00000100;
        const TRAS_1 = 0x00000200;
        const TRAS_2 = 0x00000400;
        const TRAS_3 = 0x00000800;

        const TRC = 0x0000F000;
        const TRC_0 = 0x00001000;
        const TRC_1 = 0x00002000;
        const TRC_2 = 0x00004000;
        const TRC_3 = 0x00008000;

        const TWR = 0x000F0000;
        const TWR_0 = 0x00010000;
        const TWR_1 = 0x00020000;
        const TWR_2 = 0x00040000;
        const TWR_3 = 0x00080000;

        const TRP = 0x00F00000;
        const TRP_0 = 0x00100000;
        const TRP_2 = 0x00200000;
        const TRP_3 = 0x00400000;
        const TRP_4 = 0x00800000;

        const TRCD = 0x0F000000;
        const TRCD_0 = 0x01000000;
        const TRCD_1 = 0x02000000;
        const TRCD_2 = 0x04000000;
        const TRCD_3 = 0x08000000;
    }
}

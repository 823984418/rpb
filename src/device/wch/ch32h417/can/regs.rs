use bitflags::bitflags;

use crate::register::{Reg, Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct CAN_TxMailBox {
    pub TXMIR: Reg<TXMIR>,
    pub TXMDTR: Reg<TXMDTR>,
    pub TXMDLR: Reg<TXMDLR>,
    pub TXMDHR: Reg<TXMDHR>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CAN_FIFOMailBox {
    pub RXMIR: Reg<RXMIR>,
    pub RXMDTR: Reg<RXMDTR>,
    pub RXMDLR: Reg<RXMDLR>,
    pub RXMDHR: Reg<RXMDHR>,
}

#[derive(Debug)]
#[repr(C)]
pub struct CAN_FilterRegister {
    pub FR1: Reg32,
    pub FR2: Reg32,
}

#[derive(Debug)]
#[repr(C)]
pub struct CAN {
    pub CTLR: Reg<CTLR>,
    pub STATR: Reg<STATR>,
    pub TSTATR: Reg<TSTATR>,
    pub RFIFO0: Reg<RFIFO0>,
    pub RFIFO1: Reg<RFIFO1>,
    pub INTENR: Reg<INTENR>,
    pub ERRSR: Reg<ERRSR>,
    pub BTIMR: Reg<BTIMR>,
    pub TTCTLR: Reg<TTCTLR>,
    pub TTCNT: Reg<TTCNT>,
    pub TERR_CNT: Reg<TERR_CNT>,
    RESERVED0: Reserved<[u32; 85]>,
    pub sTxMailBox: [CAN_TxMailBox; 3],
    pub sFIFOMailBox: [CAN_FIFOMailBox; 2],
    RESERVED1: Reserved<[u32; 12]>,
    pub FCTLR: Reg<FCTLR>,
    pub FMCFGR: Reg<FMCFGR>,
    pub FMCFGR_CAN3: Reg32,
    pub FSCFGR: Reg<FSCFGR>,
    pub FSCFGR_CAN3: Reg32,
    pub FAFIFOR: Reg<FAFIFOR>,
    pub FAFIFOR_CAN3: Reg32,
    pub FWR: Reg<FWR>,
    pub FWR_CAN3: Reg32,
    RESERVED2: Reserved<[u32; 7]>,
    pub sFilterRegister: [CAN_FilterRegister; 42],
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CTLR: u32 {
        const CFGCANM = 1 << 17;
        const DBF = 1 << 16;
        const RESET = 1 << 15;
        const TTCM = 1 << 7;
        const ABOM = 1 << 6;
        const AWUM = 1 << 5;
        const NART = 1 << 4;
        const RFLM = 1 << 3;
        const TXFP = 1 << 2;
        const SLEEP = 1 << 1;
        const INRQ = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const RX = 1 << 11;
        const SAMP = 1 << 10;
        const RXM = 1 << 9;
        const TXM = 1 << 8;
        const SLAKI = 1 << 4;
        const WKUI = 1 << 3;
        const ERRI = 1 << 2;
        const SLAK = 1 << 1;
        const INAK = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TSTATR: u32 {
        const LOW2 = 1 << 31;
        const LOW1 = 1 << 30;
        const LOW0 = 1 << 29;
        const LOW_MASK = 0x7 << 29;

        const TME2 = 1 << 28;
        const TME1 = 1 << 27;
        const TME0 = 1 << 26;
        const TME_MASK = 0x7 << 26;

        const CODE_MASK = 0x3 << 24;

        const ABRQ2 = 1 << 23;
        const TERR2 = 1 << 19;
        const ALST2 = 1 << 18;
        const TXOK2 = 1 << 17;
        const RQCP2 = 1 << 16;
        const ABRQ1 = 1 << 15;
        const TERR1 = 1 << 11;
        const ALST1 = 1 << 10;
        const TXOK1 = 1 << 9;
        const RQCP1 = 1 << 8;
        const ABRQ0 = 1 << 7;
        const TERR0 = 1 << 3;
        const ALST0 = 1 << 2;
        const TXOK0 = 1 << 1;
        const RQCP0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RFIFO0: u32 {
        const RFOM0 = 1 << 5;
        const FOVR0 = 1 << 4;
        const FULL0 = 1 << 3;

        const FMP0_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RFIFO1: u32 {
        const RFOM1 = 1 << 5;
        const FOVR1 = 1 << 4;
        const FULL1 = 1 << 3;

        const FMP1_MASK = 0x3 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTENR: u32 {
        const SLKIE = 1 << 17;
        const WKUIE = 1 << 16;
        const ERRIE = 1 << 15;
        const LECIE = 1 << 11;
        const BOFIE = 1 << 10;
        const EPVIE = 1 << 9;
        const EWGIE = 1 << 8;
        const FOVIE1 = 1 << 6;
        const FFIE1 = 1 << 5;
        const EMPIE1 = 1 << 4;
        const FOVIE0 = 1 << 3;
        const FFIE0 = 1 << 2;
        const FMPIE0 = 1 << 1;
        const TMEIE = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ERRSR: u32 {
        const REC_MASK = 0xFF << 24;

        const TEC_MASK = 0xFF << 16;

        const LEC_2 = 1 << 6;
        const LEC_1 = 1 << 5;
        const LEC_0 = 1 << 4;
        const LEC_MASK = 0x7 << 4;

        const BOFF = 1 << 2;
        const EPVF = 1 << 1;
        const EWGF = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BTIMR: u32 {
        const SILM = 1 << 31;
        const LBKM = 1 << 30;

        const SJW_MASK = 0xF << 24;

        const TS2_MASK = 0xF << 20;

        const TS1_MASK = 0xF << 16;

        const BTR_TS1_T_MASK = 0xF << 12;

        const BRP_MASK = 0x3FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TTCTLR: u32 {
        const MODE = 1 << 17;
        const TIMRST = 1 << 16;

        const TIMCMV_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TTCNT: u32 {
        const TIMCNT_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TERR_CNT: u32 {
        const TERR_CNT_MASK = 0x1FF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TXMIR: u32 {
        const STID_MASK = 0x7FF << 21;

        const EXID_MASK = 0x3FFFF << 3;

        const IDE = 1 << 2;
        const RTR = 1 << 1;
        const TXRQ = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TXMDTR: u32 {
        const TIME_MASK = 0xFFFF << 16;

        const TGT = 1 << 8;

        const DLC_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TXMDLR: u32 {
        const DATA3_MASK = 0xFF << 24;

        const DATA2_MASK = 0xFF << 16;

        const DATA1_MASK = 0xFF << 8;

        const DATA0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TXMDHR: u32 {
        const DATA7_MASK = 0xFF << 24;

        const DATA6_MASK = 0xFF << 16;

        const DATA5_MASK = 0xFF << 8;

        const DATA4_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RXMIR: u32 {
        const STID_MASK = 0x7FF << 21;

        const EXID_MASK = 0x3FFFF << 3;

        const IDE = 1 << 2;
        const RTR = 1 << 1;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RXMDTR: u32 {
        const TIME_MASK = 0xFFFF << 16;

        const RES = 1 << 8;
        const ESI = 1 << 5;
        const BRS = 1 << 4;

        const DLC_MASK = 0xF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RXMDLR: u32 {
        const DATA3_MASK = 0xFF << 24;

        const DATA2_MASK = 0xFF << 16;

        const DATA1_MASK = 0xFF << 8;

        const DATA0_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RXMDHR: u32 {
        const DATA7_MASK = 0xFF << 24;

        const DATA6_MASK = 0xFF << 16;

        const DATA5_MASK = 0xFF << 8;

        const DATA4_MASK = 0xFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FCTLR: u32 {
        const CAN3SB_MASK = 0x3F << 16;

        const CAN2SB_MASK = 0x1F << 8;

        const TINIT = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FMCFGR: u32 {
        const FBM27 = 1 << 27;
        const FBM26 = 1 << 26;
        const FBM25 = 1 << 25;
        const FBM24 = 1 << 24;
        const FBM23 = 1 << 23;
        const FBM22 = 1 << 22;
        const FBM21 = 1 << 21;
        const FBM20 = 1 << 20;
        const FBM19 = 1 << 19;
        const FBM18 = 1 << 18;
        const FBM17 = 1 << 17;
        const FBM16 = 1 << 16;
        const FBM15 = 1 << 15;
        const FBM14 = 1 << 14;
        const FBM13 = 1 << 13;
        const FBM12 = 1 << 12;
        const FBM11 = 1 << 11;
        const FBM10 = 1 << 10;
        const FBM9 = 1 << 9;
        const FBM8 = 1 << 8;
        const FBM7 = 1 << 7;
        const FBM6 = 1 << 6;
        const FBM5 = 1 << 5;
        const FBM4 = 1 << 4;
        const FBM3 = 1 << 3;
        const FBM2 = 1 << 2;
        const FBM1 = 1 << 1;
        const FBM0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FSCFGR: u32 {
        const FSC27 = 1 << 27;
        const FSC26 = 1 << 26;
        const FSC25 = 1 << 25;
        const FSC24 = 1 << 24;
        const FSC23 = 1 << 23;
        const FSC22 = 1 << 22;
        const FSC21 = 1 << 21;
        const FSC20 = 1 << 20;
        const FSC19 = 1 << 19;
        const FSC18 = 1 << 18;
        const FSC17 = 1 << 17;
        const FSC16 = 1 << 16;
        const FSC15 = 1 << 15;
        const FSC14 = 1 << 14;
        const FSC13 = 1 << 13;
        const FSC12 = 1 << 12;
        const FSC11 = 1 << 11;
        const FSC10 = 1 << 10;
        const FSC9 = 1 << 9;
        const FSC8 = 1 << 8;
        const FSC7 = 1 << 7;
        const FSC6 = 1 << 6;
        const FSC5 = 1 << 5;
        const FSC4 = 1 << 4;
        const FSC3 = 1 << 3;
        const FSC2 = 1 << 2;
        const FSC1 = 1 << 1;
        const FSC0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FAFIFOR: u32 {
        const FFA27 = 1 << 27;
        const FFA26 = 1 << 26;
        const FFA25 = 1 << 25;
        const FFA24 = 1 << 24;
        const FFA23 = 1 << 23;
        const FFA22 = 1 << 22;
        const FFA21 = 1 << 21;
        const FFA20 = 1 << 20;
        const FFA19 = 1 << 19;
        const FFA18 = 1 << 18;
        const FFA17 = 1 << 17;
        const FFA16 = 1 << 16;
        const FFA15 = 1 << 15;
        const FFA14 = 1 << 14;
        const FFA13 = 1 << 13;
        const FFA12 = 1 << 12;
        const FFA11 = 1 << 11;
        const FFA10 = 1 << 10;
        const FFA9 = 1 << 9;
        const FFA8 = 1 << 8;
        const FFA7 = 1 << 7;
        const FFA6 = 1 << 6;
        const FFA5 = 1 << 5;
        const FFA4 = 1 << 4;
        const FFA3 = 1 << 3;
        const FFA2 = 1 << 2;
        const FFA1 = 1 << 1;
        const FFA0 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FWR: u32 {
        const FACT27 = 1 << 27;
        const FACT26 = 1 << 26;
        const FACT25 = 1 << 25;
        const FACT24 = 1 << 24;
        const FACT23 = 1 << 23;
        const FACT22 = 1 << 22;
        const FACT21 = 1 << 21;
        const FACT20 = 1 << 20;
        const FACT19 = 1 << 19;
        const FACT18 = 1 << 18;
        const FACT17 = 1 << 17;
        const FACT16 = 1 << 16;
        const FACT15 = 1 << 15;
        const FACT14 = 1 << 14;
        const FACT13 = 1 << 13;
        const FACT12 = 1 << 12;
        const FACT11 = 1 << 11;
        const FACT10 = 1 << 10;
        const FACT9 = 1 << 9;
        const FACT8 = 1 << 8;
        const FACT7 = 1 << 7;
        const FACT6 = 1 << 6;
        const FACT5 = 1 << 5;
        const FACT4 = 1 << 4;
        const FACT3 = 1 << 3;
        const FACT2 = 1 << 2;
        const FACT1 = 1 << 1;
        const FACT0 = 1 << 0;
    }
}

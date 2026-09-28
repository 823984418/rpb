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
        const INRQ = 0x0001;
        const SLEEP = 0x0002;
        const TXFP = 0x0004;
        const RFLM = 0x0008;
        const NART = 0x0010;
        const AWUM = 0x0020;
        const ABOM = 0x0040;
        const TTCM = 0x0080;
        const RESET = 0x8000;
        const DBF = 0x10000;
        const CFGCANM = 0x20000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct STATR: u32 {
        const INAK = 0x0001;
        const SLAK = 0x0002;
        const ERRI = 0x0004;
        const WKUI = 0x0008;
        const SLAKI = 0x0010;
        const TXM = 0x0100;
        const RXM = 0x0200;
        const SAMP = 0x0400;
        const RX = 0x0800;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TSTATR: u32 {
        const RQCP0 = 0x00000001;
        const TXOK0 = 0x00000002;
        const ALST0 = 0x00000004;
        const TERR0 = 0x00000008;
        const ABRQ0 = 0x00000080;
        const RQCP1 = 0x00000100;
        const TXOK1 = 0x00000200;
        const ALST1 = 0x00000400;
        const TERR1 = 0x00000800;
        const ABRQ1 = 0x00008000;
        const RQCP2 = 0x00010000;
        const TXOK2 = 0x00020000;
        const ALST2 = 0x00040000;
        const TERR2 = 0x00080000;
        const ABRQ2 = 0x00800000;
        const CODE = 0x03000000;

        const TME = 0x1C000000;
        const TME0 = 0x04000000;
        const TME1 = 0x08000000;
        const TME2 = 0x10000000;

        const LOW = 0xE0000000;
        const LOW0 = 0x20000000;
        const LOW1 = 0x40000000;
        const LOW2 = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RFIFO0: u32 {
        const FMP0 = 0x03;
        const FULL0 = 0x08;
        const FOVR0 = 0x10;
        const RFOM0 = 0x20;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RFIFO1: u32 {
        const FMP1 = 0x03;
        const FULL1 = 0x08;
        const FOVR1 = 0x10;
        const RFOM1 = 0x20;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTENR: u32 {
        const TMEIE = 0x00000001;
        const FMPIE0 = 0x00000002;
        const FFIE0 = 0x00000004;
        const FOVIE0 = 0x00000008;
        const EMPIE1 = 0x00000010;
        const FFIE1 = 0x00000020;
        const FOVIE1 = 0x00000040;
        const EWGIE = 0x00000100;
        const EPVIE = 0x00000200;
        const BOFIE = 0x00000400;
        const LECIE = 0x00000800;
        const ERRIE = 0x00008000;
        const WKUIE = 0x00010000;
        const SLKIE = 0x00020000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct ERRSR: u32 {
        const EWGF = 0x00000001;
        const EPVF = 0x00000002;
        const BOFF = 0x00000004;

        const LEC = 0x00000070;
        const LEC_0 = 0x00000010;
        const LEC_1 = 0x00000020;
        const LEC_2 = 0x00000040;

        const TEC = 0x00FF0000;
        const REC = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BTIMR: u32 {
        const BRP = 0x000003FF;
        const BTR_TS1_T = 0x0000F000;
        const TS1 = 0x000F0000;
        const TS2 = 0x00F00000;
        const SJW = 0x0F000000;

        const LBKM = 0x40000000;
        const SILM = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TTCTLR: u32 {
        const TIMCMV = 0x0000FFFF;
        const TIMRST = 0x00010000;
        const MODE = 0x00020000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TTCNT: u32 {
        const TIMCNT = 0x0000FFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TERR_CNT: u32 {
        const TERR_CNT = 0x000001FF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TXMIR: u32 {
        const TXRQ = 0x00000001;
        const RTR = 0x00000002;
        const IDE = 0x00000004;
        const EXID = 0x001FFFF8;
        const STID = 0xFFE00000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TXMDTR: u32 {
        const DLC = 0x0000000F;
        const TGT = 0x00000100;
        const TIME = 0xFFFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TXMDLR: u32 {
        const DATA0 = 0x000000FF;
        const DATA1 = 0x0000FF00;
        const DATA2 = 0x00FF0000;
        const DATA3 = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TXMDHR: u32 {
        const DATA4 = 0x000000FF;
        const DATA5 = 0x0000FF00;
        const DATA6 = 0x00FF0000;
        const DATA7 = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RXMIR: u32 {
        const RTR = 0x00000002;
        const IDE = 0x00000004;
        const EXID = 0x001FFFF8;
        const STID = 0xFFE00000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RXMDTR: u32 {
        const DLC = 0x0000000F;
        const BRS = 0x00000010;
        const ESI = 0x00000020;
        const RES = 0x00000100;
        const TIME = 0xFFFF0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RXMDLR: u32 {
        const DATA0 = 0x000000FF;
        const DATA1 = 0x0000FF00;
        const DATA2 = 0x00FF0000;
        const DATA3 = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct RXMDHR: u32 {
        const DATA4 = 0x000000FF;
        const DATA5 = 0x0000FF00;
        const DATA6 = 0x00FF0000;
        const DATA7 = 0xFF000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FCTLR: u32 {
        const TINIT = 0x00000001;

        const CAN2SB = 0x00001F00;
        const CAN3SB = 0x003F0000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FMCFGR: u32 {
        const FBM0 = 0x00000001;
        const FBM1 = 0x00000002;
        const FBM2 = 0x00000004;
        const FBM3 = 0x00000008;
        const FBM4 = 0x00000010;
        const FBM5 = 0x00000020;
        const FBM6 = 0x00000040;
        const FBM7 = 0x00000080;
        const FBM8 = 0x00000100;
        const FBM9 = 0x00000200;
        const FBM10 = 0x00000400;
        const FBM11 = 0x00000800;
        const FBM12 = 0x00001000;
        const FBM13 = 0x00002000;
        const FBM14 = 0x00004000;
        const FBM15 = 0x00008000;
        const FBM16 = 0x00010000;
        const FBM17 = 0x00020000;
        const FBM18 = 0x00040000;
        const FBM19 = 0x00080000;
        const FBM20 = 0x00100000;
        const FBM21 = 0x00200000;
        const FBM22 = 0x00400000;
        const FBM23 = 0x00800000;
        const FBM24 = 0x01000000;
        const FBM25 = 0x02000000;
        const FBM26 = 0x04000000;
        const FBM27 = 0x08000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FSCFGR: u32 {
        const FSC0 = 0x00000001;
        const FSC1 = 0x00000002;
        const FSC2 = 0x00000004;
        const FSC3 = 0x00000008;
        const FSC4 = 0x00000010;
        const FSC5 = 0x00000020;
        const FSC6 = 0x00000040;
        const FSC7 = 0x00000080;
        const FSC8 = 0x00000100;
        const FSC9 = 0x00000200;
        const FSC10 = 0x00000400;
        const FSC11 = 0x00000800;
        const FSC12 = 0x00001000;
        const FSC13 = 0x00002000;
        const FSC14 = 0x00004000;
        const FSC15 = 0x00008000;
        const FSC16 = 0x00010000;
        const FSC17 = 0x00020000;
        const FSC18 = 0x00040000;
        const FSC19 = 0x00080000;
        const FSC20 = 0x00100000;
        const FSC21 = 0x00200000;
        const FSC22 = 0x00400000;
        const FSC23 = 0x00800000;
        const FSC24 = 0x01000000;
        const FSC25 = 0x02000000;
        const FSC26 = 0x04000000;
        const FSC27 = 0x08000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FAFIFOR: u32 {
        const FFA0 = 0x00000001;
        const FFA1 = 0x00000002;
        const FFA2 = 0x00000004;
        const FFA3 = 0x00000008;
        const FFA4 = 0x00000010;
        const FFA5 = 0x00000020;
        const FFA6 = 0x00000040;
        const FFA7 = 0x00000080;
        const FFA8 = 0x00000100;
        const FFA9 = 0x00000200;
        const FFA10 = 0x00000400;
        const FFA11 = 0x00000800;
        const FFA12 = 0x00001000;
        const FFA13 = 0x00002000;
        const FFA14 = 0x00004000;
        const FFA15 = 0x00008000;
        const FFA16 = 0x00010000;
        const FFA17 = 0x00020000;
        const FFA18 = 0x00040000;
        const FFA19 = 0x00080000;
        const FFA20 = 0x00100000;
        const FFA21 = 0x00200000;
        const FFA22 = 0x00400000;
        const FFA23 = 0x00800000;
        const FFA24 = 0x01000000;
        const FFA25 = 0x02000000;
        const FFA26 = 0x04000000;
        const FFA27 = 0x08000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FWR: u32 {
        const FACT0 = 0x00000001;
        const FACT1 = 0x00000002;
        const FACT2 = 0x00000004;
        const FACT3 = 0x00000008;
        const FACT4 = 0x00000010;
        const FACT5 = 0x00000020;
        const FACT6 = 0x00000040;
        const FACT7 = 0x00000080;
        const FACT8 = 0x00000100;
        const FACT9 = 0x00000200;
        const FACT10 = 0x00000400;
        const FACT11 = 0x00000800;
        const FACT12 = 0x00001000;
        const FACT13 = 0x00002000;
        const FACT14 = 0x00004000;
        const FACT15 = 0x00008000;
        const FACT16 = 0x00010000;
        const FACT17 = 0x00020000;
        const FACT18 = 0x00040000;
        const FACT19 = 0x00080000;
        const FACT20 = 0x00100000;
        const FACT21 = 0x00200000;
        const FACT22 = 0x00400000;
        const FACT23 = 0x00800000;
        const FACT24 = 0x01000000;
        const FACT25 = 0x02000000;
        const FACT26 = 0x04000000;
        const FACT27 = 0x08000000;
    }
}

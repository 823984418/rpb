use bitflags::bitflags;

use crate::register::Reg;

#[derive(Debug)]
#[repr(C)]
pub struct DMA_Channel {
    pub CFGR: Reg<CFGR>,
    pub CNTR: Reg<CNTR>,
    pub PADDR: Reg<PADDR>,
    pub MADDR: Reg<MADDR>,
    pub M1ADDR: Reg<M1ADDR>,
}

#[derive(Debug)]
#[repr(C)]
pub struct DMA {
    pub INTFR: Reg<INTFR>,
    pub INTFCR: Reg<INTFCR>,
}

#[derive(Debug)]
#[repr(C)]
pub struct DMAMUX {
    pub CFGR0_3: Reg<CFGR0_3>,
    pub CFGR4_7: Reg<CFGR4_7>,
    pub CFGR8_11: Reg<CFGR8_11>,
    pub CFGR12_15: Reg<CFGR12_15>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFR: u32 {
        const TEIF8 = 1 << 31;
        const HTIF8 = 1 << 30;
        const TCIF8 = 1 << 29;
        const GIF8 = 1 << 28;
        const TEIF7 = 1 << 27;
        const HTIF7 = 1 << 26;
        const TCIF7 = 1 << 25;
        const GIF7 = 1 << 24;
        const TEIF6 = 1 << 23;
        const HTIF6 = 1 << 22;
        const TCIF6 = 1 << 21;
        const GIF6 = 1 << 20;
        const TEIF5 = 1 << 19;
        const HTIF5 = 1 << 18;
        const TCIF5 = 1 << 17;
        const GIF5 = 1 << 16;
        const TEIF4 = 1 << 15;
        const HTIF4 = 1 << 14;
        const TCIF4 = 1 << 13;
        const GIF4 = 1 << 12;
        const TEIF3 = 1 << 11;
        const HTIF3 = 1 << 10;
        const TCIF3 = 1 << 9;
        const GIF3 = 1 << 8;
        const TEIF2 = 1 << 7;
        const HTIF2 = 1 << 6;
        const TCIF2 = 1 << 5;
        const GIF2 = 1 << 4;
        const TEIF1 = 1 << 3;
        const HTIF1 = 1 << 2;
        const TCIF1 = 1 << 1;
        const GIF1 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFCR: u32 {
        const CTEIF8 = 1 << 31;
        const CHTIF8 = 1 << 30;
        const CTCIF8 = 1 << 29;
        const CGIF8 = 1 << 28;
        const CTEIF7 = 1 << 27;
        const CHTIF7 = 1 << 26;
        const CTCIF7 = 1 << 25;
        const CGIF7 = 1 << 24;
        const CTEIF6 = 1 << 23;
        const CHTIF6 = 1 << 22;
        const CTCIF6 = 1 << 21;
        const CGIF6 = 1 << 20;
        const CTEIF5 = 1 << 19;
        const CHTIF5 = 1 << 18;
        const CTCIF5 = 1 << 17;
        const CGIF5 = 1 << 16;
        const CTEIF4 = 1 << 15;
        const CHTIF4 = 1 << 14;
        const CTCIF4 = 1 << 13;
        const CGIF4 = 1 << 12;
        const CTEIF3 = 1 << 11;
        const CHTIF3 = 1 << 10;
        const CTCIF3 = 1 << 9;
        const CGIF3 = 1 << 8;
        const CTEIF2 = 1 << 7;
        const CHTIF2 = 1 << 6;
        const CTCIF2 = 1 << 5;
        const CGIF2 = 1 << 4;
        const CTEIF1 = 1 << 3;
        const CHTIF1 = 1 << 2;
        const CTCIF1 = 1 << 1;
        const CGIF1 = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const FLAG_CURMEM = 1 << 16;
        const DOUBLE_MODE = 1 << 15;
        const MEM2MEM = 1 << 14;

        const PL_MASK = 0x3 << 12;

        const MSIZE_MASK = 0x3 << 10;

        const PSIZE_MASK = 0x3 << 8;

        const MINC = 1 << 7;
        const PINC = 1 << 6;
        const CIRC = 1 << 5;
        const DIR = 1 << 4;
        const TEIE = 1 << 3;
        const HTIE = 1 << 2;
        const TCIE = 1 << 1;
        const EN = 1 << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTR: u32 {
        const NDT_MASK = 0xFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PADDR: u32 {
        const PA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MADDR: u32 {
        const MA_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct M1ADDR: u32 {
        const M1A_MASK = 0xFFFFFFFF << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR0_3: u32 {
        const CH3_MASK = 0x7F << 24;

        const CH2_MASK = 0x7F << 16;

        const CH1_MASK = 0x7F << 8;

        const CH0_MASK = 0x7F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR4_7: u32 {
        const CH7_MASK = 0x7F << 24;

        const CH6_MASK = 0x7F << 16;

        const CH5_MASK = 0x7F << 8;

        const CH4_MASK = 0x7F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR8_11: u32 {
        const CH11_MASK = 0x7F << 24;

        const CH10_MASK = 0x7F << 16;

        const CH9_MASK = 0x7F << 8;

        const CH8_MASK = 0x7F << 0;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR12_15: u32 {
        const CH15_MASK = 0x7F << 24;

        const CH14_MASK = 0x7F << 16;

        const CH13_MASK = 0x7F << 8;

        const CH12_MASK = 0x7F << 0;
    }
}

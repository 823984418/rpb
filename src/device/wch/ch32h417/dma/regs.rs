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
        const GIF1 = 0x00000001;
        const TCIF1 = 0x00000002;
        const HTIF1 = 0x00000004;
        const TEIF1 = 0x00000008;
        const GIF2 = 0x00000010;
        const TCIF2 = 0x00000020;
        const HTIF2 = 0x00000040;
        const TEIF2 = 0x00000080;
        const GIF3 = 0x00000100;
        const TCIF3 = 0x00000200;
        const HTIF3 = 0x00000400;
        const TEIF3 = 0x00000800;
        const GIF4 = 0x00001000;
        const TCIF4 = 0x00002000;
        const HTIF4 = 0x00004000;
        const TEIF4 = 0x00008000;
        const GIF5 = 0x00010000;
        const TCIF5 = 0x00020000;
        const HTIF5 = 0x00040000;
        const TEIF5 = 0x00080000;
        const GIF6 = 0x00100000;
        const TCIF6 = 0x00200000;
        const HTIF6 = 0x00400000;
        const TEIF6 = 0x00800000;
        const GIF7 = 0x01000000;
        const TCIF7 = 0x02000000;
        const HTIF7 = 0x04000000;
        const TEIF7 = 0x08000000;
        const GIF8 = 0x10000000;
        const TCIF8 = 0x20000000;
        const HTIF8 = 0x40000000;
        const TEIF8 = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct INTFCR: u32 {
        const CGIF1 = 0x00000001;
        const CTCIF1 = 0x00000002;
        const CHTIF1 = 0x00000004;
        const CTEIF1 = 0x00000008;
        const CGIF2 = 0x00000010;
        const CTCIF2 = 0x00000020;
        const CHTIF2 = 0x00000040;
        const CTEIF2 = 0x00000080;
        const CGIF3 = 0x00000100;
        const CTCIF3 = 0x00000200;
        const CHTIF3 = 0x00000400;
        const CTEIF3 = 0x00000800;
        const CGIF4 = 0x00001000;
        const CTCIF4 = 0x00002000;
        const CHTIF4 = 0x00004000;
        const CTEIF4 = 0x00008000;
        const CGIF5 = 0x00010000;
        const CTCIF5 = 0x00020000;
        const CHTIF5 = 0x00040000;
        const CTEIF5 = 0x00080000;
        const CGIF6 = 0x00100000;
        const CTCIF6 = 0x00200000;
        const CHTIF6 = 0x00400000;
        const CTEIF6 = 0x00800000;
        const CGIF7 = 0x01000000;
        const CTCIF7 = 0x02000000;
        const CHTIF7 = 0x04000000;
        const CTEIF7 = 0x08000000;
        const CGIF8 = 0x10000000;
        const CTCIF8 = 0x20000000;
        const CHTIF8 = 0x40000000;
        const CTEIF8 = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR: u32 {
        const EN = 0x0001;
        const TCIE = 0x0002;
        const HTIE = 0x0004;
        const TEIE = 0x0008;
        const DIR = 0x0010;
        const CIRC = 0x0020;
        const PINC = 0x0040;
        const MINC = 0x0080;

        const PSIZE = 0x0300;
        const PSIZE_0 = 0x0100;
        const PSIZE_1 = 0x0200;

        const MSIZE = 0x0C00;
        const MSIZE_0 = 0x0400;
        const MSIZE_1 = 0x0800;

        const PL = 0x3000;
        const PL_0 = 0x1000;
        const PL_1 = 0x2000;

        const MEM2MEM = 0x4000;
        const DOUBLE_MODE = 0x8000;
        const FLAG_CURMEM = 0x00010000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CNTR: u32 {
        const NDT = 0xFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct PADDR: u32 {
        const PA = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MADDR: u32 {
        const MA = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct M1ADDR: u32 {
        const M1A = 0xFFFFFFFF;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR0_3: u32 {
        const CH0 = 0x0000007F;
        const CH1 = 0x00007F00;
        const CH2 = 0x007F0000;
        const CH3 = 0x7F000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR4_7: u32 {
        const CH4 = 0x0000007F;
        const CH5 = 0x00007F00;
        const CH6 = 0x007F0000;
        const CH7 = 0x7F000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR8_11: u32 {
        const CH8 = 0x0000007F;
        const CH9 = 0x00007F00;
        const CH10 = 0x007F0000;
        const CH11 = 0x7F000000;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CFGR12_15: u32 {
        const CH12 = 0x0000007F;
        const CH13 = 0x00007F00;
        const CH14 = 0x007F0000;
        const CH15 = 0x7F000000;
    }
}

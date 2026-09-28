use crate::register::{Reg32, Reserved};

#[derive(Debug)]
#[repr(C)]
pub struct SDIO {
    pub POWER: Reg32,
    pub CLKCR: Reg32,
    pub ARG: Reg32,
    pub CMD: Reg32,
    pub RESPCMD: Reg32,
    pub RESP1: Reg32,
    pub RESP2: Reg32,
    pub RESP3: Reg32,
    pub RESP4: Reg32,
    pub DTIMER: Reg32,
    pub DLEN: Reg32,
    pub DCTRL: Reg32,
    pub DCOUNT: Reg32,
    pub STA: Reg32,
    pub ICR: Reg32,
    pub MASK: Reg32,
    RESERVED0: Reserved<[u32; 2]>,
    pub FIFOCNT: Reg32,
    RESERVED1: Reserved<[u32; 5]>,
    pub DCTRL2: Reg32,
    RESERVED2: Reserved<[u32; 7]>,
    pub FIFO: Reg32,
}

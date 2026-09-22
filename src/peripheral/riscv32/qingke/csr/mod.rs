use core::ops::{Deref, DerefMut};

use crate::peripheral::riscv32::csr::mstatus::MSTATUS;
use crate::peripheral::riscv32::csr::{CSR, RiscvCSR};

pub mod cache_pmp_ovr;
pub mod cache_strtg_ctlr;
pub mod corecfgr;
pub mod inestcr;
pub mod intsyscr;
pub mod mtvec;
pub mod opcache_ctlr;

use cache_strtg_ctlr::CACHE_STRTG_CTLR;
use corecfgr::CORECFGR;
use inestcr::INESTCR;
use intsyscr::INTSYSCR;

#[derive(Debug)]
pub struct QingKeCSR {
    pub gintenr: CSR<MSTATUS, 0x800>,
    pub cache_strtg_ctlr: CSR<CACHE_STRTG_CTLR, 0xBC2>,
    pub corecfgr: CSR<CORECFGR, 0xBC0>,
    pub inestcr: CSR<INESTCR, 0xBC1>,
    pub intsyscr: CSR<INTSYSCR, 0xBC1>,
}

impl Deref for QingKeCSR {
    type Target = RiscvCSR;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for QingKeCSR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

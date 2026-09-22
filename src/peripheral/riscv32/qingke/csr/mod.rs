use core::ops::{Deref, DerefMut};

use crate::peripheral::riscv32::csr::mstatus::MSTATUS;
use crate::peripheral::riscv32::csr::{CSR, RiscvCSR};

pub mod cache_pmp_ovr;
pub mod cache_strtg_ctlr;
pub mod corecfgr;
pub mod hw_popdm_addr;
pub mod inestcr;
pub mod intsyscr;
pub mod memary_cfgr;
pub mod meminfo;
pub mod mie;
pub mod mtvec;
pub mod opcache_ctlr;
pub mod tcm_rrduty_cfgr;

use cache_pmp_ovr::CACHE_PMP_OVR;
use cache_strtg_ctlr::CACHE_STRTG_CTLR;
use corecfgr::CORECFGR;
use hw_popdm_addr::HW_POPDM_ADDR;
use inestcr::INESTCR;
use intsyscr::INTSYSCR;
use memary_cfgr::MEMARY_CFGR;
use meminfo::MEMINFO;
use mie::MIE;
use opcache_ctlr::OPCACHE_CTLR;
use tcm_rrduty_cfgr::TCM_RRDUTY_CFGR;

#[derive(Debug)]
pub struct QingKeCSR {
    pub RISCV_CSR: RiscvCSR,

    pub gintenr: CSR<MSTATUS, 0x800>,
    pub intsyscr: CSR<INTSYSCR, 0x804>,

    pub corecfgr: CSR<CORECFGR, 0xBC0>,
    pub inestcr: CSR<INESTCR, 0xBC1>,
    pub cache_strtg_ctlr: CSR<CACHE_STRTG_CTLR, 0xBC2>,
    pub cache_pmp_ovr: CSR<CACHE_PMP_OVR, 0xBC3>,
    pub hw_popdm_addr: CSR<HW_POPDM_ADDR, 0xBC4>,
    pub memary_cfgr: CSR<MEMARY_CFGR, 0xBC5>,
    pub tcm_rrduty_cfgr: CSR<TCM_RRDUTY_CFGR, 0xBC6>,

    pub mie: CSR<MIE, 0xBC8>,

    pub opcache_ctlr: CSR<OPCACHE_CTLR, 0xBD0>,

    pub meminfo: CSR<MEMINFO, 0xFC0>,
}

impl Deref for QingKeCSR {
    type Target = RiscvCSR;

    fn deref(&self) -> &Self::Target {
        &self.RISCV_CSR
    }
}

impl DerefMut for QingKeCSR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.RISCV_CSR
    }
}

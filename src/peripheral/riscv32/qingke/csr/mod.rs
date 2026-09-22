use crate::peripheral::riscv32::csr::CSR;
use crate::peripheral::riscv32::csr::csr_mstatus::mstatus;

pub mod csr_cache_pmp_ovr;
pub mod csr_cache_strtg_ctlr;
pub mod csr_corecfgr;
pub mod csr_inestcr;
pub mod csr_intsyscr;
pub mod csr_mtvec;
pub mod csr_opcache_ctlr;

use csr_cache_strtg_ctlr::cache_strtg_ctlr;

#[derive(Debug)]
pub struct QingKeCSR {
    pub gintenr: CSR<mstatus, 0x800>,
    pub cache_strtg_ctlr: CSR<cache_strtg_ctlr, 0xBC2>,
}

use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct cache_strtg_ctlr: usize {
          const IC_MEM1_STRTG = 1 << 27;
          const IC_MEM0_STRTG = 1 << 26;
          const IC_SRAM_STRTG = 1 << 25;
          const IC_CODE_STRTG = 1 << 24;
          const IC_DISABLE = 1 << 1;
     }
}

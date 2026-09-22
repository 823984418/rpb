use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct MEMARY_CFGR: usize {
          const MEM0_ADDR_HI_MASK = 0xFFF << 20;

          const DTCM_RR_MODE_MASK = 0x3 << 18;

          const ITCM_RR_MODE = 0x3 << 16;

          const CO_IFU_DTCM_MODE_READ = 1 << 11;
          const CO_IFU_DTCM_MODE_WRITE = 1 << 10;
          const CO_IFU_ITCM_MODE_READ = 1 << 9;
          const CO_IFU_ITCM_MODE_WRITE = 1 << 8;
          const CO_LSU_DTCM_MODE_READ = 1 << 7;
          const CO_LSU_DTCM_MODE_WRITE = 1 << 6;
          const CO_LSU_ITCM_MODE_READ = 1 << 5;
          const CO_LSU_ITCM_MODE_WRITE = 1 << 4;
          const CO_DMA_DTCM_MODE_READ = 1 << 3;
          const CO_DMA_DTCM_MODE_WRITE = 1 << 2;
          const CO_DMA_ITCM_MODE_READ = 1 << 1;
          const CO_DMA_ITCM_MODE_WRITE = 1 << 0;
     }
}

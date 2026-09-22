use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct MEMINFO: usize {
          const DTCM_DATA_SIZE_MASK = 0xF << 26;
          const DTCM_LINE_SIZE_MASK = 0x3 << 24;
          const ITCM_DATA_SIZE_MASK = 0xF << 18;
          const ITCM_LINE_SIZE_MASK = 0x3 << 16;
          const ICACHE_WAY_MASK = 0x3 << 5;
          const ICACHE_DATA_SIZE_MASK = 0x7 << 2;
          const ICACHE_LINE_SIZE_MASK = 0x3 << 0;
     }
}

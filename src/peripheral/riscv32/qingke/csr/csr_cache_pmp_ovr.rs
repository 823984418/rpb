use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct cache_pmp_ovr: usize {
          const IC_PMP3CACHE_STRTG = 1 << 12;
          const IC_PMP2CACHE_STRTG = 1 << 8;
          const IC_PMP1CACHE_STRTG = 1 << 4;
          const IC_PMP0CACHE_STRTG = 1 << 0;
     }
}

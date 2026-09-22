use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct INTSYSCR: usize {
          const LOCK = 1 << 31;
          const GIHWSTKNEN = 1 << 5;

          const PMTCFG_3 = 0b11 << 2;
          const PMTCFG_2 = 0b10 << 2;
          const PMTCFG_1 = 0b01 << 2;
          const PMTCFG_0 = 0b00 << 2;
          const PMTCFG_MASK = 0x3 << 2;

          const INESTEN = 1 << 1;
          const HWSTKEN = 1 << 0;
     }
}

use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct TCM_RRDUTY_CFGR: usize {
          const C1_DTCM_RR_CYCLE_MASK = 0xFF << 24;
          const CX_DTCM_RR_CYCLE_MASK = 0xFF << 16;
          const C1_ITCM_RR_CYCLE_MASK = 0xFF << 8;
          const CX_ITCM_RR_CYCLE_MASK = 0xFF << 0;
     }
}

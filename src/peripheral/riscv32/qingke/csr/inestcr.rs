use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct INESTCR: usize {
          const LSU_NMI_STA = 1 << 31;
          const NEST_OV = 1 << 30;

          const NEST_STA_MASK = 0xF << 8;

          const NEST_LVL_8 = 0b111 << 0;
          const NEST_LVL_7 = 0b110 << 0;
          const NEST_LVL_6 = 0b101 << 0;
          const NEST_LVL_5 = 0b100 << 0;
          const NEST_LVL_4 = 0b011 << 0;
          const NEST_LVL_3 = 0b010 << 0;
          const NEST_LVL_2 = 0b001 << 0;
          const NEST_LVL_1 = 0b000 << 0;
          const NEST_LVL_MASK = 0x7 << 0;
     }
}

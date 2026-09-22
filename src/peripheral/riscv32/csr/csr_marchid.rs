use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct marchid: usize {
          const VENDER0_MASK = 0x1F << 26;
          const VENDER1_MASK = 0x1F << 21;
          const VENDER2_MASK = 0x1F << 16;

          const ARCH_MASK = 0x1F << 10;
          const SERIAL_MASK = 0x1F << 5;
          const VERSION_MASK = 0x1F << 0;
     }
}

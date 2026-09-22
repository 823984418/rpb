use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct OPCACHE_CTLR: usize {
          /// 直接地址，否则为索引模式
          const IDX_MODE = 1 << 2;

          /// 无效指令缓存
          const OPCODE_ICACHE_INVALIDATE = 0b00 << 0;
          /// 操作码
          const OPCODE_MASK = 0x3 << 0;
     }
}

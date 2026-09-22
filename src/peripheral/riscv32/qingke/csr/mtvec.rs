use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct MTVEC: usize {
          /// 使用地址模式
          const MODE1 = 1 << 1;

          /// 中断编号*4偏移
          const MODE0 = 1 << 0;
     }
}

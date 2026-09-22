use bitflags::bitflags;

bitflags! {
     #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
     pub struct MISA: usize {
          const MXL_32 = 1 << 30;
          const MXL_MASK = 0x3 << 30;

          const Z = 1 << 25;
          const Y = 1 << 24;
          const X = 1 << 23;
          const W = 1 << 22;
          const V = 1 << 21;
          const U = 1 << 20;
          const T = 1 << 19;
          const S = 1 << 18;
          const R = 1 << 17;
          const Q = 1 << 16;
          const P = 1 << 15;
          const O = 1 << 14;
          const N = 1 << 13;
          const M = 1 << 12;
          const L = 1 << 11;
          const K = 1 << 10;
          const J = 1 << 9;
          const I = 1 << 8;
          const H = 1 << 7;
          const G = 1 << 6;
          const F = 1 << 5;
          const E = 1 << 4;
          const D = 1 << 3;
          const C = 1 << 2;
          const B = 1 << 1;
          const A = 1 << 0;
     }
}

use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// DIV register block.
#[derive(Debug)]
#[repr(C)]
pub struct DIV {
    pub CSR: Reg<CSR>,
    pub DND: Reg32,
    pub SOR: Reg32,
    pub RLT: Reg32,
}

bitflags! {
    /// `CSR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct CSR: u32 {
        const SAT = 1 << 2;

        const SIGN = 1 << 1;

        const RUN = 1 << 0;
    }
}

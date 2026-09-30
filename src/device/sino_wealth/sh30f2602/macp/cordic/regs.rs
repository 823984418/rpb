use bitflags::bitflags;

use crate::register::{Reg, Reg32};

/// CORDIC register block.
#[derive(Debug)]
#[repr(C)]
pub struct CORDIC {
    pub SR: Reg<SR>,
    pub X: Reg32,
    pub Y: Reg32,
    pub Z: Reg32,
}

bitflags! {
    /// `CSR` (0x0).
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SR: u32 {
        const MODE = 1 << 7;

        const XYMRS = 1 << 3;

        const OVF = 1 << 1;

        const RUN = 1 << 0;
    }
}

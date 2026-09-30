use super::cordic::CORDIC;
use super::div::DIV;
use crate::register::Reserved;

/// MACP register block.
#[derive(Debug)]
#[repr(C)]
pub struct MACP {
    pub CORDIC0: CORDIC,
    pub CORDIC1: CORDIC,
    /// Reserved (0x20..=0xff).
    RESERVED0: Reserved<[u32; 56]>,
    pub DIV0: DIV,
    pub DIV1: DIV,
}

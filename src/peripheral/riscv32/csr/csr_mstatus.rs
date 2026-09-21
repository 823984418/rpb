use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct mstatus: usize {
        const SD = 1 << 31;
        const FS_MASK = 0x3 << 13;
        const MPP_MASK = 0x3 << 11;
        const MPIE = 1 << 7;
        const MIE = 1 << 3;
    }
}

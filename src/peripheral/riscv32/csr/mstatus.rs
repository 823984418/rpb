use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct MSTATUS: usize {
        const SD = 1 << 31;

        const FS_DIRTY = 0b11 << 13;
        const FS_CLEAN = 0b10 << 13;
        const FS_INIRIAL = 0b01 << 13;
        const FS_OFF = 0b00 << 13;
        const FS_MASK = 0x3 << 13;

        const MPP_MASK = 0x3 << 11;
        const MPIE = 1 << 7;
        const MIE = 1 << 3;
    }
}

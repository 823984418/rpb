

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OffsetBus<B> {
    offset: isize,
    bus: B,
}

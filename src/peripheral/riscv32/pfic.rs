#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PFIC<B> {
    pub addr: usize,
    pub bus: B,
}

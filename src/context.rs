#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context<E, C> {
    pub event: E,
    pub container: C,
}

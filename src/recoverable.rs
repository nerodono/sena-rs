#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Describes recoverable error, can be used to express things like [`crate::handling::or::Or`]:
/// after an error, data can be extracted
pub struct Recoverable<T, E> {
    pub data: T,
    pub error: E,
}

impl<T, E> Recoverable<T, E> {
    pub const fn new(data: T, error: E) -> Self {
        Self { data, error }
    }
}

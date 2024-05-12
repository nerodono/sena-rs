/// This structure needed when handler needs some dependencies from the caller,
/// Database pool, for example.
///
/// It is needed here to unify the way one works with dependencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dependent<T, D = ()> {
    pub data: T,
    pub deps: D,
}

impl<T, D> Dependent<T, D> {
    pub const fn new(data: T, deps: D) -> Self {
        Self { data, deps }
    }

    pub fn into_pair(self) -> (T, D) {
        (self.data, self.deps)
    }
}

impl<T, D> From<Dependent<T, D>> for (T, D) {
    fn from(value: Dependent<T, D>) -> Self {
        value.into_pair()
    }
}

impl<T> From<T> for Dependent<T> {
    fn from(value: T) -> Self {
        Self::new(value, ())
    }
}

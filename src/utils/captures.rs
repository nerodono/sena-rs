/// Trait for so-called `Captures`-trick.
///
/// Need for this trick emerges from, for example, heavy usage of existential
/// types when specifying async function's return type.
///
/// More info about it: <https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html>
pub trait Captures<U: ?Sized> {}
impl<T, U: ?Sized> Captures<U> for T {}

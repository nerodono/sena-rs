pub mod captures;

/// # Trait aliases for functions
///
/// Useful, since currently rust doesn't allow you to access the return type of [`Fn`]
/// trait without mentioning it, for example
/// ```no_compile
/// impl<F: Fn() -> ???> Something for F {
///    type Output = Result<(), F::Output>; // this is not possible
/// }
/// ```
/// So, we need somehow to work-around this limitation of [`Fn`]-traits, it is done like this:
/// ```
/// pub trait Fn1<T>: Fn(T) -> Self::SOut {
///     type SOut;
/// }
/// impl<T, O, F: Fn(T) -> O> Fn1<T> for F {
///     type SOut = O;
/// }
/// ```
/// And instead of using [`Fn`] trait we will use `Fn1` definition
pub mod fn_n;

/// # [`std::future::Future`] that never resolves
pub mod never_resolve;

/// # Select-like future
pub mod poll_biased;

/// Specifies type of an error,
/// same as calling [`type_eq`] with expression and type `Result<_, $tp>`
#[macro_export]
macro_rules! err_eq {
    ($e:expr, $tp:ty $(,)?) => {
        $crate::type_eq!($e, Result<_, $tp>)
    };
}

/// Specifies type of the expression
#[macro_export]
macro_rules! type_eq {
    ($e:expr, $tp:ty $(,)?) => {
        ::core::convert::identity::<$tp>($e)
    };
}

pub use sena_macros::hlist;

pub mod pipeline;

/// # Handling
///
/// All things related to composable handling: [`handling::handler::Handler`] definition and default recipes,
/// like [`handling::pipe::Pipe`], [`handling::seq::Seq`] and so on.
///
/// To start, refer to [`handling::handler`] and specifially to [`handling::handler::Handler`].
pub mod handling;

/// # Utilities for handling
pub mod utils;

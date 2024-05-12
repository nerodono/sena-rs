#[macro_export]
macro_rules! err_eq {
    ($e:expr, $tp:ty $(,)?) => {
        $crate::type_eq!($e, Result<_, $tp>)
    };
}

#[macro_export]
macro_rules! type_eq {
    ($e:expr, $tp:ty $(,)?) => {{
        let result: $tp = $e;
        result
    }};
}

/// # Module for CSP communication
///
/// This needs to offload
pub mod csp;

/// # Handling
///
/// All things related to handling: [`handling::handler::Handler`] definition and default recipes,
/// like [`handling::pipe::Pipe`], [`handling::seq::Seq`] and so on
pub mod handling;
pub mod recoverable;

pub mod dependent;
pub mod utils;

use std::future::Future;

use crate::utils::{captures::Captures, never_resolve::NeverResolve};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Shutdown token that never resolves and consumes every shutdown request
pub struct NoShutdown;

impl<E: Send> ShutdownRx<E> for NoShutdown {
    fn wait_shutdown(&mut self) -> impl Future<Output = Result<(), E>> + Send + Captures<&'_ Self> {
        NeverResolve::new()
    }
}

impl<E> ShutdownTx<E> for NoShutdown {
    fn shutdown(self) -> impl Future<Output = Result<(), E>> + Send {
        async move { Ok(()) }
    }
}

pub trait ShutdownTx<E>: Send + Clone {
    fn shutdown(self) -> impl Future<Output = Result<(), E>> + Send;
}

pub trait ShutdownRx<E>: Send {
    fn wait_shutdown(
        &mut self,
    ) -> impl Future<Output = Result<(), E>> + Unpin + Send + Captures<&'_ Self>;
}

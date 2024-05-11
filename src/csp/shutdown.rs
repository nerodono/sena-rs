use std::future::Future;

use crate::utils::captures::Captures;

pub trait ShutdownTx<E>: Send + Clone {
    fn shutdown(self) -> impl Future<Output = Result<(), E>> + Send;
}

pub trait ShutdownRx<E>: Send {
    fn wait_shutdown(&self) -> impl Future<Output = Result<(), E>> + Send + Captures<&'_ Self>;
}

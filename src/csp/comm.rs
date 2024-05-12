use std::future::Future;

use super::message::Message;
use crate::utils::captures::Captures;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// [`OutputTx`] that just drops any input data
pub struct VoidTx;

impl<T, E> OutputTx<T, E> for VoidTx {
    fn send(&self, data: T) -> impl Future<Output = Result<(), E>> + Captures<&'_ Self> {
        _ = data;
        async move { Ok(()) }
    }
}

pub trait Responder<R, E>: Send {
    fn respond_with(self, with: R) -> impl Future<Output = Result<(), E>> + Send;
}

pub trait OutputTx<T, E>: Send + Clone {
    fn send(&self, data: T) -> impl Future<Output = Result<(), E>> + Send + Captures<&'_ Self>;
}

pub trait TxChan<T, E, R>: Send + Clone {
    fn send<'a>(
        &'a self,
        data: T,
    ) -> impl Future<Output = Result<Option<R>, E>> + Send + Captures<&'a Self>;

    fn send_nowait<'a>(
        &'a self,
        data: T,
    ) -> impl Future<Output = Result<(), E>> + Send + Captures<&'a Self>;
}

pub trait RxChan<T, E, R>: Send {
    type Responder: Responder<R, E>;

    fn recv(
        &mut self,
    ) -> impl Future<Output = Result<Message<T, Self::Responder>, E>>
           + Unpin
           + Send
           + Captures<&'_ mut Self>;
}

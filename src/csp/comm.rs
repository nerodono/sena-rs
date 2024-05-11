use std::future::Future;

use crate::utils::captures::Captures;

pub trait Responder<R, E> {
    fn respond_with(self, with: R) -> impl Future<Output = Result<(), E>> + Send;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message<T, R> {
    pub data: T,
    pub responder: Option<R>,
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
    ) -> impl Future<Output = Result<Message<T, Self::Responder>, E>> + Send + Captures<&'_ mut Self>;
}

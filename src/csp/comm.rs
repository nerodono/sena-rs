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

impl<T, R> Message<T, R> {
    /// Creates message which asks for response
    pub const fn new(data: T, responder: R) -> Self {
        Self {
            data,
            responder: Some(responder),
        }
    }

    /// Creates message without need for responding
    pub const fn without_responder(data: T) -> Self {
        Self {
            data,
            responder: None,
        }
    }

    pub const fn needs_response(&self) -> bool {
        self.responder.is_some()
    }
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

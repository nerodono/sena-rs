use std::future::Future;
use std::mem;

use tokio::sync::{mpsc, oneshot};

use crate::{
    csp::{
        comm::{OutputTx, Responder, RxChan, TxChan},
        message::Message,
    },
    utils::captures::Captures,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecvError;

struct Pkt<T, R, E> {
    pub data: T,
    pub respond_tx: Option<oneshot::Sender<Option<Result<R, E>>>>,
}

pub struct OneshotResponder<R, E> {
    tx: Option<oneshot::Sender<Option<Result<R, E>>>>,
}
impl<R: Send, E: Send> Responder<R, E> for OneshotResponder<R, E> {
    fn respond_with(mut self, with: Result<R, E>) -> impl Future<Output = Result<(), E>> + Send {
        if let Some(chan) = mem::take(&mut self.tx) {
            _ = chan.send(Some(with));
        }

        async move { Ok(()) }
    }
}

impl<R, E> Drop for OneshotResponder<R, E> {
    fn drop(&mut self) {
        // If receiver didn't send response through the [`Responder::respond_with`],
        // value will be `Some`
        if let Some(chan) = mem::take(&mut self.tx) {
            _ = chan.send(None);
        }
    }
}

pub struct BoundedTx<T, R, E>(mpsc::Sender<Pkt<T, R, E>>);
pub struct BoundedRx<T, R, E>(mpsc::Receiver<Pkt<T, R, E>>);

pub fn bounded<T, R, E>(cap: usize) -> (BoundedTx<T, R, E>, BoundedRx<T, R, E>) {
    let (tx, rx) = mpsc::channel(cap);
    (BoundedTx(tx), BoundedRx(rx))
}

impl<T, R, E> Clone for BoundedTx<T, R, E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, E> OutputTx<T, E> for mpsc::UnboundedSender<Result<T, E>>
where
    T: Send,
    E: From<mpsc::error::SendError<Result<T, E>>> + Send,
{
    fn send(&self, data: Result<T, E>) -> impl Future<Output = Result<(), E>> + Captures<&'_ Self> {
        async move {
            mpsc::UnboundedSender::send(self, data)?;
            Ok(())
        }
    }
}
impl<T, E> OutputTx<T, E> for mpsc::Sender<Result<T, E>>
where
    T: Send,
    E: From<mpsc::error::SendError<Result<T, E>>> + Send,
{
    fn send(&self, data: Result<T, E>) -> impl Future<Output = Result<(), E>> + Captures<&'_ Self> {
        async move {
            mpsc::Sender::send(self, data).await?;
            Ok(())
        }
    }
}

impl<T, E, R> RxChan<T, E, R> for BoundedRx<T, R, E>
where
    T: Send,
    R: Send,
    E: From<RecvError> + Send,
{
    type Responder = OneshotResponder<R, E>;

    fn recv(
        &mut self,
    ) -> impl Future<Output = Result<Message<T, Self::Responder>, E>> + Send + Captures<&'_ mut Self>
    {
        Box::pin(async move {
            let Some(pkt) = self.0.recv().await else {
                return Err(RecvError.into());
            };
            let message = if let Some(respond_tx) = pkt.respond_tx {
                Message::new(
                    pkt.data,
                    OneshotResponder {
                        tx: Some(respond_tx),
                    },
                )
            } else {
                Message::without_responder(pkt.data)
            };
            Ok(message)
        })
    }
}

impl<T, E, R> TxChan<T, E, R> for BoundedTx<T, R, E>
where
    T: Send,
    R: Send,
    E: From<mpsc::error::SendError<T>> + From<oneshot::error::RecvError> + Send,
{
    fn send<'a>(
        &'a self,
        data: T,
    ) -> impl Future<Output = Result<Option<Result<R, E>>, E>> + Send + Captures<&'a Self> {
        let (tx, rx) = oneshot::channel();
        async move {
            let result = self
                .0
                .send(Pkt {
                    data,
                    respond_tx: Some(tx),
                })
                .await;
            if let Err(e) = result {
                return Err(mpsc::error::SendError(e.0.data).into());
            }

            Ok(rx.await?)
        }
    }

    fn send_nowait<'a>(
        &'a self,
        data: T,
    ) -> impl Future<Output = Result<(), E>> + Send + Captures<&'a Self> {
        async move {
            if let Err(e) = self
                .0
                .send(Pkt {
                    data,
                    respond_tx: None,
                })
                .await
            {
                return Err(mpsc::error::SendError(e.0.data).into());
            }

            Ok(())
        }
    }
}

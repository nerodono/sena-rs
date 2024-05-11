use std::future::Future;

use tokio::sync::broadcast;

use crate::{
    csp::shutdown::{ShutdownRx, ShutdownTx},
    utils::captures::Captures,
};

#[derive(Debug, Clone, Copy)]
pub struct SendError;

#[derive(Debug, Clone, Copy)]
pub struct ShutdownError;

#[derive(Debug, Clone)]
pub struct ShutTx(pub broadcast::Sender<()>);

#[derive(Debug)]
pub struct ShutRx(pub broadcast::Receiver<()>);

impl Clone for ShutRx {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

impl<E: From<ShutdownError>> ShutdownRx<E> for ShutRx {
    fn wait_shutdown(
        &mut self,
    ) -> impl Future<Output = Result<(), E>> + Unpin + Send + Captures<&'_ Self> {
        let fut = self.0.recv();
        Box::pin(async move {
            fut.await.map_err(|_| ShutdownError)?;
            Ok(())
        })
    }
}

impl<E: From<SendError>> ShutdownTx<E> for ShutTx {
    fn shutdown(self) -> impl Future<Output = Result<(), E>> + Send {
        async move {
            self.0.send(()).map_err(|_| SendError)?;
            Ok(())
        }
    }
}

pub fn make_pair() -> (ShutTx, ShutRx) {
    let (tx, rx) = broadcast::channel(1);
    (ShutTx(tx), ShutRx(rx))
}

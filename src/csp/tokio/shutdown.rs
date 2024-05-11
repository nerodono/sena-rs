use std::{future::Future, sync::Arc};

use tokio::sync::Notify;

use crate::{
    csp::shutdown::{ShutdownRx, ShutdownTx},
    utils::captures::Captures,
};

#[derive(Debug, Clone)]
pub struct Tx(pub Arc<Notify>);

#[derive(Debug, Clone)]
pub struct Rx(pub Arc<Notify>);

impl<E> ShutdownTx<E> for Tx {
    fn shutdown(self) -> impl Future<Output = Result<(), E>> + Send {
        self.0.notify_waiters();
        async move { Ok(()) }
    }
}

impl<E> ShutdownRx<E> for Rx {
    fn wait_shutdown(&self) -> impl Future<Output = Result<(), E>> + Send + Captures<&'_ Self> {
        async move {
            self.0.notified().await;
            Ok(())
        }
    }
}

pub fn make_pair() -> (Tx, Rx) {
    let notify = Arc::new(Notify::new());
    (Tx(Arc::clone(&notify)), Rx(notify))
}

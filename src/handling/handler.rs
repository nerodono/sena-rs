use std::{future::Future, sync::Arc};

use crate::{
    csp::{
        comm::{OutputTx, RxChan},
        shutdown::ShutdownRx,
    },
    utils::captures::Captures,
};

use super::{
    erased::{ErasedHandler, TypeErasedHandler},
    map::{Fn1, Map},
    pipe::Pipe,
    seq::{Seq, SeqHandler},
    server::{ServeOptions, Server},
};

/// Core of this library
///
/// - It is designed with the minimalism in mind: you can avoid using any advanced features
///   If you don't need them, so request is arbitrary
/// - [`Handler`] must return generic error, if you want throw specific error, consider restricting
///   implementation by adding [`From`] bound, for example: `E: From<io::Error>`.
///   This ensures that your handler is flexible in terms of context it can be used
pub trait Handler<T, E>: Send + Sync {
    type Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)>;

    /// Run this handler after specified, specified handler must
    /// implement [`SeqHandler`] to create something meaningful
    fn after<C>(self, current: C) -> Seq<C, Self>
    where
        C: SeqHandler<T, E, Self>,
        Self: Sized,
    {
        Seq::new(current, self)
    }

    fn serve<X, OTx, SRx>(
        self,
        rx: X,
        options: ServeOptions<OTx, SRx>,
    ) -> impl Future<Output = Result<(), E>> + Send
    where
        T: Send,
        E: Send,
        Self::Output: Send,

        Self: Sized,
        X: RxChan<T, E, Self::Output>,
        OTx: OutputTx<Self::Output, E>,
        SRx: ShutdownRx<E>,
    {
        async move {
            let mut server = self.into_server(rx);
            server.serve(options).await
        }
    }

    /// Creates server from that handler, for more, see docs for [`Server`] handler
    fn into_server<X>(self, rx: X) -> Server<Self, X>
    where
        Self: Sized,
        X: RxChan<T, E, Self::Output>,
    {
        Server { handler: self, rx }
    }

    /// Changes input of the handler into another type, to do that - it
    /// accepts function that maps that type to input type of the current handler
    fn map<K, F>(self, f: F) -> Map<F, Self>
    where
        Self: Sized,
        F: Fn1<K, E, SOut = T>,
    {
        Map { f, handler: self }
    }

    /// See docs for [`Pipe`] module
    fn pipe<Dst>(self, dst: Dst) -> Pipe<Self, Dst>
    where
        Self: Sized,
        Dst: Handler<Self::Output, E>,
    {
        Pipe::new(self, dst)
    }

    /// Wraps handler in [`Arc`]
    fn share(self) -> Arc<Self>
    where
        Self: Sized,
    {
        Arc::new(self)
    }

    /// Erase type of handler, this requires `T` to be Send + 'static,
    /// I am not sure if this is absolute requirement or it (`'static`) can be lifted, PRs are welcome
    fn erase(self) -> ErasedHandler<T, Self::Output, E>
    where
        T: Send + 'static,
        Self: Sized + 'static,
    {
        Box::new(TypeErasedHandler(self))
    }
}

impl<T, E, O, F, Fut> Handler<T, E> for F
where
    F: Send + Sync + Fn(T) -> Fut,
    Fut: Future<Output = Result<O, E>> + Send,
{
    type Output = O;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        self(request)
    }
}

impl<T, E, H> Handler<T, E> for Arc<H>
where
    H: Handler<T, E>,
{
    type Output = H::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        Arc::as_ref(self).handle(request)
    }
}

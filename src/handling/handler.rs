use std::{future::Future, sync::Arc};

use crate::{
    csp::comm::RxChan,
    recoverable::Recoverable,
    utils::{
        captures::Captures,
        fn_n::{Fn1Result, Fn1ResultAsync},
    },
};

with_rt! {
    use crate::{
        csp::{comm::OutputTx, shutdown::ShutdownRx},
        handling::server::ServeOptions,
    };
}

use super::{
    erased::{ErasedHandler, TypeErasedHandler},
    map::{Map, MapAsync},
    or::Or,
    pipe::Pipe,
    provide::Provide,
    seq::{Seq, SeqHandler},
    server::Server,
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

    /// Logical or for handlers, refer to [`Or`] for documentation
    fn or<R, EE>(self, rhs: R) -> Or<Self, R>
    where
        Self: Sized + Handler<T, Recoverable<T, EE>>,
        R: Handler<T, EE>,
    {
        Or::new(self, rhs)
    }

    #[cfg(feature = "tokio")]
    /// Launches [`Server`], same as calling [`Handling::into_server`]
    /// and [`Server::serve`] method on it
    fn serve<X, OTx, SRx>(
        self,
        rx: X,
        options: ServeOptions<OTx, SRx>,
    ) -> impl Future<Output = Result<(), E>> + Send
    where
        T: Send + 'static,
        E: Send + 'static,
        Self::Output: Send,

        Self: Sized + Clone + 'static,
        X: RxChan<T, E, Self::Output>,
        X::Responder: Send + 'static,
        OTx: OutputTx<Self::Output, E> + 'static,
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

    /// Special case of [`Handler::map_async`],
    /// refer to [`Provide`] documentation for more.
    fn provide<Env, F>(self, env: Env, f: F) -> Provide<Env, F, Self>
    where
        Self: Sized,
    {
        Provide {
            env,
            f,
            handler: self,
        }
    }

    /// Same as [`Handler::map`], but provided function is async
    fn map_async<K, F>(self, f: F) -> MapAsync<F, Self>
    where
        Self: Sized,
        F: Fn1ResultAsync<K, E, Ok = T>,
    {
        MapAsync { handler: self, f }
    }

    /// Changes input of the handler into another type, to do that - it
    /// accepts function that maps that type to input type of the current handler
    fn map<K, F>(self, f: F) -> Map<F, Self>
    where
        Self: Sized,
        F: Fn1Result<K, E, SOut = T>,
    {
        Map { f, handler: self }
    }

    /// Pipes output from one handler to `Dst`, refer to docs of [`Pipe`] for more.
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

use std::{future::Future, pin::Pin};

use crate::utils::captures::Captures;

use super::handler::Handler;

pub type BoxFuture<'a, R> = Pin<Box<dyn Future<Output = R> + Send + 'a>>;

pub type DynErasedHandler<T, O, E> = dyn DynHandler<T, E, Output = O>;
pub type ErasedHandler<T, O, E> = Box<DynErasedHandler<T, O, E>>;

/// Object-safe version of the [`Handler`] trait, useful for type erasure.
///
/// It is quite hard to track your handler's type precisely, it even becomes a burden to carry
/// necessary static-type information through your code, and sometimes not even possible due to
/// specific needs, here's solution. In definition, it is same as [`Handler`] trait, but due to
/// object-safety it additionally boxes returned future;
pub trait DynHandler<T, E>: Send + Sync {
    type Output;

    fn handle(&self, request: T) -> BoxFuture<'_, Result<Self::Output, E>>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Handler with type that can be type-erased by boxing it or wrapping
/// in [`std::sync::Arc`]/[`std::rc::Rc`] or whatever.
pub struct TypeErasedHandler<H>(pub H);

impl<T, E, H: Handler<T, E>> Handler<T, E> for TypeErasedHandler<H> {
    type Output = H::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        self.0.handle(request)
    }
}
impl<T, E, H> DynHandler<T, E> for TypeErasedHandler<H>
where
    T: Send + 'static,
    H: Handler<T, E>,
{
    type Output = H::Output;

    fn handle(&self, request: T) -> BoxFuture<'_, Result<Self::Output, E>> {
        Box::pin(async move { self.0.handle(request).await })
    }
}

use std::future::Future;

use crate::utils::captures::Captures;

use super::{seq::SeqHandler, Handler};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Inner<F, H> {
    pub f: F,
    pub handler: H,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inspect<F, H>(Inner<F, H>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeqInspect<F>(pub F);

impl<F, H> Inspect<F, H> {
    pub const fn new(f: F, handler: H) -> Self {
        Self(Inner { f, handler })
    }
}

impl<T, E, F, H> SeqHandler<T, E, H> for SeqInspect<F>
where
    F: Send + Sync + Fn(&T),
    H: Handler<T, E>,
{
    type Output = H::Output;

    fn process<'a, 'h>(
        &'a self,
        request: T,
        next: &'h H,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, &'h H)> {
        (self.0)(&request);
        next.handle(request)
    }
}

impl<T, E, F, H> Handler<T, E> for Inspect<F, H>
where
    F: Send + Sync + Fn(&T),
    H: Handler<T, E>,
{
    type Output = H::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        (self.0.f)(&request);
        self.0.handler.handle(request)
    }
}

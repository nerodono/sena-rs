use std::future::Future;

use crate::utils::captures::Captures;

use super::handler::Handler;

/// [`Handler`] aware of continuation. You can think of it
/// as middleware trait.
pub trait SeqHandler<T, E, H>: Send + Sync {
    type Output;

    fn process<'a, 'h>(
        &'a self,
        request: T,
        next: &'h H,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, &'h H)>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Handler that processes `next` [`Handler`] after `current` [`SeqHandler`] run.
pub struct Seq<C, N> {
    pub current: C,
    pub next: N,
}

impl<C, N> Seq<C, N> {
    pub const fn new(current: C, next: N) -> Self {
        Self { current, next }
    }
}

impl<T, E, C, N> Handler<T, E> for Seq<C, N>
where
    C: SeqHandler<T, E, N>,
    N: Handler<T, E>,
{
    type Output = C::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        self.current.process(request, &self.next)
    }
}

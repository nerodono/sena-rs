use std::future::Future;

use crate::utils::captures::Captures;

use super::handler::Handler;

/// Forward output from [`from`] to [`to`],
/// or "pipe" output from one handler to another
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pipe<Src, Dst> {
    pub from: Src,
    pub to: Dst,
}

impl<Src, Dst, T, E> Handler<T, E> for Pipe<Src, Dst>
where
    T: Send,
    Src: Handler<T, E>,
    Dst: Handler<Src::Output, E>,
{
    type Output = Dst::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        async move {
            let left = self.from.handle(request).await?;
            self.to.handle(left).await
        }
    }
}

impl<Src, Dst> Pipe<Src, Dst> {
    pub const fn new(from: Src, to: Dst) -> Self {
        Self { from, to }
    }
}

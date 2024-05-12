use std::future::Future;

use crate::{recoverable::Recoverable, utils::captures::Captures};

use super::Handler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Or<L, R> {
    pub left: L,
    pub right: R,
}

impl<T, L, R, E> Handler<T, E> for Or<L, R>
where
    T: Send,
    L: Handler<T, Recoverable<T, E>>,
    R: Handler<T, E, Output = L::Output>,

    L::Output: Send,
    E: Send,
{
    type Output = L::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        async move {
            match self.left.handle(request).await {
                Ok(r) => Ok(r),
                Err(Recoverable { data, error: _ }) => self.right.handle(data).await,
            }
        }
    }
}

impl<L, R> Or<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

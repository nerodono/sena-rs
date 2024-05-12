use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use either::Either;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Polls two futures and returns result of the first completed,
/// drops the second. Both are required to be [`Unpin`]
pub struct PollBiased<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Future for PollBiased<L, R>
where
    L: Future + Unpin,
    R: Future + Unpin,
{
    type Output = Either<L::Output, R::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.left).poll(cx) {
            Poll::Ready(left) => Poll::Ready(Either::Left(left)),
            Poll::Pending => match Pin::new(&mut self.right).poll(cx) {
                Poll::Ready(right) => Poll::Ready(Either::Right(right)),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

impl<L, R> PollBiased<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

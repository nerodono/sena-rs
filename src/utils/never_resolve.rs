use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

// [`Future`] that never resolves
pub struct NeverResolve<T> {
    _phantom: PhantomData<T>,
}

impl<T> NeverResolve<T> {
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Future for NeverResolve<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl<T> Unpin for NeverResolve<T> {}
impl<T> Clone for NeverResolve<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

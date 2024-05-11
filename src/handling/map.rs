use std::future::Future;

use crate::utils::captures::Captures;

use super::handler::Handler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Map<F, H> {
    pub handler: H,
    pub f: F,
}

pub trait Fn1<T, E>: Fn(T) -> Result<Self::SOut, E> {
    type SOut;
}
impl<T, O, E, F: Fn(T) -> Result<O, E>> Fn1<T, E> for F {
    type SOut = O;
}

impl<F, H, T, E> Handler<T, E> for Map<F, H>
where
    T: Send,
    F: Send + Sync + Fn1<T, E>,
    H: Handler<F::SOut, E>,
{
    type Output = H::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        async move {
            let result = (self.f)(request)?;
            self.handler.handle(result).await
        }
    }
}

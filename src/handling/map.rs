use std::future::Future;

use crate::utils::{
    captures::Captures,
    fn_n::{Fn1Result, Fn1ResultAsync},
};

use super::handler::Handler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Map<F, H> {
    pub handler: H,
    pub f: F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapAsync<F, H> {
    pub handler: H,
    pub f: F,
}

impl<F, H, T, E> Handler<T, E> for MapAsync<F, H>
where
    T: Send,
    F: Send + Sync + Fn1ResultAsync<T, E>,
    H: Handler<F::Ok, E>,
{
    type Output = H::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        async move {
            let result = (self.f)(request).await?;
            self.handler.handle(result).await
        }
    }
}

impl<F, H, T, E> Handler<T, E> for Map<F, H>
where
    T: Send,
    F: Send + Sync + Fn1Result<T, E>,
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

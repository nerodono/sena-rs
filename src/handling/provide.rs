use std::future::Future;

use super::handler::Handler;
use crate::utils::captures::Captures;

pub trait ContFn<T, E>: Fn(T) -> Self::Future {
    type Ok;
    type Future: Future<Output = Result<Self::Ok, E>>;
}
impl<T, E, O, F, Fut> ContFn<T, E> for F
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = Result<O, E>>,
{
    type Ok = O;
    type Future = Fut;
}

pub trait SeqHandler<T, E, H: Handler<Self::Requires, E>>: Send + Sync {
    type Requires;
    type Output;

    fn process<'a, F: ContFn<Self::Requires, E, Ok = H::Output>>(
        &'a self,
        request: T,
        next: F,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, F)>;
}

pub struct ConvertToU64;
impl<E, H> SeqHandler<i32, E, H> for ConvertToU64
where
    H: Handler<u64, E>,
{
    type Requires = u64;
    type Output = H::Output;

    fn process<'a, F: ContFn<Self::Requires, E, Ok = H::Output>>(
        &'a self,
        request: i32,
        next: F,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, F)> {
        async move { todo!() }
    }
}

pub struct Seq<C, N> {
    pub current: C,
    pub next: N,
}

impl<C, N, T, E> Handler<T, E> for Seq<C, N>
where
    T: Send,
    C: SeqHandler<T, E, N>,
    N: Handler<C::Requires, E>,
{
    type Output = C::Output;

    fn handle<'a>(
        &'a self,
        request: T,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, T)> {
        async move { self.current.process(request, |m| self.next.handle(m)).await }
    }
}

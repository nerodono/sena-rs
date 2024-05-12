use std::future::Future;

pub trait Fn1Result<T, E>: Fn(T) -> Result<Self::SOut, E> {
    type SOut;
}
pub trait Fn1<T>: Fn(T) -> Self::SOut {
    type SOut;
}
pub trait Fn1ResultAsync<T, E>: Fn(T) -> Self::Future {
    type Ok;
    type Future: Future<Output = Result<Self::Ok, E>> + Send;
}
pub trait Fn2ResultAsync<A1, A2, E>: Fn(A1, A2) -> Self::Future {
    type Ok;
    type Future: Future<Output = Result<Self::Ok, E>> + Send;
}

impl<A1, A2, O, E, F, Fut> Fn2ResultAsync<A1, A2, E> for F
where
    F: Fn(A1, A2) -> Fut,
    Fut: Future<Output = Result<O, E>> + Send,
{
    type Ok = O;
    type Future = Fut;
}

impl<T, O, E, F, Fut> Fn1ResultAsync<T, E> for F
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = Result<O, E>> + Send,
{
    type Ok = O;
    type Future = Fut;
}
impl<T, O, F> Fn1<T> for F
where
    F: Fn(T) -> O,
{
    type SOut = O;
}
impl<T, O, E, F: Fn(T) -> Result<O, E>> Fn1Result<T, E> for F {
    type SOut = O;
}

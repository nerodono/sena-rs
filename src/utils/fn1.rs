pub trait Fn1Result<T, E>: Fn(T) -> Result<Self::SOut, E> {
    type SOut;
}
pub trait Fn1<T>: Fn(T) -> Self::SOut {
    type SOut;
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

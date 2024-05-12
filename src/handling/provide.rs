use std::future::Future;

use crate::{
    dependent::Dependent,
    utils::{captures::Captures, fn_n::Fn2ResultAsync},
};

use super::Handler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Special case of `super::map::MapAsync`, with a different interface.
/// It can be more ergonomic to provide environment as a function argument, not by capturing
pub struct Provide<Env, F, H> {
    pub env: Env,
    pub f: F,
    pub handler: H,
}

impl<Env, E, F, H, Args, Deps> Handler<Args, E> for Provide<Env, F, H>
where
    Args: Send,
    Env: Send + Sync + Clone,
    F: Send + Sync + Fn2ResultAsync<Env, Args, E, Ok = Dependent<Args, Deps>>,
    H: Handler<Dependent<Args, Deps>, E>,
{
    type Output = H::Output;

    fn handle<'a>(
        &'a self,
        request: Args,
    ) -> impl Future<Output = Result<Self::Output, E>> + Send + Captures<(&'a Self, Args)> {
        let env = self.env.clone();
        async move {
            let dependent = (self.f)(env, request).await?;
            self.handler.handle(dependent).await
        }
    }
}

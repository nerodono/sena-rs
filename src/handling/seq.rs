use std::future::Future;

use crate::context::Context;

use super::handler::Handler;

pub trait SeqHandler<Event, C>: Send {
    fn handle<'a, H>(
        &'a self,
        ctx: Context<Event, C>,
        next: H,
    ) -> impl Future<Output = Result<H::Output, H::Error>> + Send + 'a
    where
        H: Handler<Event, C> + 'a;
}

#[derive(Debug, Clone)]
pub struct Seq<Curr, Next> {
    pub next: Next,
    pub current: Curr,
}

impl<Event, C, Curr, Next> Handler<Event, C> for Seq<Curr, Next>
where
    for<'a> &'a Next: Handler<Event, C>,
    Curr: SeqHandler<Event, C>,
{
    type Output = <for<'a> &'a Next as Handler<Event, C>>::Output;
    type Error = <for<'a> &'a Next as Handler<Event, C>>::Error;

    fn handle(
        &self,
        ctx: Context<Event, C>,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + '_ {
        self.current.handle(ctx, &self.next)
    }
}

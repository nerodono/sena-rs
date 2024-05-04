use std::future::Future;

use crate::context::Context;

pub trait Handler<Event, C>: Send {
    type Output;
    type Error;

    fn handle(
        &self,
        ctx: Context<Event, C>,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + '_;
}

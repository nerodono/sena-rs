use either::Either;

use crate::{
    csp::{
        comm::{OutputTx, Responder, RxChan, VoidTx},
        message::Message,
        shutdown::{NoShutdown, ShutdownRx},
    },
    utils::poll_biased::PollBiased,
};

use super::handler::Handler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Server<H, X> {
    pub handler: H,
    pub rx: X,
}

pub struct ServeOptions<OTx, SRx> {
    /// Token that can be used to gracefully shutdown server
    pub shutdown_rx: SRx,

    /// Channel where all output will be thrown, outputs will be sent here
    /// only if output is not forwarded as a response (client doesn't wants response)
    pub output_tx: OTx,
}

impl Default for ServeOptions<VoidTx, NoShutdown> {
    fn default() -> Self {
        Self {
            shutdown_rx: NoShutdown,
            output_tx: VoidTx,
        }
    }
}

impl<H, X> Server<H, X> {
    pub async fn serve<T, E, OTx, SRx>(
        &mut self,
        mut options: ServeOptions<OTx, SRx>,
    ) -> Result<(), E>
    where
        OTx: OutputTx<H::Output, E>,
        H: Handler<T, E>,
        X: RxChan<T, E, H::Output>,
        SRx: ShutdownRx<E>,
    {
        loop {
            let result = PollBiased::new(options.shutdown_rx.wait_shutdown(), self.rx.recv()).await;
            match result {
                Either::Left(_) => return Ok(()),
                Either::Right(message) => {
                    let message = message?;
                    if let Some(output) = self.handle_message(message).await? {
                        options.output_tx.send(output).await?;
                    }
                }
            }
        }
    }

    pub async fn handle_message<T, E>(
        &mut self,
        message: Message<T, X::Responder>,
    ) -> Result<Option<H::Output>, E>
    where
        H: Handler<T, E>,
        X: RxChan<T, E, H::Output>,
    {
        let output = self.handler.handle(message.data).await?;

        if let Some(responder) = message.responder {
            responder.respond_with(output).await?;
            Ok(None)
        } else {
            Ok(Some(output))
        }
    }
}

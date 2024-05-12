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

impl<SRx> ServeOptions<VoidTx, SRx> {
    pub const fn with_shutdown_token(shutdown_rx: SRx) -> Self {
        Self {
            shutdown_rx,
            output_tx: VoidTx,
        }
    }
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
    #[cfg(feature = "tokio")]
    pub async fn serve<T, E, OTx, SRx>(
        &mut self,
        mut options: ServeOptions<OTx, SRx>,
    ) -> Result<(), E>
    where
        T: Send + 'static,
        E: Send + 'static,
        OTx: OutputTx<H::Output, E> + 'static,
        H: Handler<T, E> + Clone + 'static,
        X: RxChan<T, E, H::Output>,
        X::Responder: Send + 'static,
        SRx: ShutdownRx<E>,
    {
        loop {
            let result = PollBiased::new(options.shutdown_rx.wait_shutdown(), self.rx.recv()).await;
            match result {
                Either::Left(_) => return Ok(()),
                Either::Right(message) => {
                    let message = message?;

                    let out_tx = options.output_tx.clone();
                    let handler = self.handler.clone();

                    tokio::spawn(async move {
                        let output = handler.handle(message.data).await?;
                        if let Some(responder) = message.responder {
                            responder.respond_with(output).await?;
                        } else {
                            out_tx.send(output).await?;
                        }

                        Ok::<_, E>(())
                    });
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

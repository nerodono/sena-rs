use derive_more::From;

use sena::{
    csp::{
        comm::TxChan,
        shutdown::ShutdownTx,
        tokio::{
            comm::{self, RecvError},
            shutdown::{self, SendError, ShutdownError},
        },
    },
    handling::{handler::Handler, server::ServeOptions},
};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, From)]
pub enum Error {
    Shutdown(ShutdownError),
    Recv(RecvError),
    ShutSend(SendError),

    ResponseRecv(oneshot::error::RecvError),
    Send(mpsc::error::SendError<i32>),
}

fn increment<E>() -> impl Handler<i32, E, Output = i32> {
    |req: i32| async move { Ok(req + 1) }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let (shut_tx, shut_rx) = shutdown::make_pair();
    let (inc_tx, inc_rx) = comm::bounded::<_, i32>(1);

    let join_handle = tokio::spawn(
        increment::<Error>()
            .share()
            .serve(inc_rx, ServeOptions::with_shutdown_token(shut_rx)),
    );
    let res = sena::err_eq!(inc_tx.send(10).await, Error)?;

    if let Some(response) = res {
        println!("Got response: {response}");
    } else {
        println!("Got no response");
    }
    println!("Waiting for service to shutdown...");
    sena::err_eq!(shut_tx.shutdown().await, Error)?;
    join_handle.await.unwrap().unwrap();

    println!("Done!");

    Ok(())
}

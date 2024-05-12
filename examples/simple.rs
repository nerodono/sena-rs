use sena::handling::handler::Handler;

fn increment<E>() -> impl Handler<i32, E, Output = i32> {
    |req: i32| async move { Ok(req + 1) }
}

fn decrement<E>() -> impl Handler<i32, E, Output = i32> {
    |req: i32| async move { Ok(req - 1) }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let inc = increment::<()>();
    let dec = decrement::<()>();

    let add2 = increment().pipe(increment::<()>());

    assert_eq!(inc.handle(10).await, Ok(11));
    assert_eq!(dec.handle(10).await, Ok(9));
    assert_eq!(add2.handle(10).await, Ok(12));
}

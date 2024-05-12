use std::num::ParseIntError;

use sena::handling::handler::Handler;

fn increment<E>() -> impl Handler<i32, E, Output = i32> {
    |req: i32| async move { Ok(req + 1) }
}

async fn add2<E>(req: i32) -> Result<i32, E> {
    Ok(req + 2)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let chain = increment::<ParseIntError>()
        .map_async(|req: i32| async move { Ok(req - 2) })
        .map_async(add2)
        .map(|src: String| src.trim().parse());
    let result = chain.handle(" 1337".to_owned()).await.unwrap();

    assert_eq!(result, 1338);
}

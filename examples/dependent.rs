use sena::{dependent::Dependent, handling::Handler};

#[derive(Debug, Clone)]
pub struct Delta(pub i32);

fn add<E>() -> impl Handler<Dependent<i32, Delta>, E, Output = i32> {
    |req: Dependent<i32, Delta>| async move { Ok(req.data + req.deps.0) }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let handler = add::<()>().provide(
        Delta(10),
        |env, req| async move { Ok(Dependent::new(req, env)) },
    );
    assert_eq!(handler.handle(10).await, Ok(20));
}

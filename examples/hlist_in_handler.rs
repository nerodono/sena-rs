use sena::handling::Handler;
use sena::pipeline::{ByRefPicker, HList};

#[derive(Debug, Clone, Copy)]
pub struct Delta(pub usize);

#[derive(HList)]
struct Args {
    pub value: usize,
    pub delta: Delta,

    pub name: String,
}

async fn add<E, T, I0, I1>(mut container: T) -> Result<T, E>
where
    T: ByRefPicker<usize, I0> + ByRefPicker<Delta, I1>,
{
    let delta: Delta = *container.pick_ref();
    let value: &mut usize = container.pick_mut();
    *value += delta.0;

    Ok(container)
}

async fn log<E, T, I0, I1>(container: T) -> Result<String, E>
where
    T: ByRefPicker<String, I0> + ByRefPicker<usize, I1>,
{
    let value: usize = *container.pick_ref();
    let name: &String = container.pick_ref();
    Ok(format!("{name} = {value}"))
}

fn get_handler() -> impl Handler<Args, (), Output = String> {
    add.pipe(log)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let handler = get_handler();
    let result = handler
        .handle(Args {
            value: 100,
            delta: Delta(10),
            name: "Nero".to_owned(),
        })
        .await
        .unwrap();

    dbg!(result);
}

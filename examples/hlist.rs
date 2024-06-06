use sena::pipeline::{ByRefPicker, HList};

#[derive(HList)]
struct Args {
    pub int: i32,
    pub string: String,
}

fn main() {
    let args = Args {
        int: 100,
        string: "Hello world".to_owned(),
    };

    let age: i32 = *args.pick_ref();
    let hello_world: &String = args.pick_ref();

    dbg!(age, hello_world);

    let with_bool = args.prepend(true);
    let verified: bool = *with_bool.pick_ref();

    dbg!(verified);
}

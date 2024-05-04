use crate::dep_picker::DepPicker;
use std::any::type_name;

#[derive(Debug, Clone, Copy)]
pub struct Stub;

#[derive(Debug, Clone, Copy)]
pub struct PanicStub;

impl<Dep, I> DepPicker<Dep, I> for PanicStub {
    fn pick_ref(&self) -> &Dep {
        panic!(
            "failed to pick shared reference of {}: Stub!",
            type_name::<Dep>()
        )
    }
}

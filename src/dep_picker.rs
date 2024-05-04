pub struct Z;
pub struct S<T>(pub [T; 0]);

pub trait DepPicker<Dep, I> {
    fn pick_ref(&self) -> &Dep;
}

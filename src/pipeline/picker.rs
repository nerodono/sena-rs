use super::hlist::HCons;

pub struct S<I>([I; 0]);
pub struct Z;

pub trait TypeNum {
    const INT: u64;

    type Pred: TypeNum;
    type Succ: TypeNum;
}
impl TypeNum for Z {
    const INT: u64 = 0;

    type Pred = Z;
    type Succ = S<Z>;
}
impl<I: TypeNum> TypeNum for S<I> {
    const INT: u64 = 1 + I::INT;

    type Pred = I;
    type Succ = S<I>;
}

pub trait ByRefPicker<T, I> {
    fn pick_ref(&self) -> &T;
    fn pick_mut(&mut self) -> &mut T;
}

impl<R, H, Tail, PrevIdx> ByRefPicker<R, S<PrevIdx>> for HCons<H, Tail>
where
    Tail: ByRefPicker<R, PrevIdx>,
{
    #[inline]
    fn pick_ref(&self) -> &R {
        self.1.pick_ref()
    }

    #[inline]
    fn pick_mut(&mut self) -> &mut R {
        self.1.pick_mut()
    }
}
impl<H, Tail> ByRefPicker<H, Z> for HCons<H, Tail> {
    #[inline]
    fn pick_ref(&self) -> &H {
        &self.0
    }

    #[inline]
    fn pick_mut(&mut self) -> &mut H {
        &mut self.0
    }
}

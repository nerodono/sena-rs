#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HCons<H, T>(pub H, pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HNil;

pub trait HList: Sized {
    fn prepend<T>(self, item: T) -> HCons<T, Self> {
        HCons(item, self)
    }
}

macro_rules! tup_impl {
    (@expand_ty) => { HNil };
    (@expand_ty $generic:ident $($tail:ident)*) => {
        HCons<$generic, tup_impl!(@expand_ty $($tail)*)>
    };

    (@expand_value) => { HNil };
    (@expand_value $generic:ident $($generics:ident)*) => {
        HCons($generic, tup_impl!(@expand_value $($generics)*))
    };

    (@index_ty) => { Z };
    (@index_ty $generic:ident $($generics:ident)*) => {
        S<tup_impl!(@index_ty $($generics)*)>
    };

    (
        $($generic:ident; $($generics:ident)*),*
        $(,)?
    ) => {
        $(
            #[automatically_derived]
            #[allow(non_snake_case)]
            impl<$generic, $($generics),*> From<($generic, $($generics),*)> for HCons<$generic, tup_impl!(@expand_ty $($generics)*)> {
                fn from(($generic, $($generics),*): ($generic, $($generics),*)) -> Self {
                    Self($generic, tup_impl!(@expand_value $($generics)*))
                }
            }
        )*
    };
}

impl From<()> for HNil {
    fn from((): ()) -> Self {
        Self
    }
}

tup_impl! {
    T0;,
    T0; T1,
    T0; T1 T2,
    T0; T1 T2 T3,
    T0; T1 T2 T3 T4,
    T0; T1 T2 T3 T4 T5,
    T0; T1 T2 T3 T4 T5 T6,
    T0; T1 T2 T3 T4 T5 T6 T7,
    T0; T1 T2 T3 T4 T5 T6 T7 T8,
    T0; T1 T2 T3 T4 T5 T6 T7 T8 T9,
    T0; T1 T2 T3 T4 T5 T6 T7 T8 T9 T10,
    T0; T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11,
    T0; T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12,
    T0; T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13,
    T0; T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14,
    T0; T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15,
}

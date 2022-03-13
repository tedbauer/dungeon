use std::any::{Any, TypeId};
use std::fmt::Debug;

pub trait ComponentTuple {
    fn type_ids() -> Vec<TypeId>;
}

pub trait Component: 'static {
    fn type_id(&self) -> TypeId;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
}

macro_rules! component_tuple_impls {
    ( $head:ident, $( $tail:ident, )* ) => {
        impl<$head, $( $tail ),*> ComponentTuple for ($head, $( $tail ),*)
        where
            $head: Component,
            $( $tail: Component ),*
        {

            fn type_ids() -> Vec<TypeId> {
                vec![ TypeId::of::<$head>(), $( TypeId::of::<$tail>() ),* ]
            }
        }

        component_tuple_impls!($( $tail, )*);
    };

    () => {};
}

component_tuple_impls!(A, B, C, D, E, F, G, H, I, J,);

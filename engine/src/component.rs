use std::any::TypeId;
use std::fmt::Debug;

pub trait ComponentTuple {
    fn new(components: &[Box<dyn Component>]) -> Self;
    fn type_ids() -> Vec<TypeId>;
}

pub trait Component: 'static + Debug {
    fn type_id(&self) -> TypeId;
}

macro_rules! component_tuple_impls {
    ( $head:ident, $( $tail:ident, )* ) => {
        impl<$head, $( $tail ),*> ComponentTuple for ($head, $( $tail ),*)
        where
            $head: Component,
            $( $tail: Component ),*
        {

            fn new(components: &[Box<dyn Component>]) -> Self {
                todo!()
            }

            fn type_ids() -> Vec<TypeId> {
                vec![ TypeId::of::<$head>(), $( TypeId::of::<$tail>() ),* ]
            }
        }

        component_tuple_impls!($( $tail, )*);
    };

    () => {};
}

component_tuple_impls!(A, B, C, D, E, F, G, H, I, J,);

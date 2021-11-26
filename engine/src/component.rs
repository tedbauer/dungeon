pub trait ComponentTuple {}

pub trait Component {
    fn type_id(&self) -> String;
}

macro_rules! component_tuple_impls {
    ( $head:ident, $( $tail:ident, )* ) => {
        impl<$head, $( $tail ),*> ComponentTuple for ($head, $( $tail ),*)
        where
            $head: Component,
            $( $tail: Component ),*
        {}

        component_tuple_impls!($( $tail, )*);
    };

    () => {};
}

component_tuple_impls!(A, B, C, D, E, F, G, H, I, J,);

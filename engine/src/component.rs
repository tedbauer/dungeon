trait ComponentTuple {}

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

struct View<C: ComponentTuple> {
    components: Option<C>
}

impl<C: ComponentTuple> Iterator for View<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

}

fn main() {
    for (position, _) in (View::<(Position, Player)>::create() { components: None }) {
        println!("{:?}", position);
    }
}

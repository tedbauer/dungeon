use crate::component::Component;
use crate::util::Point;
use std::any::Any;

#[proc_macro_derive(Component)]
pub struct Position {
    pub pos: Point,
}

#[proc_macro_derive(Component)]
pub struct Player {}

#[proc_macro_derive(Component)]
pub struct Render {}

impl Component for Position {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Component for Player {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Component for Render {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/*
macro_rules! register_components {
    ( $( $c:stmt )*  ) => {

        enum

    };
}

register_components! {
    pub struct Position {
        pub pos: Point,
    }

    pub struct Render {}
}
*/

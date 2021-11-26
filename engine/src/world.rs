use crate::component::ComponentTuple;
use crate::component::Component;

pub struct World {
	component_pools: Vec<Vec<Box<dyn Component>>>
}

impl World {
    pub fn new() -> Self {
			Self { component_pools: vec![] }
    }

    pub fn add_entity(&self, entity: Vec<Box<dyn Component>>) {
        todo!()
    }
}

/// An iterator containing every entity in the `World` that has the components in `C`.
/// 
/// Example usage:
/// ```
/// for (player, position) in View::<(Player, Position)>::new(&world) {
///   println!("player: {:?}", player);
///   println!("player position: {:?}", position);
/// }
/// ```
pub struct View<C: ComponentTuple> {
    components: C,
}

impl<C: ComponentTuple> Iterator for View<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<C: ComponentTuple> View<C> {
    pub fn new(world: &World) -> Self {
        todo!()
    }
}

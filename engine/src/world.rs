use crate::component::Component;
use crate::component::ComponentTuple;
use std::any::TypeId;
use std::collections::HashMap;

type ComponentID = usize;

/// Implementation sketch.
///
/// Entity ID -> Bitmask.
///   `Bitmask[i]` = 1 if entity contains component `i`.
///
/// for each entity in entityToComponentBits {
///   componentBits ^ entityBits == 0
/// }
///
/// add entity with components (c1, ..., cn).
///
/// get all entities with components (c1, ..., cn).
///
///
///
#[derive(Debug)]
pub struct World {
    component_pools: Vec<Vec<Box<dyn Component>>>,
    component_ids: HashMap<TypeId, ComponentID>,
    component_id_cap: ComponentID,
}

impl World {
    pub fn new() -> Self {
        Self {
            component_pools: vec![],
            component_ids: HashMap::new(),
            component_id_cap: 0,
        }
    }

    pub fn add_entity(&mut self, components: Vec<Box<dyn Component>>) {
        for component in components {
            if self.component_ids.contains_key(&component.type_id()) {
                self.component_pools
                    .get_mut(*self.component_ids.get(&component.type_id()).unwrap())
                    .unwrap()
                    .push(component)
            } else {
                self.component_ids
                    .insert(component.type_id(), self.component_id_cap);
                self.component_id_cap += 1;
                self.component_pools.push(vec![component]);
            }
        }
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
        let type_ids = C::blah;
        todo!()
    }
}

impl<C: ComponentTuple> View<C> {
    pub fn new(world: &World) -> Self {
        todo!()
    }
}

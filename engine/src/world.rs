use crate::component::Component;
use crate::component::ComponentTuple;
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

type ComponentID = usize;

#[derive(Debug)]
pub struct World {
    component_pools: Vec<Vec<Box<dyn Component>>>,
    component_ids: HashMap<TypeId, ComponentID>,
    component_id_cap: ComponentID,
    entity_components: Vec<HashSet<ComponentID>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            component_pools: vec![],
            component_ids: HashMap::new(),
            component_id_cap: 0,
            entity_components: vec![],
        }
    }

    pub fn add_entity(&mut self, components: Vec<Box<dyn Component>>) {
        let mut entity_component_ids = HashSet::new();
        for component in components {
            if self.component_ids.contains_key(&component.type_id()) {
                self.component_pools
                    .get_mut(*self.component_ids.get(&component.type_id()).unwrap())
                    .unwrap()
                    .push(component);
                //entity_component_ids.insert(*self.component_ids.get(&component.type_id()).unwrap());
            } else {
                self.component_ids
                    .insert(component.type_id(), self.component_id_cap);
                self.component_id_cap += 1;
                //entity_component_ids.insert(*self.component_ids.get(&component.type_id()).unwrap());
                self.component_pools.push(vec![component]);
            }
        }
        self.entity_components.push(entity_component_ids);
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
pub struct View<'a, C: ComponentTuple> {
    entity_index: usize,
    phantom: PhantomData<C>,
    world: &'a World,
}

impl<'a, C: ComponentTuple> Iterator for View<'a, C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        let mut done = false;

        while !done {
            let mut ent_match = true;
            for type_id in C::type_ids() {
                if !self
                    .world
                    .entity_components
                    .get(self.entity_index)
                    .unwrap()
                    .contains(self.world.component_ids.get(&type_id).unwrap())
                {
                    ent_match = false;
                    break;
                }
            }
            if ent_match {
                self.entity_index += 1;
                return Some(C::new(&vec![]));
            }

            if self.entity_index == self.world.entity_components.len() {
                done = true;
            }
        }
        None
    }
}

impl<'a, C: ComponentTuple> View<'a, C> {
    pub fn new(world: &'a World) -> Self {
        Self {
            entity_index: 0,
            phantom: PhantomData::<C>,
            world,
        }
    }
}

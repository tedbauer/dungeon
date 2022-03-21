use crate::component::Component;
use crate::component::ComponentTuple;
use crate::component_pool::ComponentPool;
use crate::component_pool::Pool;
use anyhow::{anyhow, Result};
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

pub type ComponentId = usize;
pub type EntityId = usize;

pub struct World {
    pools: Vec<Box<dyn ComponentPool>>,
    component_ids: HashMap<TypeId, ComponentId>,
    component_id_head: ComponentId,
    entity_id_head: EntityId,
    entity_components: HashMap<EntityId, HashSet<ComponentId>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            pools: vec![],
            component_ids: HashMap::new(),
            component_id_head: 0,
            entity_id_head: 0,
            entity_components: HashMap::new(),
        }
    }

    pub fn create_pool<C: 'static + Component>(&mut self) {
        let p = Pool::<C>::new();
        self.pools.push(Box::new(p));

        self.component_ids
            .insert(TypeId::of::<C>(), self.component_id_head);
        self.component_id_head += 1;
    }

    pub fn create_entity(&mut self) -> EntityId {
        let new_entity = self.entity_id_head;
        self.entity_id_head += 1;
        new_entity
    }

    pub fn get_component<C: Component>(&self, entity: EntityId) -> Result<Option<&C>> {
        let component_id = self.component_ids.get(&TypeId::of::<C>()).unwrap();
        let pool = self
            .pools
            .get(*component_id)
            .ok_or(anyhow!(
                "no pool for component '{:?}' exists",
                TypeId::of::<C>()
            ))?
            .as_any()
            .downcast_ref::<Pool<C>>()
            .ok_or(anyhow!("downcast to Pool<{:?}> failed", TypeId::of::<C>()))?;
        pool.get_component(entity)
    }

    pub fn get_component_mut<C: Component>(&mut self, entity: EntityId) -> Result<Option<&mut C>> {
        let component_id = self.component_ids.get(&TypeId::of::<C>()).unwrap();
        let pool = self
            .pools
            .get_mut(*component_id)
            .ok_or(anyhow!(
                "no pool for component '{:?}' exists",
                TypeId::of::<C>()
            ))?
            .as_any_mut()
            .downcast_mut::<Pool<C>>()
            .ok_or(anyhow!("downcast to Pool<{:?}> failed", TypeId::of::<C>()))?;
        pool.get_component_mut(entity)
    }

    pub fn assign<C: Component>(&mut self, entity: EntityId, component: C) -> Result<()> {
        let component_id = self.component_ids.get(&TypeId::of::<C>()).ok_or(anyhow!(
            "no pool for component {:?} exists",
            TypeId::of::<C>()
        ))?;

        let mut pool = self
            .pools
            .get_mut(*component_id)
            .ok_or(anyhow!(
                "no pool for component '{:?}' exists",
                TypeId::of::<C>()
            ))?
            .as_any_mut()
            .downcast_mut::<Pool<C>>()
            .ok_or(anyhow!("downcast to Pool<{:?}> failed", TypeId::of::<C>()))?;
        pool.add_component(entity, component);

        if self.entity_components.contains_key(&entity) {
            self.entity_components
                .get_mut(&entity)
                .unwrap()
                .insert(*self.component_ids.get(&TypeId::of::<C>()).unwrap());
        } else {
            let mut new_set = HashSet::new();
            new_set.insert(*self.component_ids.get(&TypeId::of::<C>()).unwrap());
            self.entity_components.insert(entity, new_set);
        }

        Ok(())
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
    type Item = EntityId;

    // TODO: this is crazy inefficient, use bitmasks or something
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.entity_index == self.world.entity_components.len() {
                break;
            }
            let mut ent_match = true;
            for type_id in C::type_ids() {
                if !self
                    .world
                    .entity_components
                    .get(&self.entity_index)
                    .unwrap()
                    .contains(self.world.component_ids.get(&type_id).unwrap())
                {
                    ent_match = false;
                    break;
                }
            }
            if ent_match {
                let ent = self.entity_index;
                self.entity_index += 1;
                return Some(ent);
            }

            self.entity_index += 1;
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

#[cfg(test)]
mod tests {}

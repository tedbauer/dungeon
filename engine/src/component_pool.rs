use crate::component::Component;
use crate::world::EntityId;
use anyhow::anyhow;
use std::any::Any;

/// A component pool like one typically used in an
/// [Entity-Component-System](https://en.wikipedia.org/wiki/Entity_component_system)
/// game engine.
///
pub struct Pool<C: Component> {
    pool: Vec<Option<C>>,
    size: usize,
}

impl<C: Component> Pool<C> {
    pub fn new() -> Self {
        let mut pool = Vec::<Option<C>>::new();
        let size = 1000;
        pool.resize_with(size, || None);

        Self { pool, size }
    }

    pub fn add_component(&mut self, entity_id: EntityId, component: C) -> anyhow::Result<()> {
        if entity_id >= self.size {
            self.size = entity_id * 2;
            self.pool.resize_with(self.size, || None);
        }

        if let Some(component_slot) = self.pool.get_mut(entity_id) {
            *component_slot = Some(component);
            Ok(())
        } else {
            Err(anyhow!("no space allocated for entity {:?}", entity_id))
        }
    }

    pub fn get_component(&self, entity_id: EntityId) -> Result<Option<&C>, anyhow::Error> {
        self.pool
            .get(entity_id)
            .map(|component_slot| component_slot.as_ref())
            .ok_or(anyhow!("no space allocated for entity {:?}", entity_id))
    }

    pub fn get_component_mut(
        &mut self,
        entity_id: EntityId,
    ) -> Result<Option<&mut C>, anyhow::Error> {
        self.pool
            .get_mut(entity_id)
            .map(|component_slot| component_slot.as_mut())
            .ok_or(anyhow!("no space allocated for entity {:?}", entity_id))
    }
}

pub trait ComponentPool {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> ComponentPool for Pool<T>
where
    T: Component,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

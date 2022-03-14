use crate::component::Component;
use crate::world::EntityId;

use std::any::Any;

pub struct Pool<C: Component> {
    pool: Vec<Option<C>>,
    size: usize,
}

impl<C: Component> Pool<C> {
    //FIXME: there's a bug with this resizing
    pub fn new() -> Self {
        let mut pool = Vec::<Option<C>>::new();
        pool.resize_with(2500, || None);

        Self { pool, size: 2500 }
    }

    pub fn add_component(&mut self, entityId: EntityId, component: C) {
        if entityId < self.size {
            *self.pool.get_mut(entityId).unwrap() = Some(component);
        } else {
            self.size = self.size * 2;
            self.pool.resize_with(self.size, || None);
        }
    }

    // TODO: create custom type instead of `Option` and get rid of `unwrap()`s.
    pub fn get_component(&self, entityId: EntityId) -> Option<&C> {
        self.pool.get(entityId).unwrap().as_ref()
    }

    pub fn get_component_mut(&mut self, entityId: EntityId) -> Option<&mut C> {
        self.pool.get_mut(entityId).unwrap().as_mut()
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

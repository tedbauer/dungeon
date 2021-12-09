use crate::component::Component;
use crate::component::ComponentTuple;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

pub type ComponentId = usize;
pub type EntityId = usize;

#[derive(Debug)]
struct Pool<C: Component> {
    pool: Vec<Option<C>>,
    size: usize,
}

impl<C: Component> Pool<C> {
    fn new() -> Self {
        let mut pool = Vec::<Option<C>>::new();
        pool.resize_with(50, || None);

        Self { pool, size: 50 }
    }

    fn add_component(&mut self, entityId: EntityId, component: C) {
        if entityId < self.size {
            *self.pool.get_mut(entityId).unwrap() = Some(component);
        } else {
            self.size = self.size * 2;
            self.pool.resize_with(self.size, || None);
        }

        println!("{:?}", self.pool);
    }

    // TODO: create custom type instead of `Option` and get rid of `unwrap()`s.
    fn get_component(&self, entityId: EntityId) -> Option<&C> {
        self.pool.get(entityId).unwrap().as_ref()
    }

    fn get_component_mut(&mut self, entityId: EntityId) -> Option<&mut C> {
        self.pool.get_mut(entityId).unwrap().as_mut()
    }
}

trait ComponentPool {
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

    pub fn get_component<C: Component>(&mut self, entity: EntityId) -> Option<&C> {
        let component_id = self.component_ids.get(&TypeId::of::<C>()).unwrap();
        let pool = self
            .pools
            .get_mut(*component_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Pool<C>>()
            .unwrap();
        pool.get_component(entity)
    }

    pub fn get_component_mut<C: Component>(&mut self, entity: EntityId) -> Option<&mut C> {
        let component_id = self.component_ids.get(&TypeId::of::<C>()).unwrap();
        let pool = self
            .pools
            .get_mut(*component_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Pool<C>>()
            .unwrap();
        pool.get_component_mut(entity)
    }

    pub fn assign<C: Component>(&mut self, entity: EntityId, component: C) {
        let component_id = self.component_ids.get(&TypeId::of::<C>()).unwrap();
        let mut pool = self
            .pools
            .get_mut(*component_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Pool<C>>()
            .unwrap();
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
    }
}

/*
fn test() {

    let player_entity = world.create_entity();
    let text = Text {
        t: "hello".to_string(),
        color: "Brown".to_string(),
    };

    world.add_component(player_entity, Position { x: 5, y: 4 });
    world.add_component(player_entity, Player {});
    world.add_component(player_entity, text);

    world.add_entity()
        .with_component(Position { x: 5, y: 4 })
        .with_component(Player {})
        .create();

    let entity_1 = world.add_entity()
        .with_component(Position { x: 5, y: 4 })
        .with_component(Text { text: "hello".to_string() })
        .with_component(Enemy {})
        .create();

}

struct EntityBuilder {
    id: u32,
}

impl EntityBuilder {
    pub fn new(world: World) -> Self {
    }

    pub fn with_component(C: impl Component) -> Self {

    }

    pub fn build() -> EntityID {

    }

}
*/

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

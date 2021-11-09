use std::any::Any;

pub trait Component {
    fn as_any(&self) -> &dyn Any;
}

pub struct ComponentQuery {
    comps: Vec<Box<dyn Component>>,
    remove: Vec<Box<dyn Component>>,
}

impl ComponentQuery {
    pub fn new(comps: Vec<Box<dyn Component>>) -> Self {
        Self {
            comps,
            remove: vec![],
        }
    }

    pub fn filter(self, remove: Vec<Box<dyn Component>>) -> Self {
        Self {
            comps: self.comps,
            remove,
        }
    }
}

pub struct World {
    entities: Vec<Vec<Box<dyn Component>>>,
}

impl World {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    pub fn add_entity(&mut self, entity: Vec<Box<dyn Component>>) {
        self.entities.push(entity);
    }

    pub fn query(&self, query: ComponentQuery) -> Vec<&Vec<Box<dyn Component>>> {
        self.entities
            .iter()
            .filter(|entity| {
                entity.iter().any(|comp| {
                    query
                        .comps
                        .iter()
                        .any(|query_comp| query_comp.type_id() == comp.type_id())
                })
            })
            .collect()
    }
}

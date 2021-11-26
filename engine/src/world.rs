use crate::component::ComponentTuple;

pub struct World {}

impl World {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_entity(&self, entity: &dyn ComponentTuple) {
        todo!()
    }
}

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

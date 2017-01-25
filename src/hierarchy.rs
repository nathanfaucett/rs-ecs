use component::Component;
use manager::HashMapManager;
use entity::Entity;


pub struct Hierarchy {
    entity: Entity,
    depth: usize,
    parent: Option<Entity>,
    children: Vec<Entity>,
}

impl Component for Hierarchy {
    type Manager = HashMapManager<Self>;
}

impl Hierarchy {

    pub fn new(entity: Entity) -> Self {
        Hierarchy {
            entity: entity,
            depth: 0usize,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn entity(&self) -> Entity { self.entity }
    pub fn depth(&self) -> usize { self.depth }
    pub fn parent(&self) -> Option<Entity> { self.parent.clone() }
    pub fn children(&self) -> &Vec<Entity> { &self.children }
}

use entity_manager::EntityManager;
use entity::Entity;


pub struct Hierarchy {
    entity: Entity,
    depth: usize,
    parent: Option<Entity>,
    children: Vec<Entity>,
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

    pub fn add_child(&mut self, entity_manager: &EntityManager, child: &Entity) {
        if &self.entity != child && !self.children.contains(child) {
            self.children.push(*child);

            if let Some(child_hierarchy) = entity_manager.component::<Hierarchy>(child) {
                let mut lock = child_hierarchy.as_mut().unwrap();
                lock.parent = Some(self.entity);
                lock.depth = self.depth + 1usize;

                Self::set_children_depth(entity_manager, &child_hierarchy.children, self.depth + 2usize);
            }
        }
    }
    pub fn remove_child(&mut self, entity_manager: &EntityManager, child: &Entity) {
        if let Some(index) = self.children.iter().position(|e| e == child) {
            self.children.remove(index);

            if let Some(child_hierarchy) = entity_manager.component::<Hierarchy>(child) {
                let mut lock = child_hierarchy.as_mut().unwrap();
                lock.parent = None;
                lock.depth = 0usize;

                Self::set_children_depth(entity_manager, &child_hierarchy.children, self.depth + 1usize);
            }
        }
    }

    fn set_children_depth(entity_manager: &EntityManager, children: &Vec<Entity>, depth: usize) {
        for child in children.iter() {
            if let Some(child_hierarchy) = entity_manager.component::<Hierarchy>(child) {
                let mut lock = child_hierarchy.as_mut().unwrap();
                lock.depth = depth;
                Self::set_children_depth(entity_manager, &child_hierarchy.children, depth + 1usize);
            }
        }
    }
}

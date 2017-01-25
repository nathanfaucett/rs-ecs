use std::collections::HashSet;

use entity::Entity;


pub struct Entities {
    entities: HashSet<Entity>,
}

unsafe impl Send for Entities {}
unsafe impl Sync for Entities {}

impl Entities {
    pub fn new() -> Self {
        Entities {
            entities: HashSet::new(),
        }
    }
    pub fn new_entity(&mut self) -> Entity {
        let entity = Entity::new();
        self.entities.insert(entity);
        entity
    }
    pub fn remove_entity(&mut self, entity: &Entity) -> bool {
        self.entities.remove(entity)
    }
    pub fn is_alive(&self, entity: &Entity) -> bool {
        self.entities.contains(&entity)
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_entities() {
        let mut entities = Entities::new();
        let entity = entities.new_entity();
        assert!(entities.is_alive(&entity));
        assert!(entities.remove_entity(&entity));
        assert!(!entities.is_alive(&entity));
    }
}

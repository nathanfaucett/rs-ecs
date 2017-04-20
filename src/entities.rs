use collection_traits::*;
use hash_set::HashSet;

use super::entity::Entity;


pub struct Entities {
    entities: HashSet<Entity>,
}

unsafe impl Send for Entities {}
unsafe impl Sync for Entities {}

impl Entities {
    #[inline]
    pub fn new() -> Self {
        Entities {
            entities: HashSet::new(),
        }
    }
    #[inline]
    pub fn create(&mut self) -> Entity {
        let entity = Entity::new();
        self.entities.insert(entity);
        entity
    }
    #[inline]
    pub fn remove(&mut self, entity: &Entity) -> bool {
        self.entities.remove(entity)
    }
    #[inline]
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
        let entity = entities.create();
        assert!(entities.is_alive(&entity));
        assert!(entities.remove(&entity));
        assert!(!entities.is_alive(&entity));
    }
}

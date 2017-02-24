use std::sync::{Arc, RwLock, RwLockWriteGuard, RwLockReadGuard};

use super::component::Component;
use super::components::Components;
use super::entities::Entities;
use super::entity::Entity;


#[derive(Clone)]
pub struct EntityManager {
    components: Arc<RwLock<Components>>,
    entities: Arc<RwLock<Entities>>,
}

impl EntityManager {
    #[inline]
    pub fn new() -> Self {
        EntityManager {
            components: Arc::new(RwLock::new(Components::new())),
            entities: Arc::new(RwLock::new(Entities::new())),
        }
    }

    #[inline]
    pub fn components(&self) -> RwLockReadGuard<Components> {
        self.components.read().expect("failed to acquire lock on components")
    }
    #[inline]
    pub fn components_mut(&self) -> RwLockWriteGuard<Components> {
        self.components.write().expect("failed to acquire lock on components")
    }

    #[inline]
    pub fn entities(&self) -> RwLockReadGuard<Entities> {
        self.entities.read().expect("failed to acquire lock on entities")
    }
    #[inline]
    pub fn entities_mut(&self) -> RwLockWriteGuard<Entities> {
        self.entities.write().expect("failed to acquire lock on entities")
    }

    #[inline]
    pub fn create_entity(&self) -> Entity {
        self.entities_mut().create()
    }
    #[inline]
    pub fn remove_entity(&self, entity: &Entity) -> bool {
        self.entities_mut().remove(entity)
    }
    #[inline]
    pub fn is_entity_alive(&self, entity: &Entity) -> bool {
        self.entities().is_alive(entity)
    }

    #[inline]
    pub fn register_component<T: Component>(&self) {
        self.components_mut().register::<T>()
    }
    #[inline]
    pub fn unregister_component<T: Component>(&self) {
        self.components_mut().unregister::<T>()
    }

    #[inline]
    pub fn insert_component<T: Component>(&self, entity: Entity, component: T) {
        self.components_mut().insert::<T>(entity, component)
    }
    #[inline]
    pub fn remove_component<T: Component>(&self, entity: &Entity) -> Option<T> {
        self.components_mut().remove::<T>(entity)
    }
}

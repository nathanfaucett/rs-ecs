use std::any::{Any, TypeId};
use std::sync::RwLock;
use std::collections::HashMap;

use component::Component;
use entity::Entity;
use manager::MaskedManager;


pub struct Components {
    managers: HashMap<TypeId, Box<ManagerLock>>,
}

unsafe impl Send for Components {}
unsafe impl Sync for Components {}

impl Components {
    pub fn new() -> Self {
        Components {
            managers: HashMap::new(),
        }
    }

    pub fn register<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.managers.insert(type_id, Box::new(RwLock::new(MaskedManager::<T>::new())));
    }
    pub fn unregister<T: Component>(&mut self) {
        self.managers.remove(&TypeId::of::<T>());
    }

    pub fn managers(&self) -> &HashMap<TypeId, Box<ManagerLock>> {
        &self.managers
    }
    pub fn manager<T: Component>(&self) -> &RwLock<MaskedManager<T>> {
        unsafe {
            self.managers
                .get(&TypeId::of::<T>())
                .expect("unregistered component use, make sure to register components.")
                .downcast_ref_unchecked::<RwLock<MaskedManager<T>>>()
        }
    }

    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        match self.manager::<T>().write() {
            Ok(mut manager) => manager.insert(entity, component),
            Err(..) => (),
        }
    }
    pub fn remove<T: Component>(&mut self, entity: &Entity) -> Option<T> {
        match self.manager::<T>().write() {
            Ok(mut manager) => manager.remove(entity),
            Err(..) => None,
        }
    }
}


pub trait ManagerLock: Any + Send + Sync {}

impl_any!(ManagerLock);

impl<T: Component> ManagerLock for RwLock<MaskedManager<T>> {}


#[cfg(test)]
mod test {
    use super::*;
    use entities::Entities;
    use manager::HashMapManager;


    #[derive(Debug, PartialEq, Eq)]
    pub struct SomeComponent;

    impl Component for SomeComponent {
        type Manager = HashMapManager<Self>;
    }


    #[test]
    fn test_components() {
        let mut components = Components::new();
        let mut entities = Entities::new();
        let entity = entities.new_entity();

        components.register::<SomeComponent>();
        components.insert(entity.clone(), SomeComponent);

        let manager = components.manager::<SomeComponent>().read().unwrap();
        assert_eq!(manager.get(&entity), Some(&SomeComponent));
    }
}

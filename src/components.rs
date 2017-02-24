use std::any::{Any, TypeId};
use std::sync::RwLock;
use std::collections::HashMap;

use super::component::Component;
use super::entity::Entity;
use super::component_manager::WrappedComponentManager;


pub struct Components {
    component_managers: HashMap<TypeId, Box<ComponentManagerLock>>,
}

unsafe impl Send for Components {}
unsafe impl Sync for Components {}

impl Components {
    #[inline]
    pub fn new() -> Self {
        Components {
            component_managers: HashMap::new(),
        }
    }

    pub fn register<T: Component>(&mut self) {
        self.component_managers.insert(
            TypeId::of::<T>(),
            Box::new(RwLock::new(WrappedComponentManager::<T>::new()))
        );
    }
    #[inline]
    pub fn unregister<T: Component>(&mut self) {
        self.component_managers.remove(&TypeId::of::<T>());
    }

    #[inline]
    pub fn component_managers(&self) -> &HashMap<TypeId, Box<ComponentManagerLock>> {
        &self.component_managers
    }
    pub fn component_manager<T: Component>(&self) -> &RwLock<WrappedComponentManager<T>> {
        unsafe {
            self.component_managers
                .get(&TypeId::of::<T>())
                .expect("unregistered component use, make sure to register components.")
                .downcast_ref_unchecked::<RwLock<WrappedComponentManager<T>>>()
        }
    }

    #[inline]
    pub fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        match self.component_manager::<T>().write() {
            Ok(mut component_manager) => component_manager.insert(entity, component),
            Err(..) => panic!("unregistered component inserted, make sure to register components."),
        }
    }
    #[inline]
    pub fn remove<T: Component>(&mut self, entity: &Entity) -> Option<T> {
        match self.component_manager::<T>().write() {
            Ok(mut component_manager) => component_manager.remove(entity),
            Err(..) => None,
        }
    }
}


pub trait ComponentManagerLock: Any + Send + Sync {}

impl_any!(ComponentManagerLock);

impl<T: Component> ComponentManagerLock for RwLock<WrappedComponentManager<T>> {}


#[cfg(test)]
mod test {
    use super::*;
    use entities::Entities;
    use component_manager::HashMapComponentManager;


    #[derive(Debug, PartialEq, Eq)]
    pub struct SomeComponent;

    impl Component for SomeComponent {
        type ComponentManager = HashMapComponentManager<Self>;
    }


    #[test]
    fn test_components() {
        let mut components = Components::new();
        let mut entities = Entities::new();
        let entity = entities.create();

        components.register::<SomeComponent>();
        components.insert(entity.clone(), SomeComponent);

        let component_manager = components.component_manager::<SomeComponent>().read().unwrap();
        assert_eq!(component_manager.get(&entity), Some(&SomeComponent));
    }
}

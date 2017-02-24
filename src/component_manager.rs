use std::any::Any;
use std::collections::HashMap;

use super::component::Component;
use super::entity::Entity;


pub trait ComponentManager<T: Component>: Sized + Any + Send + Sync {

    fn new() -> Self;

    fn clear(&mut self);

    fn get(&self, entity: &Entity) -> Option<&T>;
    fn get_mut(&mut self, entity: &Entity) -> Option<&mut T>;

    fn contains(&self, entity: &Entity) -> bool;
    fn insert(&mut self, entity: Entity, component: T);
    fn remove(&mut self, entity: &Entity) -> Option<T>;
}


pub struct WrappedComponentManager<T: Component> {
    inner: T::ComponentManager,
}

impl<T: Component> WrappedComponentManager<T> {
    #[inline]
    pub fn new() -> WrappedComponentManager<T> {
        WrappedComponentManager {
            inner: ComponentManager::new(),
        }
    }
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    #[inline]
    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.inner.get(entity)
    }
    #[inline]
    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.inner.get_mut(entity)
    }
    #[inline]
    pub fn contains(&self, entity: &Entity) -> bool {
        self.inner.contains(entity)
    }
    #[inline]
    pub fn insert(&mut self, entity: Entity, component: T) {
        self.inner.insert(entity, component);
    }
    #[inline]
    pub fn remove(&mut self, entity: &Entity) -> Option<T> {
        self.inner.remove(entity)
    }
}

impl<T: Component> Drop for WrappedComponentManager<T> {
    #[inline]
    fn drop(&mut self) {
        self.clear();
    }
}


pub struct HashMapComponentManager<T: Component> {
    map: HashMap<Entity, T>,
}

impl<T: Component> ComponentManager<T> for HashMapComponentManager<T> {

    #[inline]
    fn new() -> Self {
        HashMapComponentManager {
            map: HashMap::new(),
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.map.clear();
    }

    #[inline]
    fn get(&self, entity: &Entity) -> Option<&T> {
        self.map.get(entity)
    }
    #[inline]
    fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.map.get_mut(entity)
    }

    #[inline]
    fn contains(&self, entity: &Entity) -> bool {
        self.map.contains_key(entity)
    }
    #[inline]
    fn insert(&mut self, entity: Entity, component: T) {
        self.map.insert(entity, component);
    }
    #[inline]
    fn remove(&mut self, entity: &Entity) -> Option<T> {
        self.map.remove(entity)
    }
}


pub struct VecComponentManager<T: Component> {
    vec: Vec<(Entity, T)>,
}

impl<T: Component> VecComponentManager<T> {
    #[inline]
    fn index_of(&self, entity: &Entity) -> Option<usize> {
        self.vec.iter().position(|&(e, _)| &e == entity)
    }
}

impl<T: Component> ComponentManager<T> for VecComponentManager<T> {

    #[inline]
    fn new() -> Self {
        VecComponentManager {
            vec: Vec::new(),
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.vec.clear();
    }

    fn get(&self, entity: &Entity) -> Option<&T> {
        match self.index_of(entity) {
            Some(index) => {
                let &(_, ref component) = unsafe { self.vec.get_unchecked(index) };
                Some(component)
            },
            None => None,
        }
    }
    fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        match self.index_of(entity) {
            Some(index) => {
                let &mut (_, ref mut component) = unsafe { self.vec.get_unchecked_mut(index) };
                Some(component)
            },
            None => None,
        }
    }

    #[inline]
    fn contains(&self, entity: &Entity) -> bool {
        self.index_of(entity).is_some()
    }
    #[inline]
    fn insert(&mut self, entity: Entity, component: T) {
        self.vec.push((entity, component));
    }
    fn remove(&mut self, entity: &Entity) -> Option<T> {
        match self.index_of(entity) {
            Some(index) => {
                let (_, component) = self.vec.remove(index);
                Some(component)
            },
            None => None,
        }
    }
}

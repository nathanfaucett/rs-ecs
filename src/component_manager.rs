use std::any::Any;

use atomic::Atomic;

use collection_traits::*;
use vector::Vector;
use hash_map::HashMap;

use super::component::Component;
use super::entity::Entity;


pub trait ComponentManager<T: Component>: Sized + Any + Send + Sync {

    fn new() -> Self;

    fn clear(&mut self);

    fn get(&self, entity: &Entity) -> Option<&Atomic<T>>;

    fn contains(&self, entity: &Entity) -> bool;
    fn insert(&mut self, entity: Entity, component: T);
    fn remove(&mut self, entity: &Entity) -> Option<T>;

    fn replace(&mut self);
}


pub struct WrappedComponentManager<T: Component> {
    inner: T::ComponentManager,
}

impl<T: Component> ComponentManager<T> for WrappedComponentManager<T> {
    #[inline]
    fn new() -> WrappedComponentManager<T> {
        WrappedComponentManager {
            inner: ComponentManager::new(),
        }
    }
    #[inline]
    fn clear(&mut self) {
        self.inner.clear();
    }
    #[inline]
    fn get(&self, entity: &Entity) -> Option<&Atomic<T>> {
        self.inner.get(entity)
    }
    #[inline]
    fn contains(&self, entity: &Entity) -> bool {
        self.inner.contains(entity)
    }
    #[inline]
    fn insert(&mut self, entity: Entity, component: T) {
        self.inner.insert(entity, component);
    }
    #[inline]
    fn remove(&mut self, entity: &Entity) -> Option<T> {
        self.inner.remove(entity)
    }
    #[inline]
    fn replace(&mut self) {
        self.inner.replace();
    }
}

impl<T: Component> Drop for WrappedComponentManager<T> {
    #[inline]
    fn drop(&mut self) {
        self.clear();
    }
}


pub struct HashMapComponentManager<T: Component> {
    map: HashMap<Entity, Atomic<T>>,
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
    fn get(&self, entity: &Entity) -> Option<&Atomic<T>> {
        self.map.get(entity)
    }

    #[inline]
    fn contains(&self, entity: &Entity) -> bool {
        self.map.contains_key(entity)
    }
    #[inline]
    fn insert(&mut self, entity: Entity, component: T) {
        self.map.insert(entity, Atomic::new(component));
    }
    #[inline]
    fn remove(&mut self, entity: &Entity) -> Option<T> {
        match self.map.remove(entity) {
            Some(component) => Some(component.take()),
            None => None,
        }
    }
    #[inline]
    fn replace(&mut self) {
        for (_, component) in self.map.iter_mut() {
            component.replace()
        }
    }
}


pub struct VecComponentManager<T: Component> {
    vec: Vector<(Entity, Atomic<T>)>,
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
            vec: Vector::new(),
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.vec.clear();
    }

    #[inline]
    fn get(&self, entity: &Entity) -> Option<&Atomic<T>> {
        match self.index_of(entity) {
            Some(index) => {
                let &(_, ref component) = unsafe { self.vec.get_unchecked(index) };
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
        self.vec.push((entity, Atomic::new(component)));
    }
    fn remove(&mut self, entity: &Entity) -> Option<T> {
        match self.index_of(entity) {
            Some(index) => {
                let (_, component) = self.vec.remove(index);
                Some(component.take())
            },
            None => None,
        }
    }
    #[inline]
    fn replace(&mut self) {
        for &mut (_, ref mut component) in self.vec.iter_mut() {
            component.replace()
        }
    }
}

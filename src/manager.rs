use std::any::Any;
use std::collections::HashMap;

use component::Component;
use entity::Entity;


pub trait Manager<T: Component>: Sized + Any + Send + Sync {

    fn new() -> Self;

    fn clear(&mut self);

    fn get(&self, entity: &Entity) -> Option<&T>;
    fn get_mut(&mut self, entity: &Entity) -> Option<&mut T>;

    fn contains(&self, entity: &Entity) -> bool;
    fn insert(&mut self, entity: Entity, component: T);
    fn remove(&mut self, entity: &Entity) -> Option<T>;
}


pub struct MaskedManager<T: Component> {
    inner: T::Manager,
}

impl<T: Component> MaskedManager<T> {
    pub fn new() -> MaskedManager<T> {
        MaskedManager {
            inner: Manager::new(),
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.inner.get(entity)
    }
    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.inner.get_mut(entity)
    }

    pub fn contains(&self, entity: &Entity) -> bool {
        self.inner.contains(entity)
    }
    pub fn insert(&mut self, entity: Entity, component: T) {
        self.inner.insert(entity, component);
    }
    pub fn remove(&mut self, entity: &Entity) -> Option<T> {
        self.inner.remove(entity)
    }
}

impl<T: Component> Drop for MaskedManager<T> {
    fn drop(&mut self) {
        self.clear();
    }
}


pub struct HashMapManager<T: Component> {
    map: HashMap<Entity, T>,
}

impl<T: Component> Manager<T> for HashMapManager<T> {

    fn new() -> Self {
        HashMapManager {
            map: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.map.clear();
    }

    fn get(&self, entity: &Entity) -> Option<&T> {
        self.map.get(entity)
    }
    fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.map.get_mut(entity)
    }

    fn contains(&self, entity: &Entity) -> bool {
        self.map.contains_key(entity)
    }
    fn insert(&mut self, entity: Entity, component: T) {
        self.map.insert(entity, component);
    }
    fn remove(&mut self, entity: &Entity) -> Option<T> {
        self.map.remove(entity)
    }
}


pub struct VecManager<T: Component> {
    vec: Vec<(Entity, T)>,
}

impl<T: Component> VecManager<T> {
    #[inline]
    fn index_of(&self, entity: &Entity) -> Option<usize> {
        self.vec.iter().position(|&(e, _)| &e == entity)
    }
}

impl<T: Component> Manager<T> for VecManager<T> {

    fn new() -> Self {
        VecManager {
            vec: Vec::new(),
        }
    }

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

    fn contains(&self, entity: &Entity) -> bool {
        self.index_of(entity).is_some()
    }
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

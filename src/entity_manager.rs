use std::marker::PhantomData;
use std::any::TypeId;
use std::collections::{hash_map, HashMap};

use atomic::{Atomic, AtomicValue};

use entity::Entity;
use hierarchy::Hierarchy;


pub struct EntityManager {
    entities: HashMap<Entity, HashMap<TypeId, Atomic>>,
    count: HashMap<TypeId, usize>,
}

unsafe impl Send for EntityManager {}
unsafe impl Sync for EntityManager {}

impl EntityManager {

    pub fn new() -> Self {
        EntityManager {
            entities: HashMap::new(),
            count: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = Entity::new();
        self.entities.insert(entity, HashMap::new());
        self.add_component(&entity, Hierarchy::new(entity));
        entity
    }
    pub fn remove_entity(&mut self, entity: &Entity) {
        if let Some(components) = self.entities.remove(entity) {
            for (type_id, _) in components.iter() {
                if let Some(count) = self.count.get_mut(type_id) {
                    *count -= 1usize;
                }
            }
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: &Entity, component: T) -> bool {
        let mut added = false;
        let type_id = TypeId::of::<T>();

        if let Some(entity_components) = self.entities.get_mut(entity) {
            entity_components.insert(type_id, Atomic::new(component));
            added = true;
        }

        if added {
            if self.count.contains_key(&type_id) {
                *(self.count.get_mut(&type_id).unwrap()) += 1usize;
            } else {
                self.count.insert(type_id, 1usize);
            }
        }

        added
    }

    pub fn component_count<T: 'static>(&self) -> usize {
        self.component_count_by_type_id(&TypeId::of::<T>())
    }
    pub fn component_count_by_type_id(&self, type_id: &TypeId) -> usize {
        if let Some(count) = self.count.get(&type_id) {
            *count
        } else {
            0usize
        }
    }

    pub fn component<T: 'static>(&self, entity: &Entity) -> Option<AtomicValue<T>> {
        match self.entities.get(entity) {
            Some(components) => match components.get(&TypeId::of::<T>()) {
                Some(atomic) => atomic.downcast_ref::<T>(),
                None => None,
            },
            None => None,
        }
    }

    pub fn remove_component<T: 'static>(&mut self, entity: &Entity) -> Option<T> {
        match self.remove_component_by_type_id(entity, &TypeId::of::<T>()) {
            Some(atomic) => atomic.take::<T>(),
            None => None,
        }
    }
    pub fn remove_component_by_type_id(&mut self, entity: &Entity, type_id: &TypeId) -> Option<Atomic>{
        let component = if let Some(entity_components) = self.entities.get_mut(entity) {
            entity_components.remove(&type_id)
        } else {
            None
        };
        if component.is_some() {
            if let Some(count) = self.count.get_mut(&type_id) {
                *count -= 1usize;
            }
            component
        } else {
            None
        }
    }

    pub fn iter(&self) -> hash_map::Iter<Entity, HashMap<TypeId, Atomic>> {
        self.entities.iter()
    }
    pub fn iter_mut(&mut self) -> hash_map::IterMut<Entity, HashMap<TypeId, Atomic>> {
        self.entities.iter_mut()
    }

    pub fn component_iter<T: 'static>(&self) -> ComponentIter<T> {
        ComponentIter::new(&self.count, self.entities.iter())
    }
}

pub struct ComponentIter<'a, T: 'static> {
    phantom_data: PhantomData<T>,
    iter: hash_map::Iter<'a, Entity, HashMap<TypeId, Atomic>>,
    type_id: TypeId,
    is_empty: bool,
}

impl<'a, T: 'static> ComponentIter<'a, T> {
    fn new(count: &'a HashMap<TypeId, usize>, iter: hash_map::Iter<'a, Entity, HashMap<TypeId, Atomic>>) -> Self {
        let type_id = TypeId::of::<T>();

        ComponentIter {
            phantom_data: PhantomData,
            iter: iter,
            type_id: type_id,
            is_empty: match count.get(&type_id) {
                Some(count) => *count == 0,
                None => true,
            },
        }
    }
}

impl<'a, T: 'static> Iterator for ComponentIter<'a, T> {
    type Item = (&'a Entity, AtomicValue<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty {
            None
        } else {
            loop {
                match self.iter.next() {
                    Some((entity, map)) => match map.get(&self.type_id) {
                        Some(component) => return Some((
                            entity,
                            component.downcast_ref::<T>().unwrap()
                        )),
                        None => continue,
                    },
                    None => return None,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_entity_manager() {
        let mut entity_manager = EntityManager::new();
        let entity = entity_manager.new_entity();

        assert_eq!(entity_manager.component_count::<Hierarchy>(), 1usize);

        {
            let component = entity_manager.component::<Hierarchy>(&entity).unwrap();
            assert_eq!(component.entity(), entity);
        }
        {
            let component = entity_manager.remove_component::<Hierarchy>(&entity).unwrap();
            assert_eq!(entity_manager.component_count::<Hierarchy>(), 0usize);
            assert_eq!(component.entity(), entity);
        }
    }

    #[test]
    fn test_many_entity_manager() {
        let mut entity_manager = EntityManager::new();
        let mut entities = Vec::with_capacity(1024usize);

        for i in 0..1024usize {
            let entity = entity_manager.new_entity();
            entities.push(entity);

            if i != 0 {
                let hierarchy = entity_manager.component::<Hierarchy>(&entities[i - 1]).unwrap();
                let mut lock = hierarchy.as_mut().unwrap();
                lock.add_child(&entity_manager, &entity);
            }
        }
        assert_eq!(entity_manager.component_count::<Hierarchy>(), 1024usize);

        for entity in entities.iter() {
            entity_manager.remove_component::<Hierarchy>(&entity);
        }
        assert_eq!(entity_manager.component_count::<Hierarchy>(), 0usize);
    }
}

use std::sync::RwLock;

use super::component::Component;
use super::component_manager::{WrappedComponentManager, HashMapComponentManager};
use super::entity::Entity;


pub struct Hierarchy {
    entity: Entity,
    depth: usize,
    parent: Option<Entity>,
    children: Vec<Entity>,
}

impl Component for Hierarchy {
    type ComponentManager = HashMapComponentManager<Self>;
}

impl Hierarchy {
    #[inline]
    pub fn new(entity: Entity) -> Self {
        Hierarchy {
            entity: entity,
            depth: 0usize,
            parent: None,
            children: Vec::new(),
        }
    }

    #[inline]
    pub fn entity(&self) -> Entity { self.entity }
    #[inline]
    pub fn depth(&self) -> usize { self.depth }
    #[inline]
    pub fn parent(&self) -> Option<Entity> { self.parent.clone() }
    #[inline]
    pub fn children(&self) -> &Vec<Entity> { &self.children }

    pub fn add_child(&mut self, component_manager: &RwLock<WrappedComponentManager<Self>>, child: &Entity) {
        if &self.entity != child && !self.children.contains(child) {
            self.children.push(*child);

            let children = if let Some(child_hierarchy) = component_manager.write().unwrap().get_mut(child) {
                child_hierarchy.parent = Some(self.entity);
                child_hierarchy.depth = self.depth + 1usize;
                Some(child_hierarchy.children.clone())
            } else {
                None
            };

            if let Some(children) = children {
                Self::set_children_depth(component_manager, children, self.depth + 2usize);
            }
        }
    }
    pub fn remove_child(&mut self, component_manager: &RwLock<WrappedComponentManager<Self>>, child: &Entity) {
        if let Some(index) = self.children.iter().position(|e| e == child) {
            self.children.remove(index);

            let children = if let Some(child_hierarchy) = component_manager.write().unwrap().get_mut(child) {
                child_hierarchy.parent = None;
                child_hierarchy.depth = 0usize;
                Some(child_hierarchy.children.clone())
            } else {
                None
            };

            if let Some(children) = children {
                Self::set_children_depth(component_manager, children, self.depth + 2usize);
            }
        }
    }

    fn set_children_depth(component_manager: &RwLock<WrappedComponentManager<Self>>, children: Vec<Entity>, depth: usize) {
        for child in children.iter() {
            let children = if let Some(child_hierarchy) = component_manager.write().unwrap().get_mut(child) {
                child_hierarchy.depth = depth;
                Some(child_hierarchy.children.clone())
            } else {
                None
            };

            if let Some(children) = children {
                Self::set_children_depth(component_manager, children, depth + 1usize);
            }
        }
    }
}

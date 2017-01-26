use std::any::Any;
use std::sync::RwLock;

use components::Components;
use entities::Entities;


pub trait Process: Any + Send + Sync {
    fn run(&mut self, &RwLock<Components>, &RwLock<Entities>);
}

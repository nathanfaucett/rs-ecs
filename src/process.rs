use std::any::Any;

use entity_manager::EntityManager;


pub trait Process: Any + Send + Sync {
    fn handle(&mut self, entity_manager: &EntityManager);
}

impl Process {
    impl_any!();
}

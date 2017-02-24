use std::any::Any;

use super::entity_manager::EntityManager;


pub trait Process: Any + Send + Sync {
    fn run(&mut self, &EntityManager);
    #[inline]
    fn priority(&self) -> usize {
        0usize
    }
}

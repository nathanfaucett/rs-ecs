use std::any::Any;


pub trait Process: Any + Send + Sync {
    fn handle(&mut self);
}

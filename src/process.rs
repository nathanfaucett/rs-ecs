use std::any::Any;


pub trait Process: Any + Send + Sync {
    fn run(&mut self);
}

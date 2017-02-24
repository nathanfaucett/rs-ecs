use std::any::Any;

use super::component_manager::ComponentManager;


pub trait Component: Sized + Any + Send + Sync {
    type ComponentManager: ComponentManager<Self> + Any + Send + Sync;
}

use std::any::Any;

use manager::Manager;


pub trait Component: Sized + Any + Send + Sync {
    type Manager: Manager<Self> + Any + Send + Sync;
}

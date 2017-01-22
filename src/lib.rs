#![feature(test)]
#![feature(raw)]
#![feature(get_type_id)]


extern crate uuid;
extern crate atomic;
extern crate thread_pool;
#[macro_use]
extern crate impl_any;


mod entity_manager;
mod entity;

mod hierarchy;

mod process;
mod process_manager;

mod scene;


pub use entity_manager::EntityManager;
pub use entity::Entity;

pub use hierarchy::Hierarchy;

pub use process::Process;
pub use process_manager::ProcessManager;

pub use scene::Scene;

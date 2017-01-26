#![feature(get_type_id)]


#[macro_use]
extern crate impl_any;
extern crate uuid;
extern crate waiter;
extern crate thread_pool;


mod component;
mod components;
mod entities;
mod entity;

mod hierarchy;

mod manager;

mod process;
mod processes;

mod scene;


pub use component::Component;
pub use components::Components;
pub use entities::Entities;
pub use entity::Entity;

pub use hierarchy::Hierarchy;

pub use manager::*;

pub use process::Process;
pub use processes::Processes;

pub use scene::Scene;

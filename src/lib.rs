#![feature(get_type_id)]


extern crate atomic;
extern crate collection_traits;
#[macro_use] extern crate impl_any;
extern crate uuid;
extern crate hash_map;
extern crate hash_set;
extern crate waiter;
extern crate thread_pool;
extern crate vector;


mod component_manager;
mod component;
mod components;

mod entities;
mod entity_manager;
mod entity;

mod process;
mod processes;

mod scene;


pub use self::component_manager::*;
pub use self::component::Component;
pub use self::components::Components;

pub use self::entities::Entities;
pub use self::entity_manager::EntityManager;
pub use self::entity::Entity;

pub use self::process::Process;
pub use self::processes::Processes;

pub use self::scene::Scene;

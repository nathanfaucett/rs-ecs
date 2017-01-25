use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use uuid::Uuid;


fn next_id() -> u64 {
    let mut hasher = DefaultHasher::new();
    Uuid::new_v4().hash(&mut hasher);
    hasher.finish()
}


#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct Entity {
    id: u64,
}

unsafe impl Send for Entity {}
unsafe impl Sync for Entity {}

impl Entity {
    pub fn new() -> Self {
        Entity {
            id: next_id(),
        }
    }
}


#[cfg(test)]
mod test {
    extern crate num_cpus;


    use super::*;
    use std::thread;


    static SIZE: usize = 1024usize;


    #[test]
    fn test_entity_id() {
        let threads = num_cpus::get() - 1usize;
        let mut handles = Vec::new();

        for _ in 0..threads {
            handles.push(thread::spawn(move || {
                let mut out = Vec::with_capacity(SIZE);
                for _ in 0..SIZE {
                    out.push(Entity::new());
                }
                out
            }));
        }

        let mut entities = Vec::with_capacity(SIZE * threads);
        for handle in handles {
            for entity in handle.join().unwrap() {
                entities.push(entity);
            }
        }

        thread::spawn(move || {
            for i in 0..entities.len() {
                let mut j = 0;

                while j != i {
                    assert_ne!(entities[i], entities[j]);
                    j += 1;
                }
            }
        }).join().unwrap();
    }
}

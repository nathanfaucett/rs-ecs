use std::hash::{Hash, Hasher};

use hash_map::DefaultHasher;

use uuid::Uuid;


#[inline]
fn next_id() -> u64 {
    let mut hasher = DefaultHasher::default();
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

    #[inline(always)]
    pub fn new() -> Self {
        Entity {
            id: next_id(),
        }
    }
}


#[cfg(test)]
mod test {
    extern crate num_cpus;
    extern crate vector;
    extern crate collection_traits;
    extern crate std;


    use super::*;

    use self::std::thread;

    use self::collection_traits::*;
    use self::vector::Vector;


    static SIZE: usize = 1024usize;


    #[test]
    fn test_entity_id() {
        let threads = num_cpus::get() - 1usize;
        let mut handles = Vector::new();

        for _ in 0..threads {
            handles.push(thread::spawn(move || {
                let mut out = Vector::with_capacity(SIZE);
                for _ in 0..SIZE {
                    out.push(Entity::new());
                }
                out
            }));
        }

        let mut entities = Vector::with_capacity(SIZE * threads);
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

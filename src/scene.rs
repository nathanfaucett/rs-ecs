use std::sync::{Arc, RwLock};

use thread_pool::ThreadPool;
use waiter::Waiter;

use super::entity_manager::EntityManager;
use super::processes::Processes;


pub struct Scene {
    thread_pool: ThreadPool,
    entity_manager: EntityManager,
    processes: Arc<RwLock<Processes>>,
}

impl Scene {
    #[inline]
    pub fn new() -> Arc<Self> {
        Arc::new(Scene {
            thread_pool: ThreadPool::new(),
            entity_manager: EntityManager::new(),
            processes: Arc::new(RwLock::new(Processes::new())),
        })
    }

    #[inline]
    pub fn thread_pool(&self) -> &ThreadPool { &self.thread_pool }
    #[inline]
    pub fn thread_pool_mut(&mut self) -> &mut ThreadPool { &mut self.thread_pool }

    #[inline]
    pub fn entity_manager(&self) -> &EntityManager { &self.entity_manager }
    #[inline]
    pub fn processes(&self) -> &RwLock<Processes> { &*self.processes }

    #[inline]
    pub fn init(&self) -> &Self {
        self.processes.write().unwrap().sort();
        self
    }

    pub fn update(&self) -> &Self {
        let waiter = Waiter::new_with_count(self.processes.read().unwrap().len());

        for mut process in self.processes.write().unwrap().iter_mut() {
            let entity_manager = self.entity_manager.clone();
            let waiter = waiter.clone();

            let _ = self.thread_pool.run(move || {
                process.run(&entity_manager);
                let _ = waiter.done();
            });
        }

        let _ = waiter.wait();

        self.entity_manager.update();

        self
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use process::Process;


    const FRAMES: usize = 1024usize;


    macro_rules! create_process {
        ($name: ident, $p: expr) => (
            #[derive(Debug, Eq, PartialEq)]
            pub struct $name {
                count: usize,
                done: bool,
            }
            impl $name {
                fn new() -> Self {
                    $name {
                        count: 0,
                        done: false,
                    }
                }
            }
            impl Process for $name {
                fn run(&mut self, entity_manager: &EntityManager) {
                    let _ = entity_manager.create_entity();
                    self.count += 1;
                    self.done = self.count == FRAMES;
                }
                fn priority(&self) -> usize {
                    $p
                }
            }
        );
    }

    create_process!(Process0, 9);
    create_process!(Process1, 8);
    create_process!(Process2, 7);
    create_process!(Process3, 6);
    create_process!(Process4, 5);
    create_process!(Process5, 4);
    create_process!(Process6, 3);
    create_process!(Process7, 2);
    create_process!(Process8, 1);
    create_process!(Process9, 0);


    #[test]
    fn test_scene() {
        let scene = Scene::new();

        {
            let mut p = scene.processes().write().unwrap();
            p.insert(Process0::new());
            p.insert(Process1::new());
            p.insert(Process2::new());
            p.insert(Process3::new());
            p.insert(Process4::new());
            p.insert(Process5::new());
            p.insert(Process6::new());
            p.insert(Process7::new());
            p.insert(Process8::new());
            p.insert(Process9::new());
        }

        scene.init();

        for _ in 0..FRAMES {
            scene.update();
        }

        {
            let p = scene.processes().read().unwrap();
            assert!(p.process::<Process0>().unwrap().read().unwrap().done);
            assert!(p.process::<Process1>().unwrap().read().unwrap().done);
            assert!(p.process::<Process2>().unwrap().read().unwrap().done);
            assert!(p.process::<Process3>().unwrap().read().unwrap().done);
            assert!(p.process::<Process4>().unwrap().read().unwrap().done);
            assert!(p.process::<Process5>().unwrap().read().unwrap().done);
            assert!(p.process::<Process6>().unwrap().read().unwrap().done);
            assert!(p.process::<Process7>().unwrap().read().unwrap().done);
            assert!(p.process::<Process8>().unwrap().read().unwrap().done);
            assert!(p.process::<Process9>().unwrap().read().unwrap().done);
        }
    }
}

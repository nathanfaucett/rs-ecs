use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult};

use thread_pool::ThreadPool;

use entity_manager::EntityManager;
use process_manager::ProcessManager;


pub struct Scene {
    thread_pool: ThreadPool,
    process_manager: ProcessManager,
    entity_manager: Arc<RwLock<EntityManager>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            thread_pool: ThreadPool::new(),
            process_manager: ProcessManager::new(),
            entity_manager: Arc::new(RwLock::new(EntityManager::new())),
        }
    }

    pub fn thread_pool(&self) -> &ThreadPool { &self.thread_pool }
    pub fn thread_pool_mut(&mut self) -> &mut ThreadPool { &mut self.thread_pool }

    pub fn process_manager(&self) -> &ProcessManager { &self.process_manager }
    pub fn process_manager_mut(&mut self) -> &mut ProcessManager { &mut self.process_manager }

    pub fn entity_manager(&self) -> LockResult<RwLockReadGuard<EntityManager>> {
        self.entity_manager.read()
    }
    pub fn entity_manager_mut(&mut self) -> LockResult<RwLockWriteGuard<EntityManager>> {
        self.entity_manager.write()
    }

    pub fn update(&self) -> &Self {
        let mut handles = Vec::with_capacity(self.process_manager.len());

        for (_, process) in self.process_manager.processes_iter() {
            let entity_manager = self.entity_manager.clone();

            handles.push(self.thread_pool.run(move || {
                process.lock().unwrap().handle(&*entity_manager.read().unwrap());
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        if let Ok(entity_manager) = self.entity_manager.read() {
            for (_, components) in entity_manager.iter() {
                for (_, component) in components.iter() {
                    component.replace();
                }
            }
        }

        self
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use hierarchy::Hierarchy;
    use process::Process;


    static SIZE: usize = 1024usize;


    #[derive(Debug, Eq, PartialEq)]
    pub struct SomeProcess {
        done: bool,
    }

    impl SomeProcess {
        pub fn new() -> Self {
            SomeProcess {
                done: false,
            }
        }
    }

    impl Process for SomeProcess {
        fn handle(&mut self, entity_manager: &EntityManager) {
            let mut count = 0;
            for (_entity, _component) in entity_manager.component_iter::<Hierarchy>() {
                count += 1;
            }
            self.done = count == SIZE;
        }
    }


    #[test]
    fn test_scene() {
        let mut scene = Scene::new();

        for _ in 0..SIZE {
            scene.entity_manager_mut().unwrap().new_entity();
            scene.process_manager_mut().add_process(SomeProcess::new());
        }

        scene.update();

        for (_, process) in scene.process_manager().processes_iter() {
            assert_eq!(process.lock().unwrap().downcast_ref::<SomeProcess>().unwrap().done, true);
        }
    }

    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_scene(b: &mut Bencher) {
        let mut scene = Scene::new();

        for _ in 0..SIZE {
            scene.entity_manager_mut().unwrap().new_entity();
            scene.process_manager_mut().add_process(SomeProcess::new());
        }

        b.iter(|| {
            scene.update()
        });
    }
}

use std::sync::{Arc, RwLock};

use thread_pool::ThreadPool;

use entity_manager::EntityManager;
use process_manager::ProcessManager;


pub fn update(
    thread_pool: &ThreadPool,
    process_manager: &ProcessManager,
    entity_manager: Arc<RwLock<EntityManager>>
) {
    let mut handles = Vec::with_capacity(process_manager.len());

    for (_, process) in process_manager.processes_iter() {
        let p = process.clone();
        let em = entity_manager.clone();

        handles.push(thread_pool.run(move || {
            p.lock().unwrap().handle(&*em.read().unwrap());
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    if let Ok(entity_manager) = entity_manager.read() {
        for (_, components) in entity_manager.iter() {
            for (_, component) in components.iter() {
                component.replace();
            }
        }
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
        let thread_pool = ThreadPool::new();
        let entity_manager = Arc::new(RwLock::new({
            let mut em = EntityManager::new();
            for _ in 0..SIZE {
                em.new_entity();
            }
            em
        }));
        let mut process_manager = ProcessManager::new();
        process_manager.add_process(SomeProcess::new());

        update(&thread_pool, &process_manager, entity_manager);

        let process = process_manager.process::<SomeProcess>().unwrap();
        assert_eq!(process.done, true);
    }
}

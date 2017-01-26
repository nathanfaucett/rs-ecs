use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use thread_pool::ThreadPool;

use components::Components;
use entities::Entities;
use processes::Processes;


pub struct Scene {
    thread_pool: ThreadPool,
    components: Arc<RwLock<Components>>,
    entities: Arc<RwLock<Entities>>,
    processes: Arc<RwLock<Processes>>,
}

impl Scene {
    pub fn new() -> Arc<Self> {
        Arc::new(Scene {
            thread_pool: ThreadPool::new(),
            components: Arc::new(RwLock::new(Components::new())),
            entities: Arc::new(RwLock::new(Entities::new())),
            processes: Arc::new(RwLock::new(Processes::new())),
        })
    }

    pub fn thread_pool(&self) -> &ThreadPool { &self.thread_pool }
    pub fn thread_pool_mut(&mut self) -> &mut ThreadPool { &mut self.thread_pool }

    pub fn components(&self) -> &RwLock<Components> { &*self.components }
    pub fn entities(&self) -> &RwLock<Entities> { &*self.entities }
    pub fn processes(&self) -> &RwLock<Processes> { &*self.processes }

    pub fn update(&self) -> &Self{
        let current_thread = Arc::new(thread::current());
        let count = Arc::new(AtomicUsize::new(self.processes.read().unwrap().len()));
    
        for mut process in self.processes.write().unwrap().iter_mut() {
            let current_thread = current_thread.clone();
            let count = count.clone();
            let components = self.components.clone();
            let entities = self.entities.clone();
    
            let _ = self.thread_pool.run(move || {
                process.run(&*components, &*entities);
                count.fetch_sub(1, Ordering::Relaxed);
                current_thread.unpark();
            });
        }
    
        while count.load(Ordering::Relaxed) != 0 {
            thread::park();
        }
        
        self
    }
}


#[cfg(test)]
mod test {
    use std::sync::RwLock;

    use super::*;
    use process::Process;


    macro_rules! create_process {
        ($name: ident) => (
            #[derive(Debug, Eq, PartialEq)]
            pub struct $name {
                done: bool,
            }
            impl $name {
                fn new() -> Self {
                    $name { 
                        done: false,
                    }
                }
            }
            impl Process for $name {
                fn run(&mut self, _: &RwLock<Components>, entities: &RwLock<Entities>) {
                    let _ = entities.write().unwrap().create();
                    self.done = true;
                }
            }
        );
    }

    create_process!(Process0);
    create_process!(Process1);
    create_process!(Process2);


    #[test]
    fn test_scene() {
        let scene = Scene::new();
        
        {
            let mut p =scene.processes().write().unwrap();
            p.insert(Process0::new());
            p.insert(Process1::new());
            p.insert(Process2::new());
        }

        scene.update();
        
        {
            let p = scene.processes().read().unwrap();
            assert!(p.process::<Process0>().unwrap().read().unwrap().done);
            assert!(p.process::<Process1>().unwrap().read().unwrap().done);
            assert!(p.process::<Process2>().unwrap().read().unwrap().done);
        }
    }
}

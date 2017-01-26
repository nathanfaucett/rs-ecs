use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use thread_pool::ThreadPool;

use components::Components;
use entities::Entities;
use processes::Processes;


pub struct Scene {
    thread_pool: ThreadPool,
    components: Components,
    entities: Entities,
    processes: Processes,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            thread_pool: ThreadPool::new(),
            components: Components::new(),
            entities: Entities::new(),
            processes: Processes::new(),
        }
    }

    pub fn thread_pool(&self) -> &ThreadPool { &self.thread_pool }
    pub fn thread_pool_mut(&mut self) -> &mut ThreadPool { &mut self.thread_pool }

    pub fn components(&self) -> &Components { &self.components }
    pub fn components_mut(&mut self) -> &mut Components { &mut self.components }

    pub fn entities(&self) -> &Entities { &self.entities }
    pub fn entities_mut(&mut self) -> &mut Entities { &mut self.entities }

    pub fn processes(&self) -> &Processes { &self.processes }
    pub fn processes_mut(&mut self) -> &mut Processes { &mut self.processes }

    pub fn update(&mut self) -> &Self {
        let current_thread = Arc::new(thread::current());
        let count = Arc::new(AtomicUsize::new(self.processes.len()));

        for mut process in self.processes.iter_mut() {
            let current_thread = current_thread.clone();
            let count = count.clone();

            let _ = self.thread_pool.run(move || {
                process.run();
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
    use super::*;
    use process::Process;


    macro_rules! create_process {
        ($name: ident) => (
            #[derive(Debug, Eq, PartialEq)]
            pub struct $name {
                done: bool,
            }
            impl Process for $name {
                fn run(&mut self) {
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
        let mut scene = Scene::new();

        scene.processes_mut().insert(Process0 {done: false});
        scene.processes_mut().insert(Process1 {done: false});
        scene.processes_mut().insert(Process2 {done: false});

        scene.update();

        assert!(scene .processes().process::<Process0>().unwrap() .read().unwrap().done);
        assert!(scene .processes().process::<Process1>().unwrap() .read().unwrap().done);
        assert!(scene .processes().process::<Process2>().unwrap() .read().unwrap().done);
    }
}

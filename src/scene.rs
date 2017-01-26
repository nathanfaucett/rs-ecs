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

        for mut process in self.processes.iter_mut() {
            let _ = self.thread_pool.run(move || {
                process.run();
            });
        }

        self
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use process::Process;


    #[derive(Debug, Eq, PartialEq)]
    pub struct SomeProcess;

    impl Process for SomeProcess {
        fn run(&mut self) {}
    }


    #[test]
    fn test_scene() {
        let mut scene = Scene::new();

        for _ in 0..32{
            scene.processes_mut().insert(SomeProcess);
        }

        scene.update();
    }
}

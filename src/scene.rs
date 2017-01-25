use thread_pool::ThreadPool;


pub struct Scene {
    thread_pool: ThreadPool,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            thread_pool: ThreadPool::new(),
        }
    }

    pub fn thread_pool(&self) -> &ThreadPool { &self.thread_pool }
    pub fn thread_pool_mut(&mut self) -> &mut ThreadPool { &mut self.thread_pool }

    pub fn update(&mut self) -> &Self {
        self
    }
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_scene() {
        let mut scene = Scene::new();
        scene.update();
    }
}

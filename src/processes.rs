use std::any::{Any, TypeId};
use std::collections::{hash_map, HashMap};
use std::sync::{Arc, RwLock};

use process::Process;


pub struct Processes {
    processes: HashMap<TypeId, Box<ProcessLock>>,
}

unsafe impl Send for Processes {}
unsafe impl Sync for Processes {}

impl Processes {
    pub fn new() -> Self {
        Processes {
            processes: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }

    pub fn contains<T: Process>(&mut self) -> bool {
        self.processes.contains_key(&TypeId::of::<T>())
    }

    pub fn insert<T: Process>(&mut self, process: T) {
        self.processes.insert(TypeId::of::<T>(), Box::new(Arc::new(RwLock::new(process))));
    }
    pub fn remove<T: Process>(&mut self) -> Option<T> {
        None
        /*
        match self.remove_by_type_id(&TypeId::of::<T>()) {
            Some(process_lock) => match unsafe {
                process_lock.downcast_unchecked::<RwLock<T>>().into_inner()
            } {
                Ok(process) => Some(process),
                Err(..) => None,
            },
            None => None,
        }
        */
    }
    pub fn remove_by_type_id(&mut self, type_id: &TypeId) -> Option<Box<ProcessLock>> {
        self.processes.remove(&type_id)
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self.processes.iter_mut())
    }
}


pub struct IterMut<'a> {
    iter: hash_map::IterMut<'a, TypeId, Box<ProcessLock>>,
}

impl<'a> IterMut<'a> {
    fn new(iter: hash_map::IterMut<'a, TypeId, Box<ProcessLock>>) -> Self {
        IterMut {
            iter: iter,
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = Box<ProcessLock>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((_type_id, next)) => Some(next.clone_()),
            None => None,
        }
    }
}


pub trait ProcessLock: Any + Send + Sync {
    fn run(&mut self);
    fn clone_(&self) -> Box<ProcessLock>;
}

impl_any!(ProcessLock);

impl<T: Process> ProcessLock for Arc<RwLock<T>> {
    fn run(&mut self) {
        self.write().unwrap().run();
    }
    fn clone_(&self) -> Box<ProcessLock> {
        Box::new(self.clone())
    }
}


#[cfg(test)]
mod test {
    use super::*;


    #[derive(Debug, Eq, PartialEq)]
    pub struct SomeProcess;

    impl Process for SomeProcess {
        fn run(&mut self) {}
    }


    #[test]
    fn test_process() {
        let mut processes = Processes::new();
        processes.insert(SomeProcess);

        //let process = processes.remove::<SomeProcess>().unwrap();
        //assert_eq!(process, SomeProcess);
    }
}

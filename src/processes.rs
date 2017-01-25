use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;

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
    pub fn processes(&self) -> &HashMap<TypeId, Box<ProcessLock>> {
        &self.processes
    }

    pub fn contains<T: Process>(&mut self) -> bool {
        self.processes.contains_key(&TypeId::of::<T>())
    }

    pub fn insert<T: Process>(&mut self, process: T) {
        self.processes.insert(TypeId::of::<T>(), Box::new(RwLock::new(process)));
    }
    pub fn remove<T: Process>(&mut self) -> Option<T> {
        match self.remove_by_type_id(&TypeId::of::<T>()) {
            Some(process_lock) => match unsafe {
                process_lock.downcast_unchecked::<RwLock<T>>().into_inner()
            } {
                Ok(process) => Some(process),
                Err(..) => None,
            },
            None => None,
        }
    }
    pub fn remove_by_type_id(&mut self, type_id: &TypeId) -> Option<Box<ProcessLock>> {
        self.processes.remove(&type_id)
    }
}


pub trait ProcessLock: Any + Send + Sync {}

impl_any!(ProcessLock);

impl<T: Process> ProcessLock for RwLock<T> {}


#[cfg(test)]
mod test {
    use super::*;


    #[derive(Debug, Eq, PartialEq)]
    pub struct SomeProcess;

    impl Process for SomeProcess {
        fn handle(&mut self) {}
    }


    #[test]
    fn test_process() {
        let mut processes = Processes::new();
        processes.insert(SomeProcess);

        let process = processes.remove::<SomeProcess>().unwrap();
        assert_eq!(process, SomeProcess);
    }
}

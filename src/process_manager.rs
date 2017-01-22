use std::any::TypeId;
use std::marker::PhantomData;
use std::collections::{hash_map, HashMap};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};

use process::Process;


pub struct ProcessManager {
    processes: HashMap<TypeId, Arc<Mutex<Box<Process>>>>,
}

unsafe impl Send for ProcessManager {}
unsafe impl Sync for ProcessManager {}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize { self.processes.len() }
    pub fn processes(&self) -> &HashMap<TypeId, Arc<Mutex<Box<Process>>>> { &self.processes }
    pub fn processes_iter(&self) -> ProcessIter {
        ProcessIter::new(self.processes.iter())
    }
    pub fn type_ids(&mut self) -> Vec<TypeId> {
        let mut type_ids = Vec::with_capacity(self.processes.len());

        for (type_id, _) in self.processes.iter() {
            type_ids.push(*type_id);
        }
        type_ids
    }

    pub fn add_process<T: Process + 'static>(&mut self, process: T) {
        self.processes.insert(TypeId::of::<T>(), Arc::new(Mutex::new(Box::new(process))));
    }

    pub fn remove_process<T: Process + 'static>(&mut self) -> Option<T> {
        match self.remove_process_by_type_id(&TypeId::of::<T>()) {
            Some(process) => Some(unsafe {
                let raw: *mut Process = Box::into_raw(process);
                *Box::from_raw(raw as *mut T)
            }),
            None => None,
        }
    }
    pub fn remove_process_by_type_id(&mut self, type_id: &TypeId) -> Option<Box<Process>> {
        match self.processes.remove(&type_id) {
            Some(arc) => match Arc::try_unwrap(arc) {
                Ok(mutex) => match mutex.into_inner() {
                    Ok(process) => Some(process),
                    Err(..) => None,
                },
                Err(..) => None,
            },
            None => None,
        }
    }

    pub fn process<T: Process + 'static>(&self) -> Option<Guard<T>> {
        match self.processes.get(&TypeId::of::<T>()) {
            Some(arc) => match arc.lock() {
                Ok(process) => Some(Guard::new(process)),
                Err(..) => None,
            },
            None => None,
        }
    }
}


pub struct ProcessIter<'a> {
    iter: hash_map::Iter<'a, TypeId, Arc<Mutex<Box<Process>>>>,
}

impl<'a> ProcessIter<'a> {
    fn new(iter: hash_map::Iter<'a, TypeId, Arc<Mutex<Box<Process>>>>) -> Self {
        ProcessIter {
            iter: iter,
        }
    }
}

impl<'a> Iterator for ProcessIter<'a> {
    type Item = (TypeId, Arc<Mutex<Box<Process>>>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((type_id, process)) => Some((*type_id, process.clone())),
            None => None,
        }
    }
}


pub struct Guard<'a, T: 'static> {
    phantom_data: PhantomData<T>,
    mutex_guard: MutexGuard<'a, Box<Process>>,
}

impl<'a, T: 'static> Guard<'a, T> {
    fn new(mutex_guard: MutexGuard<'a, Box<Process>>) -> Self {
        Guard {
            phantom_data: PhantomData,
            mutex_guard: mutex_guard,
        }
    }
}

impl<'a, T: 'static> Deref for Guard<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.mutex_guard.downcast_ref::<T>().unwrap()
    }
}
impl<'a, T: 'static> DerefMut for Guard<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mutex_guard.downcast_mut::<T>().unwrap()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use entity_manager::EntityManager;


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
        fn handle(&mut self, _entity_manager: &EntityManager) {
            self.done = true;
        }
    }


    #[test]
    fn test_process() {
        let mut process_manager = ProcessManager::new();

        process_manager.add_process(SomeProcess::new());
        assert_eq!(process_manager.process::<SomeProcess>().unwrap().done, false);

        process_manager.remove_process::<SomeProcess>().unwrap();
        assert!(process_manager.process::<SomeProcess>().is_none());
    }

    #[test]
    fn test_process_trait() {
        let entity_manager = EntityManager::new();
        let mut process_manager = ProcessManager::new();

        process_manager.add_process(SomeProcess::new());
        process_manager.process::<SomeProcess>().unwrap().handle(&entity_manager);

        assert_eq!(process_manager.process::<SomeProcess>().unwrap().done, true);

        let process = process_manager.remove_process::<SomeProcess>().unwrap();
        assert_eq!(process.done, true);
    }
}

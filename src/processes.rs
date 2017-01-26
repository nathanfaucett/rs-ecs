use std::any::{Any, TypeId};
use std::sync::{Arc, RwLock};
use std::slice;

use process::Process;
use components::Components;
use entities::Entities;


pub struct Processes {
    processes: Vec<(TypeId, Box<ProcessLock>)>,
}

unsafe impl Send for Processes {}
unsafe impl Sync for Processes {}

impl Processes {
    pub fn new() -> Self {
        Processes {
            processes: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }
    pub fn sort(&mut self) {
        self.processes.sort_by(|&(_, ref a), &(_, ref b)| {
            a.priority().cmp(&b.priority())
        });
    }

    #[inline]
    fn index_of(&self, type_id: &TypeId) -> Option<usize> {
        self.processes.iter().position(|&(t, _)| &t == type_id)
    }
    pub fn process<T: Process>(&self) -> Option<&Arc<RwLock<T>>> {
        match self.index_of(&TypeId::of::<T>()) {
            Some(index) => Some(unsafe {
                let &(_, ref process) = self.processes.get_unchecked(index);
                process.downcast_ref_unchecked::<Arc<RwLock<T>>>()
            }),
            None => None,
        }
    }
    pub fn contains<T: Process>(&self) -> bool {
        self.index_of(&TypeId::of::<T>()).is_some()
    }

    pub fn insert<T: Process>(&mut self, process: T) {
        self.processes.push((
            TypeId::of::<T>(),
            Box::new(Arc::new(RwLock::new(process)))
        ));
    }
    pub fn remove<T: Process>(&mut self) -> Option<T> {
        match self.remove_by_type_id(&TypeId::of::<T>()) {
            Some(process_lock) => match Arc::try_unwrap(unsafe {
                *process_lock.downcast_unchecked::<Arc<RwLock<T>>>()
            }) {
                Ok(rwlock) => match rwlock.into_inner() {
                    Ok(process) => Some(process),
                    Err(..) => None,
                },
                Err(..) => None,
            },
            None => None,
        }
    }
    pub fn remove_by_type_id(&mut self, type_id: &TypeId) -> Option<Box<ProcessLock>> {
        match self.index_of(type_id) {
            Some(index) => {
                let (_type_id, process) = self.processes.remove(index);
                Some(process)
            },
            None => None,
        }
    }

    pub fn raw(&self) -> &Vec<(TypeId, Box<ProcessLock>)> {
        &self.processes
    }
    pub fn raw_mut(&mut self) -> &mut Vec<(TypeId, Box<ProcessLock>)> {
        &mut self.processes
    }

    pub fn iter(&mut self) -> Iter {
        Iter::new(self.processes.iter())
    }
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self.processes.iter_mut())
    }
}


pub struct Iter<'a> {
    iter: slice::Iter<'a, (TypeId, Box<ProcessLock>)>,
}
impl<'a> Iter<'a> {
    fn new(iter: slice::Iter<'a, (TypeId, Box<ProcessLock>)>) -> Self {
        Iter {
            iter: iter,
        }
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = Box<ProcessLock>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(&(_, ref next)) => Some(next.clone_()),
            None => None,
        }
    }
}


pub struct IterMut<'a> {
    iter: slice::IterMut<'a, (TypeId, Box<ProcessLock>)>,
}
impl<'a> IterMut<'a> {
    fn new(iter: slice::IterMut<'a, (TypeId, Box<ProcessLock>)>) -> Self {
        IterMut {
            iter: iter,
        }
    }
}
impl<'a> Iterator for IterMut<'a> {
    type Item = Box<ProcessLock>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(&mut (_, ref next)) => Some(next.clone_()),
            None => None,
        }
    }
}


pub trait ProcessLock: Any + Send + Sync {
    fn run(&mut self, &RwLock<Components>, &RwLock<Entities>);
    fn clone_(&self) -> Box<ProcessLock>;
    fn priority(&self) -> usize;
}

impl_any!(ProcessLock);

impl<T: Process> ProcessLock for Arc<RwLock<T>> {
    fn run(&mut self, components: &RwLock<Components>, entities: &RwLock<Entities>) {
        self.write().unwrap().run(components, entities);
    }
    fn clone_(&self) -> Box<ProcessLock> {
        Box::new(self.clone())
    }
    fn priority(&self) -> usize {
        self.read().unwrap().priority()
    }
}


#[cfg(test)]
mod test {
    use std::sync::RwLock;

    use super::*;
    use components::Components;
    use entities::Entities;


    #[derive(Debug, Eq, PartialEq)]
    pub struct SomeProcess;

    impl Process for SomeProcess {
        fn run(&mut self, _: &RwLock<Components>, _: &RwLock<Entities>) {}
    }


    #[test]
    fn test_process() {
        let mut processes = Processes::new();
        processes.insert(SomeProcess);

        let process = processes.remove::<SomeProcess>().unwrap();
        assert_eq!(process, SomeProcess);
    }
}

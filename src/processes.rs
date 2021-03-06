use std::any::{Any, TypeId};
use std::sync::{Arc, RwLock};
use std::slice;

use collection_traits::*;
use vector::Vector;

use super::process::Process;
use super::entity_manager::EntityManager;


pub struct Processes {
    processes: Vector<(TypeId, Box<ProcessLock>)>,
}

unsafe impl Send for Processes {}
unsafe impl Sync for Processes {}

impl Processes {
    #[inline]
    pub fn new() -> Self {
        Processes {
            processes: Vector::new(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.processes.len()
    }
    #[inline]
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
    #[inline]
    pub fn contains<T: Process>(&self) -> bool {
        self.index_of(&TypeId::of::<T>()).is_some()
    }

    #[inline]
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

    #[inline]
    pub fn raw(&self) -> &Vector<(TypeId, Box<ProcessLock>)> {
        &self.processes
    }
    #[inline]
    pub fn raw_mut(&mut self) -> &mut Vector<(TypeId, Box<ProcessLock>)> {
        &mut self.processes
    }

    #[inline]
    pub fn iter(&mut self) -> Iter {
        Iter::new(self.processes.iter())
    }
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self.processes.iter_mut())
    }
}


pub struct Iter<'a> {
    iter: slice::Iter<'a, (TypeId, Box<ProcessLock>)>,
}
impl<'a> Iter<'a> {
    #[inline]
    fn new(iter: slice::Iter<'a, (TypeId, Box<ProcessLock>)>) -> Self {
        Iter {
            iter: iter,
        }
    }
}
impl<'a> Iterator for Iter<'a> {
    type Item = Box<ProcessLock>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(&(_, ref next)) => Some(next.clone_as_box()),
            None => None,
        }
    }
}


pub struct IterMut<'a> {
    iter: slice::IterMut<'a, (TypeId, Box<ProcessLock>)>,
}
impl<'a> IterMut<'a> {
    #[inline]
    fn new(iter: slice::IterMut<'a, (TypeId, Box<ProcessLock>)>) -> Self {
        IterMut {
            iter: iter,
        }
    }
}
impl<'a> Iterator for IterMut<'a> {
    type Item = Box<ProcessLock>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(&mut (_, ref next)) => Some(next.clone_as_box()),
            None => None,
        }
    }
}


pub trait ProcessLock: Any + Send + Sync {
    fn run(&mut self, &EntityManager);
    fn clone_as_box(&self) -> Box<ProcessLock>;
    fn priority(&self) -> usize;
}

impl_any!(ProcessLock);

impl<T: Process> ProcessLock for Arc<RwLock<T>> {
    #[inline]
    fn run(&mut self, entity_manager: &EntityManager) {
        self.write().unwrap().run(entity_manager);
    }
    #[inline]
    fn clone_as_box(&self) -> Box<ProcessLock> {
        Box::new(self.clone())
    }
    #[inline]
    fn priority(&self) -> usize {
        self.read().unwrap().priority()
    }
}


#[cfg(test)]
mod test {
    use super::*;


    #[derive(Debug, Eq, PartialEq)]
    pub struct SomeProcess;

    impl Process for SomeProcess {
        fn run(&mut self, _: &EntityManager) {}
    }


    #[test]
    fn test_process() {
        let mut processes = Processes::new();
        processes.insert(SomeProcess);

        let process = processes.remove::<SomeProcess>().unwrap();
        assert_eq!(process, SomeProcess);
    }
}

use std::{marker::PhantomData, sync::atomic::AtomicUsize};
use std::sync::Arc;
use std::iter::Iterator;

use crate::backends::{Backend, Backends};

//#[derive(Debug, Copy, Clone)]
//struct Backend {
//    value: usize,
//    weight: usize
//}
//
//impl Backend {
//    fn new(value: usize, weight: usize) -> Self {
//        Self {
//            value,
//            weight
//        }
//    }
//}
//
//type Backends = Vec<Backend>;

/// Factory for generating iterators
trait IterFactory {
    type Iter;

    fn build(backends: &Backends) -> Self;
    
    fn iter(self: Arc<Self>) -> Self::Iter
        where
            Self::Iter: Iterator; 
}

struct Index(AtomicUsize);

impl Index {
    fn next(&self) -> u64 {
        self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed) as u64
    }
}

struct StatefulIterFactory {
    backends: Box<&Backends>,
    index: Index,
}

impl IterFactory for StatefulIterFactory {
    type Iter = StatefulIter;

    fn build(backends: &Backends) -> Self {
        Self {
            backends: Box::new(backends),
            index: Index(AtomicUsize::new(0)),
            phantom: PhantomData,
        }
    }

    fn iter(self: Arc<Self>) -> Self::Iter {
        StatefulIter::new(self.clone())
    }
}

struct StatefulIter<'a> {
    // selector: H
    state: Arc<StatefulIterFactory<'a>>,
    index: usize
}

impl<'a> StatefulIter<'a> {
    fn new(state: Arc<StatefulIterFactory<'a>>) -> Self {
        Self {
            state,
            index: 0
        }
    } 
}

impl<'a> Iterator for StatefulIter<'a> {
    type Item = Backend;
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.state.index.next() as usize;
        Some(self.state.backends[self.index].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let backends: Backends = Vec::from([Backend::new(0, 1), Backend::new(1, 1)]);
        let factory = Arc::new(StatefulIterFactory::build(&backends));
        let mut iter = factory.clone().iter();
        let be = iter.next().unwrap();
        println!("value = {:?}", be);
        let mut iter = factory.iter();
        let be = iter.next().unwrap();
        println!("value = {:?}", be);

    }
}

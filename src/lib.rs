//mod std_iterator;
mod backends;

use std::iter;
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use backends::{Backend, Backends};

// Algorithm is a iterator returning
trait SelectionAlgorithm {
    fn next(&self) -> Option<usize>;
}

struct RoundRobin(AtomicUsize);

impl SelectionAlgorithm for RoundRobin {
    fn next(&self) -> Option<usize> {
        Some(self.0.fetch_add(1, Ordering::Relaxed))
    }
}


/// A weighted selection algorithm iterator
/// backends is a set of backends
/// project is the projects of the backends according to their weight
/// algorithm is used to make a selection from backnds, by itterating over the projection accoring
/// to the a algorithm, e.g. RR, Random, etc.
///
///
/// be = [(0,1),(1,2),(2,3)]
/// projection = [0,1,1,2,2,2]
/// 
/// Round Robin yields = {0, 1, 1, 2, 2, 2, 0, 1, 1...}
struct SelectionAlgorithmState<'a, A> {
    /// Set of backends
    backends: Box<[Backend]>,
    /// Indexes projected according to backend weights
    projection: Vec<u16>,
    /// Selection Algorithm
    algorithm: A,
    phantom: PhantomData<&'a Backend>
}

// Is it, is it wicked...?
/// A selector algorithm 
trait Selector<'a> {
    fn select(&'a self) -> Option<&'a Backend>;
}

impl<'a, A: SelectionAlgorithm> Selector<'a> for SelectionAlgorithmState<'a, A> {
    fn select(&'a self) -> Option<&'a Backend> {
        let choice = self.algorithm.next().unwrap();
        let len = self.projection.len();
        let idx = self.projection[choice % len];
        Some(&self.backends[idx as usize])
    }
}


/// Projects backends into a vector according to their weight
fn project(backends: &[Backend]) -> Vec<u16> {
    backends
        .iter()
        .enumerate()
        .flat_map(|(idx, be)|{
            assert!(idx <= u16::MAX as usize, "Index exceeds u16::MAX");
            iter::repeat(idx as u16).take(be.weight) 
        })
        .collect()
}

// Iterate over AlgorithState
#[allow(dead_code)]
impl<'a, A> SelectionAlgorithmState<'a, A>
    where
        A: SelectionAlgorithm
{
    pub fn new(backends: &Backends, algorithm: A) -> Self {
        let projection = project(backends);
        let backends = backends.clone().into_boxed_slice();
        Self {
            backends,
            projection,
            algorithm,
            phantom: PhantomData
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        let backends: Backends = Vec::from([Backend::new(0, 1), Backend::new(1, 2)]);
        let projection = project(&backends.clone());
        assert_eq!(projection, [0, 1, 1])
    }

    #[test]
    fn test_statefule_iter() {
        let backends: Backends = Vec::from([Backend::new(0, 1), Backend::new(1, 1)]);

        let algorithm = RoundRobin(AtomicUsize::new(0));
        let sa = SelectionAlgorithmState::new(&backends, algorithm);

        let b1 = sa.select().unwrap();
        let b2 = sa.select().unwrap();
        let b3 = sa.select().unwrap();

        println!("{:?}", b1);
        println!("{:?}", b2);
        println!("{:?}", b3);

        assert_eq!(0, b1.value);
        assert_eq!(1, b2.value);
        assert_eq!(0, b3.value);
    }


    //#[test]
    //fn test_iter() {
    //    let backends: Backends = Vec::from([Backend::new(0, 1), Backend::new(1, 1)]);
    //    let factory = Arc::new(StatefulIterFactory::build(&backends));
    //    let mut iter = factory.clone().iter();
    //    let be = iter.next().unwrap();
    //    println!("value = {:?}", be);
    //    let mut iter = factory.iter();
    //    let be = iter.next().unwrap();
    //    println!("value = {:?}", be);
    //}
}

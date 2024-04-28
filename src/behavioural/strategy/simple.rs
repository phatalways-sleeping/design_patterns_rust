use std::collections::LinkedList;

use super::{Coordinator, Navigator, Path};

pub struct BFS;

impl Navigator for BFS {
    fn compute(&self, _start: Coordinator, _end: Coordinator) -> Path {
        Path {
            length: 32,
            details: LinkedList::new(),
        }
    }
}

pub struct DFS;

impl Navigator for DFS {
    fn compute(&self, _start: Coordinator, _end: Coordinator) -> Path {
        Path {
            length: 32,
            details: LinkedList::new(),
        }
    }
}

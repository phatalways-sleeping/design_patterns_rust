use std::collections::LinkedList;

use super::{Coordinator, Navigator, Path};

pub struct Dijktra;

impl Navigator for Dijktra {
    fn compute(&self, _start: Coordinator, _end: Coordinator) -> Path {
        Path {
            length: 32,
            details: LinkedList::new(),
        }
    }
}

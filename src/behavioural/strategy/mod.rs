use std::collections::LinkedList;

use self::dijktra::Dijktra;

pub mod dijktra;
pub mod simple;

pub struct Coordinator {
    pub longitude: f32,
    pub latitude: f32,
}

pub struct Path {
    pub length: u32,
    pub details: LinkedList<Coordinator>,
}

pub trait Navigator {
    fn compute(&self, start: Coordinator, end: Coordinator) -> Path;
}

pub struct Map {
    navigator: Box<dyn Navigator>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            navigator: Box::new(Dijktra),
        }
    }
}

impl Map {
    pub fn route(&self, start: Coordinator, end: Coordinator) -> Path {
        self.navigator.compute(start, end)
    }
    pub fn change_strategy(&mut self, navigator: Box<dyn Navigator>) {
        self.navigator = navigator;
    }
}

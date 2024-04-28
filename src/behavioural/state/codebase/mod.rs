use std::slice::Iter;

use super::testing::{Test, TestResult};

pub mod e_commerce;

pub trait Codebase {
    fn run(&self, tests: Iter<Test>) -> Vec<TestResult>;
}



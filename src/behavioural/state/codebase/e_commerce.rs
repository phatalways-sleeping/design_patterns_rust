use std::slice::Iter;

use crate::behavioural::state::testing::{Test, TestResult};

use super::Codebase;

pub struct ECommerceSrcCode {
    pub source: String,
}

impl Codebase for ECommerceSrcCode {
    fn run(&self, tests: Iter<Test>) -> Vec<TestResult> {
        tests
            .into_iter()
            .map(|test| test.execute(&self.source))
            .collect::<Vec<_>>()
    }
}

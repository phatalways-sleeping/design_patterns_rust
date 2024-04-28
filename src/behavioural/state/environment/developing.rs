use crate::behavioural::state::{codebase::Codebase, testing::Test};

use super::{staging::Staging, Environment, SuccessKind};

pub struct Developing {
    pub tests: Vec<Test>,
}

impl Developing {
    pub fn new() -> Box<dyn Environment> {
        Box::new(Developing { tests: vec![] })
    }
}

impl Environment for Developing {
    fn push(&self, codebase: &dyn Codebase) -> Result<SuccessKind, &'static str> {
        let results = codebase.run(self.tests.iter());
        if results.iter().find(|r| !r.is_successful()).is_some() {
            Err("Error: Some tests have failed in developing environment")
        } else {
            // Move to staging
            Ok(SuccessKind::Normal(Staging::new()))
        }
    }

    fn back(&self, _codebase: &dyn Codebase) -> Result<SuccessKind, &'static str> {
        Err("Error: Developing")
    }
}

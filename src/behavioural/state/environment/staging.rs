use crate::behavioural::state::{codebase::Codebase, testing::Test};

use super::{developing::Developing, production::Production, Environment, SuccessKind};

pub struct Staging {
    pub tests: Vec<Test>,
}

impl Staging {
    pub fn new() -> Box<dyn Environment> {
        Box::new(Staging { tests: vec![] })
    }
}

impl Environment for Staging {
    fn push(&self, codebase: &dyn Codebase) -> Result<SuccessKind, &'static str> {
        let results = codebase.run(self.tests.iter());
        if results.iter().find(|r| !r.is_successful()).is_some() {
            Err("Error: Some tests has failed in staging environment")
        } else {
            // Move to production
            Ok(SuccessKind::Normal(Production::new()))
        }
    }

    fn back(&self, _codebase: &dyn Codebase) -> Result<SuccessKind, &'static str> {
        Ok(SuccessKind::Normal(Developing::new()))
    }
}

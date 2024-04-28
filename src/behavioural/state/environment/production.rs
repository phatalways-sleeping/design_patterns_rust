use crate::behavioural::state::codebase::Codebase;

use super::{staging::Staging, Environment, SuccessKind};

pub struct Production;

impl Production {
    pub fn new() -> Box<dyn Environment> {
        Box::new(Production)
    }
}

impl Environment for Production {
    fn push(&self, _codebase: &dyn Codebase) -> Result<SuccessKind, &'static str> {
        Ok(SuccessKind::End)
    }

    fn back(&self, _codebase: &dyn Codebase) -> Result<SuccessKind, &'static str> {
        Ok(SuccessKind::Normal(Staging::new()))
    }
}

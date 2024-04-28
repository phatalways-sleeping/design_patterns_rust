use super::codebase::Codebase;

pub mod developing;
pub mod production;
pub mod staging;

pub enum SuccessKind {
    Normal(Box<dyn Environment>),
    End,
}

pub trait Environment {
    fn push(&self, codebase: &dyn Codebase) -> Result<SuccessKind, &'static str>;
    fn back(&self, codebase: &dyn Codebase) -> Result<SuccessKind, &'static str>;
}

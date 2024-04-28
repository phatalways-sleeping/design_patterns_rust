pub trait Component {
    fn execute(&self);
}

pub trait Button: Component {}

pub trait Checkbox: Component {}

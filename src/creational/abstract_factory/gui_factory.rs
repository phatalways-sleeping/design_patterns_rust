use super::components::{Button, Checkbox};

pub trait GUIFactory {
    fn create_button(value: String) -> impl Button;
    fn create_checkbox(value: bool) -> impl Checkbox;
}
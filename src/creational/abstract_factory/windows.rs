use super::{
    components::{Button, Checkbox, Component},
    gui_factory::GUIFactory,
};

pub struct WindowGUIFactory;

impl GUIFactory for WindowGUIFactory {
    fn create_button(value: String) -> impl Button {
        WindowButton(value)
    }

    fn create_checkbox(value: bool) -> impl Checkbox {
        WindowCheckbox(value)
    }
}

pub struct WindowButton(pub String);

impl Component for WindowButton {
    fn execute(&self) {
        println!("WindowButton - {} -> Clicked", self.0)
    }
}

impl Button for WindowButton {}

pub struct WindowCheckbox(pub bool);

impl Component for WindowCheckbox {
    fn execute(&self) {
        println!("WindowCheckbox - {} -> Toggle", self.0)
    }
}

impl Checkbox for WindowCheckbox {}

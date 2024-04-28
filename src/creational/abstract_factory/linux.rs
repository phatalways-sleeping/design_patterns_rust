use super::{
    components::{Button, Checkbox, Component},
    gui_factory::GUIFactory,
};

pub struct LinuxGUIFactory;

impl GUIFactory for LinuxGUIFactory {
    fn create_button(value: String) -> impl Button {
        LinuxButton(value)
    }

    fn create_checkbox(value: bool) -> impl Checkbox {
        LinuxCheckbox(value)
    }
}

pub struct LinuxButton(pub String);

impl Component for LinuxButton {
    fn execute(&self) {
        println!("LinuxButton - {} -> Clicked", self.0)
    }
}

impl Button for LinuxButton {}

pub struct LinuxCheckbox(pub bool);

impl Component for LinuxCheckbox {
    fn execute(&self) {
        println!("LinuxCheckbox - {} -> Toggle", self.0)
    }
}

impl Checkbox for LinuxCheckbox {}

use crate::behavioural::observer::ui_manager::Notifier;

use super::{Event, Listener, Unique};

pub struct ButtonClick(String);

impl ButtonClick {
    pub const fn new() -> Self {
        Self(String::new())
    }
}

impl Unique for ButtonClick {
    fn event(&self) -> super::Event {
        Event::ButtonClick
    }

    fn id(&self) -> String {
        self.0.clone()
    }
}

impl Listener for ButtonClick {
    fn receive(&self, _notifier: &dyn Notifier) {
        println!("ButtonClick {} has been notified", self.0);
    }
}

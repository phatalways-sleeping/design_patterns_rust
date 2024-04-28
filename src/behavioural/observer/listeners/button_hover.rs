use crate::behavioural::observer::ui_manager::Notifier;

use super::{Event, Listener, Unique};

pub struct ButtonHover(String);

impl ButtonHover {
    pub const fn new() -> Self {
        Self(String::new())
    }
}

impl Unique for ButtonHover {
    fn event(&self) -> super::Event {
        Event::ButtonHover
    }

    fn id(&self) -> String {
        self.0.clone()
    }
}

impl Listener for ButtonHover {
    fn receive(&self, _notifier: &dyn Notifier) {
        println!("ButtonHover {} has been notified", self.0);
    }
}

use super::ui_manager::Notifier;

pub mod button_click;
pub mod button_hover;

pub trait Unique {
    fn event(&self) -> Event;
    fn id(&self) -> String;
}

pub trait Listener: Unique {
    fn receive(&self, notifier: &dyn Notifier);
}

#[derive(PartialEq, Eq, Hash)]
pub enum Event {
    ButtonClick,
    ButtonHover,
}

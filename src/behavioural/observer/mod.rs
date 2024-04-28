pub mod listeners;
pub mod ui_manager;

#[cfg(test)]
mod test {
    use super::{
        listeners::{button_click::ButtonClick, button_hover::ButtonHover, Event},
        ui_manager::{ButtonEventsManager, Notifier},
    };

    #[test]
    fn ui_manager_works() {
        let mut manager = ButtonEventsManager::default();
        let clicky = ButtonClick::new();
        let hover = ButtonHover::new();
        manager.subsribe(Box::new(clicky));
        manager.subsribe(Box::new(hover));
        let clicky = ButtonClick::new();
        manager.notify(Event::ButtonClick);
        manager.unsubscribe(&clicky);
        manager.notify(Event::ButtonClick);
    }
}

use std::collections::HashMap;

use super::listeners::{Event, Listener, Unique};

pub trait Notifier {
    fn notify(&self, event: Event);
    fn subsribe(&mut self, subscriber: Box<dyn Listener>);
    fn unsubscribe(&mut self, subscriber: &dyn Unique);
}

#[derive(Default)]
pub struct ButtonEventsManager {
    subscribers: HashMap<Event, Vec<Box<dyn Listener>>>,
}

impl Notifier for ButtonEventsManager {
    fn notify(&self, event: Event) {
        if let Some(subscribers) = self.subscribers.get(&event) {
            subscribers.iter().for_each(|s| s.receive(self));
        }
    }

    fn subsribe(&mut self, subscriber: Box<dyn Listener>) {
        self.subscribers
            .entry(subscriber.event())
            .or_insert(vec![])
            .push(subscriber);
    }

    fn unsubscribe(&mut self, subscriber: &dyn Unique) {
        if let Some(subscribers) = self.subscribers.get_mut(&subscriber.event()) {
            subscribers.retain(|s| s.id() != subscriber.id());
        }
    }
}

use std::collections::{HashMap, VecDeque};

use crate::behavioural::mediator::train::Train;

use super::TrainStation;

#[derive(Default)]
pub struct NormalTrainStation {
    trains: HashMap<String, Box<dyn Train>>,
    train_queue: VecDeque<String>,
    train_on_platform: Option<String>,
}

impl TrainStation for NormalTrainStation {
    fn notify_arrival(&mut self, train_name: &str) -> bool {
        match &self.train_on_platform {
            Some(_) => {
                self.train_queue.push_back(train_name.into());
                false
            }
            None => {
                self.train_on_platform.replace(train_name.into());
                true
            }
        }
    }

    fn notify_departure(&mut self, train_name: &str) {
        if Some(train_name.into()) == self.train_on_platform {
            self.train_on_platform = None;
            if let Some(next_train_name) = self.train_queue.pop_front() {
                let mut next_train = self.trains.remove(&next_train_name).unwrap();
                next_train.arrive(self);
                self.trains.insert(next_train_name.clone(), next_train);
                self.train_on_platform = Some(next_train_name);
            }
        }
    }
}

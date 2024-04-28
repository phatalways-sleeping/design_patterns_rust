pub trait TrainStation {
    fn notify_arrival(&mut self, train_name: &str) -> bool;
    fn notify_departure(&mut self, train_name: &str);
}

pub mod normal_train_station;
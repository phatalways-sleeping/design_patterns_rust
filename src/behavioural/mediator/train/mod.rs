use super::train_station::TrainStation;

pub trait Train {
    fn name(&self) -> String;
    fn depart(&mut self, mediator: &mut dyn TrainStation);
    fn arrive(&mut self, mediator: &mut dyn TrainStation);
}

pub mod freight_train;
pub mod passenger_train;
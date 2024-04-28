use super::Train;

pub struct PassengerTrain {
    name: String,
}

impl PassengerTrain {
    pub fn new(name: &'static str) -> Self {
        Self { name: name.into() }
    }
}

impl Train for PassengerTrain {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn depart(
        &mut self,
        mediator: &mut dyn crate::behavioural::mediator::train_station::TrainStation,
    ) {
        mediator.notify_departure(&self.name);
        println!("PassengerTrain {}: Departed", self.name);
    }

    fn arrive(
        &mut self,
        mediator: &mut dyn crate::behavioural::mediator::train_station::TrainStation,
    ) {
        if !mediator.notify_arrival(&self.name) {
            println!("PassengerTrain {}: Arrival blocked, waiting", self.name);
            return;
        }
        println!("PassengerTrain {}: Arrived", self.name);
    }
}

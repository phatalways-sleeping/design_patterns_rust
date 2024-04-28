use super::Train;

pub struct FreightTrain {
    name: String,
}

impl FreightTrain {
    pub fn new(name: &'static str) -> Self {
        Self { name: name.into() }
    }
}

impl Train for FreightTrain {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn depart(
        &mut self,
        mediator: &mut dyn crate::behavioural::mediator::train_station::TrainStation,
    ) {
        mediator.notify_departure(&self.name);
        println!("FreightTrain {}: Departed", self.name);
    }

    fn arrive(
        &mut self,
        mediator: &mut dyn crate::behavioural::mediator::train_station::TrainStation,
    ) {
        if !mediator.notify_arrival(&self.name) {
            println!("FreightTrain {}: Arrival blocked, waiting", self.name);
            return;
        }
        println!("FreightTrain {}: Arrived", self.name);
    }
}

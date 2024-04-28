use super::transport::{Plane, Ship, Transport, Truck};

pub trait Logistics {
    fn plan_delivery(&self) {
        let transport = self.create_transport();
        transport.deliver();
    }
    fn create_transport(&self) -> impl Transport;
}

pub struct RoadLogistics;

impl Logistics for RoadLogistics {
    fn create_transport(&self) -> impl Transport {
        Truck
    }
}

pub struct SeaLogistics;

impl Logistics for SeaLogistics {
    fn create_transport(&self) -> impl Transport {
        Ship
    }
}

pub struct ExpressLogistics;

impl Logistics for ExpressLogistics {
    fn create_transport(&self) -> impl Transport {
        Plane
    }
}

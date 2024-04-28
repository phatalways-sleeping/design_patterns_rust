pub trait Transport {
    fn deliver(&self);
}

pub struct Truck;

impl Transport for Truck {
    fn deliver(&self) {
        println!("Truck is delivering packages...")
    }
}

pub struct Ship;

impl Transport for Ship {
    fn deliver(&self) {
        println!("Ship is delivering packages...")
    }
}

pub struct Plane;

impl Transport for Plane {
    fn deliver(&self) {
        println!("Plane is delivering packages...")
    }
}
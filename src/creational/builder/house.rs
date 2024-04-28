use super::components::{Door, Floor, NormalDoor, NormalFloor, NormalRoof, Roof, Window};

pub trait Builder {
    fn build(self) -> House;
}

pub struct House {
    pub door: Box<dyn Door>,
    pub roof: Box<dyn Roof>,
    pub windows: Option<Vec<Box<dyn Window>>>,
    pub floor: Box<dyn Floor>,
}

pub struct HouseBuilder {
    pub door: Option<Box<dyn Door>>,
    pub roof: Option<Box<dyn Roof>>,
    pub windows: Option<Vec<Box<dyn Window>>>,
    pub floor: Option<Box<dyn Floor>>,
}

impl Default for HouseBuilder {
    fn default() -> Self {
        Self {
            door: None,
            roof: None,
            windows: None,
            floor: None,
        }
    }
}

impl HouseBuilder {
    pub fn door(mut self, door: Box<dyn Door>) -> Self {
        self.door = Some(door);
        self
    }

    pub fn floor(mut self, floor: Box<dyn Floor>) -> Self {
        self.floor = Some(floor);
        self
    }

    pub fn windows(mut self, windows: Vec<Box<dyn Window>>) -> Self {
        self.windows = Some(windows);
        self
    }

    pub fn roof(mut self, roof: Box<dyn Roof>) -> Self {
        self.roof = Some(roof);
        self
    }
}

impl Builder for HouseBuilder {
    fn build(self) -> House {
        House {
            door: self.door.unwrap_or(Box::new(NormalDoor)),
            roof: self.roof.unwrap_or(Box::new(NormalRoof)),
            windows: self.windows,
            floor: self.floor.unwrap_or(Box::new(NormalFloor)),
        }
    }
}

pub trait Memento<T> {
    fn restore(&self) -> T;
}

pub struct Originator {
    state: usize,
}

pub struct OriginatorBackup {
    state: usize,
}

impl Originator {
    pub fn new(state: usize) -> Self {
        Self { state }
    }

    pub fn save(&self) -> OriginatorBackup {
        OriginatorBackup { state: self.state }
    }
}

impl Memento<Originator> for OriginatorBackup {
    fn restore(&self) -> Originator {
        Originator::new(self.state)
    }
}

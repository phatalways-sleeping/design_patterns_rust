use std::cell::RefCell;

use super::{
    patient::Patient,
    staff::{cashier::Cashier, doctor::Doctor, receptionist::Receptionist, Staff},
};

pub struct HospitalSystem(RefCell<Option<Box<dyn Staff>>>);

impl Default for HospitalSystem {
    fn default() -> Self {
        let this = Self::new();
        this.push(Box::new(Cashier::new("Peter".into())));
        this.push(Box::new(Doctor::new("Peter".into(), "absc".into())));
        this.push(Box::new(Receptionist::new("Peter".into(), "sjbc".into())));
        this
    }
}

impl HospitalSystem {
    pub const fn new() -> Self {
        Self(RefCell::new(None))
    }

    pub fn push(&self, staff: Box<dyn Staff>) {
        let old_chain = self.0.borrow_mut().take();
        let new_chain = match old_chain {
            Some(head) => {
                staff.next(head);
                Some(staff)
            }
            None => Some(staff),
        };
        *self.0.borrow_mut() = new_chain;
    }

    pub fn serve(&self, patient: Patient) {
        if let Some(line) = self.0.borrow().as_ref() {
            line.handle(patient)
        }
    }
}

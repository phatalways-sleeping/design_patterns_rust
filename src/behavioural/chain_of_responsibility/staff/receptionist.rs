use std::cell::RefCell;

use crate::behavioural::chain_of_responsibility::patient::Patient;

use super::Staff;

pub struct Receptionist {
    name: String,
    id: String,
    next: RefCell<Option<Box<dyn Staff>>>,
}

impl Receptionist {
    pub const fn new(name: String, id: String) -> Self {
        Self {
            name,
            id,
            next: RefCell::new(None),
        }
    }
}

impl Staff for Receptionist {
    fn handle(&self, patient: Patient) {
        println!("Receptionist {} - {}", self.name, self.id);
        println!("\tWhat is your name sir? - Patient: {}", &patient.name);
        println!(
            "\tWhat is your symptom sir? - Patient: {}",
            &patient.symptomp
        );
        if let Some(staff) = self.next.borrow().as_ref() {
            staff.handle(patient)
        } else {
            println!("Receptionist is the end of line");
        }
    }

    fn next(&self, staff: Box<dyn Staff>) {
        *self.next.borrow_mut() = Some(staff);
    }
}

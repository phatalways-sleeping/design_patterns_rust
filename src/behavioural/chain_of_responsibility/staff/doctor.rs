use std::cell::RefCell;

use crate::behavioural::chain_of_responsibility::patient::Patient;

use super::Staff;

pub struct Doctor {
    name: String,
    id: String,
    next: RefCell<Option<Box<dyn Staff>>>,
}

impl Doctor {
    pub const fn new(name: String, id: String) -> Self {
        Self {
            name,
            id,
            next: RefCell::new(None),
        }
    }
}

impl Staff for Doctor {
    fn handle(&self, patient: Patient) {
        println!("Doctor {} - {}", self.name, self.id);
        println!("\tWhat is your name sir? - Patient: {}", &patient.name);
        println!(
            "\tWhat is your symptom sir? - Patient: {}",
            &patient.symptomp
        );
        println!("\tLook like you need a rest!");
        if let Some(staff) = self.next.borrow().as_ref() {
            staff.handle(patient)
        } else {
            println!("Doctor is the end of line");
        }
    }

    fn next(&self, staff: Box<dyn Staff>) {
        *self.next.borrow_mut() = Some(staff);
    }
}

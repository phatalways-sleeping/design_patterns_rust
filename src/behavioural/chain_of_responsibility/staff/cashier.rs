use std::cell::RefCell;

use crate::behavioural::chain_of_responsibility::patient::Patient;

use super::Staff;

pub struct Cashier {
    name: String,
    next: RefCell<Option<Box<dyn Staff>>>,
}

impl Cashier {
    pub const fn new(name: String) -> Self {
        Self {
            name,
            next: RefCell::new(None),
        }
    }
}

impl Staff for Cashier {
    fn handle(&self, patient: Patient) {
        println!("Cashier {}", self.name);
        println!("\tWhat is your name sir? - Patient: {}", &patient.name);
        println!(
            "\tWhat is your credit card number sir? - Patient: {}",
            &patient.credit_card
        );
        println!("\tPayment completed");
        if let Some(staff) = self.next.borrow().as_ref() {
            staff.handle(patient)
        } else {
            println!("Cashier is the end of line");
        }
    }

    fn next(&self, staff: Box<dyn Staff>) {
        *self.next.borrow_mut() = Some(staff);
    }
}

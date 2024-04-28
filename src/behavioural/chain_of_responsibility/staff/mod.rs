use super::patient::Patient;

pub mod receptionist;
pub mod doctor;
pub mod cashier;

pub trait Staff {
    fn handle(&self, patient: Patient);
    fn next(&self, staff: Box<dyn Staff>);
}

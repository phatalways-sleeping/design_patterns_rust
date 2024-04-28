#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Data {}

pub enum File {
    PDF(String),
    Text(String),
    XML(String),
}

pub trait Extractable {
    fn sort(&self, data: &mut Vec<Data>) {
        data.sort();
    }
    fn extract(&self, _data: File) -> Vec<Data> {
        // Perform extraction
        let mut results = vec![];
        self.sort(&mut results);
        results
    }
}

pub struct BinarySortExtraction;

impl Extractable for BinarySortExtraction {
    fn sort(&self, data: &mut Vec<Data>) {
        data.sort();
    }
}

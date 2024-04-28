use super::types::{TwoValuesArray, TwoValuesStruct};

pub trait Converter {
    type Value;
    fn convert(&self, value: Vec<i32>) -> Result<Self::Value, &'static str>;
}

impl Converter for TwoValuesStruct {
    type Value = TwoValuesStruct;

    fn convert(&self, value: Vec<i32>) -> Result<Self::Value, &'static str> {
        if value.len() < 2 {
            Err("Error: Not enough params")
        } else {
            Ok(TwoValuesStruct {
                a: value[0],
                b: value[1],
            })
        }
    }
}

impl Converter for TwoValuesArray {
    type Value = TwoValuesArray;

    fn convert(&self, value: Vec<i32>) -> Result<Self::Value, &'static str> {
        if value.len() < 2 {
            Err("Error: Not enough params")
        } else {
            Ok(TwoValuesArray {
                arr: [value[0], value[1]],
            })
        }
    }
}

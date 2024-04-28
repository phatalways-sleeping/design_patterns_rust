use super::converter::Converter;

pub trait Deserializer<C: Converter> {
    fn new(converter: C) -> Self;
    fn parse_str(&self, input: &str) -> Result<C::Value, &'static str>;
    fn parse_vec(&self, input: &[i32]) -> Result<C::Value, &'static str>;
}

pub struct StringDeserializer<C: Converter> {
    converter: C,
}

impl<C: Converter> Deserializer<C> for StringDeserializer<C> {
    fn new(converter: C) -> Self {
        Self { converter }
    }

    fn parse_str(&self, input: &str) -> Result<<C as Converter>::Value, &'static str> {
        self.converter.convert(
            input
                .split_ascii_whitespace()
                .into_iter()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }

    fn parse_vec(&self, _input: &[i32]) -> Result<<C as Converter>::Value, &'static str> {
        panic!("Error: Unsupported operation")
    }
}

pub struct VecDeserializer<C: Converter> {
    converter: C,
}

impl<C: Converter> Deserializer<C> for VecDeserializer<C> {
    fn new(converter: C) -> Self {
        Self { converter }
    }

    fn parse_str(&self, _input: &str) -> Result<<C as Converter>::Value, &'static str> {
        panic!("Error: Unsupported operation")
    }

    fn parse_vec(&self, input: &[i32]) -> Result<<C as Converter>::Value, &'static str> {
        self.converter.convert(input.to_owned())
    }
}

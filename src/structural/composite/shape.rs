use super::graphic::Drawable;

#[derive(Debug)]
pub struct Circle {
    pub center: (i32, i32),
    pub radius: u32,
}

impl Circle {
    pub const fn new(center: (i32, i32), radius: u32) -> Self {
        Self { center, radius }
    }
}

impl Drawable for Circle {
    fn execute(&self) {
        println!("Drawing {:?}", self);
    }
}

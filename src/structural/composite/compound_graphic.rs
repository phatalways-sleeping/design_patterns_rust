use super::graphic::Drawable;

pub struct CompoundGraphic {
    shapes: Vec<Box<dyn Drawable>>
}

impl CompoundGraphic {
    pub fn add(&mut self, shape: Box<dyn Drawable>) {
        self.shapes.push(shape);
    }
}

impl Drawable for CompoundGraphic {
    fn execute(&self) {
        for shape in &self.shapes {
            shape.execute();
        }
    }
}
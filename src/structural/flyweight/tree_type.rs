#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeType {
    name: String,
    color: String,
    texture: String,
}

impl Default for TreeType {
    fn default() -> Self {
        Self {
            name: "Apple".into(),
            color: "green".into(),
            texture: "image_2.png".into(),
        }
    }
}

impl TreeType {
    pub const fn new(name: String, color: String, texture: String) -> Self {
        Self {
            name,
            color,
            texture,
        }
    }

    pub fn render(&self, x: i32, y: i32) {
        println!("Tree: {:?} at ({}, {})", self, x, y)
    }
}

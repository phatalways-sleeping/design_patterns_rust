use super::{tree_factory::TreeFactory, tree_type::TreeType};

pub struct Tree<'a> {
    x: i32,
    y: i32,
    state: &'a TreeType,
}

impl<'a> Tree<'a> {
    pub const fn new(x: i32, y: i32, state: &'a TreeType) -> Self {
        Self { x, y, state }
    }

    pub fn render(&self) {
        self.state.render(self.x, self.y)
    }
}

pub struct Forest<'a> {
    trees: Vec<Tree<'a>>,
    factory: TreeFactory,
}

impl<'a> Forest<'a> {
    pub const fn new() -> Self {
        Self {
            trees: vec![],
            factory: TreeFactory::new(),
        }
    }

    pub fn insert(
        &mut self,
        name: String,
        color: String,
        texture: String,
        at: (i32, i32),
    ) -> &'a Tree {
        self.trees.push(Tree::new(
            at.0,
            at.1,
            &self.factory.get(name, color, texture),
        ));
        self.trees.last().unwrap()
    }
}

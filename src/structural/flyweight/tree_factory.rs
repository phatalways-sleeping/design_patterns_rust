use std::collections::BTreeMap;

use super::tree_type::TreeType;

pub struct TreeFactory {
    trees: BTreeMap<(String, String, String), TreeType>,
}

impl TreeFactory {
    pub const fn new() -> Self {
        Self {
            trees: BTreeMap::new(),
        }
    }

    pub fn get(&mut self, name: String, color: String, texture: String) -> &TreeType {
        self.trees
            .entry((name.clone(), color.clone(), texture.clone()))
            .or_insert(TreeType::new(name, color, texture))
    }
}

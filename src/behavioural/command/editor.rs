pub trait Selectable {
    fn get_selection(&self) -> String;
}

pub trait Modifiable {
    fn select(&mut self, from: usize, to: usize);
    fn delete_selection(&mut self);
    fn replace_selection_with(&mut self, text: String);
}

pub struct Editor {
    text: String,
    selection: Option<String>,
}

impl Editor {
    pub const fn new() -> Self {
        Self {
            text: String::new(),
            selection: None,
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            text: String::new(),
            selection: None,
        }
    }
}

impl Selectable for Editor {
    fn get_selection(&self) -> String {
        self.selection.clone().unwrap_or_default()
    }
}

impl Modifiable for Editor {
    fn delete_selection(&mut self) {
        if let Some(selection) = self.selection.take() {
            self.text = self.text.replace(&selection, "");
        }
    }

    fn replace_selection_with(&mut self, text: String) {
        if let Some(selection) = self.selection.take() {
            self.text = self.text.replace(&selection, &text);
        }
    }

    fn select(&mut self, from: usize, to: usize) {
        let chars = self.text.chars().collect::<Vec<_>>();
        let selection = chars[from..to].into_iter().collect::<String>();
        self.selection = Some(selection);
    }
}

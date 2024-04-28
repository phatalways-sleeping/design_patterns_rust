pub struct Account {
    name: String,
}

impl From<&str> for Account {
    fn from(value: &str) -> Self {
        Account { name: value.into() }
    }
}

impl From<String> for Account {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

impl Account {
    pub fn check(&self, name: &str) -> Result<(), &'static str> {
        if name != self.name {
            Err("Invalid account name")
        } else {
            Ok(())
        }
    }
}

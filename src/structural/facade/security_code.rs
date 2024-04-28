pub struct SecurityCode {
    code: String,
}

impl From<&str> for SecurityCode {
    fn from(value: &str) -> Self {
        SecurityCode { code: value.into() }
    }
}

impl From<String> for SecurityCode {
    fn from(value: String) -> Self {
        Self { code: value }
    }
}

impl SecurityCode {
    pub fn check(&self, code: &str) -> Result<(), &'static str> {
        if code != self.code {
            Err("Invalid security code")
        } else {
            Ok(())
        }
    }
}

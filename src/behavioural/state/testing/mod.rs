pub struct Test {
    pub trigger: Box<dyn Fn(&str) -> TestResult>,
}

impl Test {
    pub(crate) fn execute(&self, source: &str) -> TestResult {
        (*self.trigger.as_ref())(source)
    }
}

impl From<Box<dyn Fn(&str) -> TestResult>> for Test {
    fn from(trigger: Box<dyn Fn(&str) -> TestResult>) -> Self {
        Self { trigger }
    }
}

pub struct TestResult {
    successful: bool,
    stack: Option<String>,
}

impl TestResult {
    pub const fn succeed() -> Self {
        Self {
            successful: true,
            stack: None,
        }
    }

    pub fn fail(stack: &str) -> Self {
        Self {
            successful: false,
            stack: Some(stack.into()),
        }
    }

    pub fn is_successful(&self) -> bool {
        self.successful
    }

    pub fn stack_trace(&self) -> &Option<String> {
        &self.stack
    }
}

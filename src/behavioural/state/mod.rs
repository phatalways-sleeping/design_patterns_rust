use self::{codebase::Codebase, environment::Environment};

pub mod codebase;
pub mod environment;
pub mod testing;

pub struct TestManager {
    codebase: Box<dyn Codebase>,
    environment: Box<dyn Environment>,
}

impl TestManager {
    pub const fn start_with(
        codebase: Box<dyn Codebase>,
        environment: Box<dyn Environment>,
    ) -> Self {
        Self {
            codebase,
            environment,
        }
    }

    pub fn execute(&mut self) {
        loop {
            match self.environment.push(self.codebase.as_ref()) {
                Ok(success) => match success {
                    environment::SuccessKind::Normal(env) => self.environment = env,
                    environment::SuccessKind::End => {
                        println!("All tests are ran!");
                        break;
                    }
                },
                Err(msg) => {
                    println!("{}", msg);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        codebase::e_commerce::ECommerceSrcCode, environment::developing::Developing, TestManager,
    };

    #[test]
    fn source_code_works() {
        let my_src_code = ECommerceSrcCode {
            source: "mod routes; fn handle_get() -> bool { true }".into(),
        };
        let dev_environment = Developing::new();
        let mut manager = TestManager::start_with(Box::new(my_src_code), dev_environment);
        manager.execute();
    }
}

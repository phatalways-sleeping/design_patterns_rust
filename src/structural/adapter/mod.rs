use self::target::Target;

pub mod specific_target;
pub mod target;
pub mod target_adapter;

pub fn call(target: impl Target) -> String {
    target.request()
}

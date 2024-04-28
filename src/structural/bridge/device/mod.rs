pub mod tv;

pub trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn get_volume(&self) -> i32;
    fn set_volume(&mut self, percent: i32);
    fn get_channel(&self) -> i32;
    fn set_channel(&mut self, channel: i32);
}

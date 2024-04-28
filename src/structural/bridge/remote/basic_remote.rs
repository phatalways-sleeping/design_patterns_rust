use crate::structural::bridge::device::tv::TV;

use super::{HasMutDevice, Remote};

pub struct BasicRemote {
    device: TV,
}

impl HasMutDevice<TV> for BasicRemote {
    fn device(&mut self) -> &mut TV {
        &mut self.device
    }
}

impl Remote<TV> for BasicRemote {}

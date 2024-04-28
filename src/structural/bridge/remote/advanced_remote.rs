use crate::structural::bridge::device::{tv::TV, Device};

use super::{HasMutDevice, Remote};

pub struct AdvancedRemote {
    device: TV,
}

impl AdvancedRemote {
    pub fn new() -> Self {
        Self {
            device: TV::default(),
        }
    }

    pub fn mute(&mut self) {
        while self.device().get_volume() > 0 {
            self.volume_down()
        }
    }
}

impl HasMutDevice<TV> for AdvancedRemote {
    fn device(&mut self) -> &mut TV {
        &mut self.device
    }
}

impl Remote<TV> for AdvancedRemote {}

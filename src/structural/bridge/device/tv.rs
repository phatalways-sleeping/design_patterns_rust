use super::Device;

pub struct TV {
    channel: i32,
    volume: i32,
    on: bool,
}

impl Default for TV {
    fn default() -> Self {
        Self {
            channel: 1,
            volume: 100,
            on: true,
        }
    }
}

impl Device for TV {
    fn is_enabled(&self) -> bool {
        self.on
    }

    fn enable(&mut self) {
        self.on = true;
    }

    fn disable(&mut self) {
        self.on = false;
    }

    fn get_volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&mut self, percent: i32) {
        self.volume = percent;
    }

    fn get_channel(&self) -> i32 {
        self.channel
    }

    fn set_channel(&mut self, channel: i32) {
        self.channel = channel;
    }
}

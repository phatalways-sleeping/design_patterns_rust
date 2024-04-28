use super::device::Device;

pub mod basic_remote;
pub mod advanced_remote;


pub trait HasMutDevice<D: Device> {
    fn device(&mut self) -> &mut D;
}

pub trait Remote<D: Device>: HasMutDevice<D> {
    fn toggle_power(&mut self) {
        if self.device().is_enabled() {
            self.device().disable();
        } else {
            self.device().enable();
        }
    }

    fn volume_up(&mut self) {
        let volume = self.device().get_volume();
        if volume < 100 {
            self.device().set_volume(volume + 10);
        }
    }

    fn volume_down(&mut self) {
        let volume = self.device().get_volume();
        if volume > 0 {
            self.device().set_volume(volume - 10);
        }
    }

    fn channel_up(&mut self) {
        let channel = self.device().get_channel();
        if channel < 100 {
            self.device().set_channel(channel + 1);
        }
    }

    fn channel_down(&mut self) {
        let channel = self.device().get_channel();
        if channel > 0 {
            self.device().set_channel(channel - 1);
        }
    }
}

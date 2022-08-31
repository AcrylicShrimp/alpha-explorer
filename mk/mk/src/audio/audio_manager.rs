use super::AudioChannel;

pub struct AudioManager {
    default_channel: AudioChannel,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            default_channel: AudioChannel::new(),
        }
    }

    pub fn default_channel(&self) -> &AudioChannel {
        &self.default_channel
    }
}

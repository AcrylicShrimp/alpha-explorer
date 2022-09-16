use crate::{audio::AudioClip, engine::use_context};
use rodio::Sink;
use specs::{prelude::*, Component};
use std::sync::Arc;

#[derive(Component)]
pub struct AudioSource {
    volume: f32,
    clip: Option<Arc<AudioClip>>,
    sink: Option<Sink>,
}

impl AudioSource {
    pub fn new() -> Self {
        Self {
            volume: 1f32,
            clip: None,
            sink: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.is_some()
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, mut volume: f32) {
        volume = volume.clamp(0f32, 1f32);
        self.volume = volume;

        if let Some(sink) = &mut self.sink {
            sink.set_volume(volume);
        }
    }

    pub fn clip(&self) -> Option<Arc<AudioClip>> {
        self.clip.clone()
    }

    pub fn set_clip(&mut self, clip: Option<Arc<AudioClip>>) {
        self.clip = clip;

        if let Some(sink) = self.sink.take() {
            sink.stop();
            self.play();
        }
    }

    // pub fn play_one_shot(&mut self, clip: LuaAudioClipHandle, volume: Option<f32>) {
    //     let volume = volume.unwrap_or(1f32) * self.volume;
    //     let (_, handle) = OutputStream::try_default().unwrap();
    //     let sink = Sink::try_new(&handle).unwrap();
    //     sink.set_volume(volume);
    //     sink.append(clip.raw());
    // }

    pub fn play(&mut self) {
        if self.is_playing() {
            return;
        }

        let raw_clip = if let Some(clip) = &self.clip {
            clip.raw()
        } else {
            return;
        };

        let channel = use_context().audio_mgr().default_channel();
        let sink = Sink::try_new(&channel.handle()).unwrap();
        sink.set_volume(self.volume);
        sink.append(raw_clip);
        sink.play();

        self.sink = Some(sink);
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
    }

    pub fn update(&mut self) {
        if let Some(sink) = &self.sink {
            if sink.empty() {
                self.sink = None;
            }
        }
    }
}

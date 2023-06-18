use rodio::{cpal::FromSample, source::Buffered, Sample, Source};

pub type RawAudioClip = Buffered<Box<dyn Source<Item = f32> + Send>>;

pub struct AudioClip {
    raw: RawAudioClip,
}

impl AudioClip {
    pub fn new<T, I>(raw: T) -> Self
    where
        T: Source<Item = I> + Send + 'static,
        I: Sample,
        f32: FromSample<I>,
    {
        Self {
            raw: (Box::new(raw.convert_samples()) as Box<dyn Source<Item = f32> + Send>).buffered(),
        }
    }

    pub fn raw(&self) -> RawAudioClip {
        self.raw.clone()
    }
}

unsafe impl Send for AudioClip {}

unsafe impl Sync for AudioClip {}

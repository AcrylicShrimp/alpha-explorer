use super::EntityBuilderParam;
use crate::audio::AudioClip;
use rhai::EvalAltResult;
use std::sync::Arc;

#[derive(Default)]
pub struct AudioSourceParams {
    pub volume: Option<f32>,
    pub clip: Option<Arc<AudioClip>>,
}

impl EntityBuilderParam for AudioSourceParams {
    fn from_table(mut table: rhai::Map) -> Result<Self, Box<EvalAltResult>> {
        Ok(Self {
            volume: table
                .remove("volume")
                .map(|volume| {
                    volume
                        .try_cast()
                        .ok_or_else(|| "the field 'volume' is not valid type")
                })
                .transpose()?,
            clip: table
                .remove("clip")
                .map(|clip| {
                    clip.try_cast()
                        .ok_or_else(|| "the field 'clip' is not valid type")
                })
                .transpose()?,
        })
    }
}

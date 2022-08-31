use crate::component::*;
use legion::*;

pub fn update_audio_sources(world: &mut World) {
    let mut query = <&mut AudioSource>::query();

    for source in query.iter_mut(world) {
        source.update();
    }
}

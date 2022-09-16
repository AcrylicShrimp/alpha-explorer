use crate::component::*;
use specs::prelude::*;

pub struct AudioSystem;

impl<'a> System<'a> for AudioSystem {
    type SystemData = (WriteStorage<'a, AudioSource>,);

    fn run(&mut self, (mut source,): Self::SystemData) {
        for source in (&mut source).join() {
            source.update();
        }
    }
}

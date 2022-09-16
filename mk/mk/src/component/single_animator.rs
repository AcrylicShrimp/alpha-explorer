use crate::animation::Animation;
use specs::{prelude::*, Component};

#[derive(Component)]
pub struct SingleAnimator {
    pub is_pong: bool,
    pub time: f32,
    pub speed: f32,
    pub animation: Option<Animation>,
}

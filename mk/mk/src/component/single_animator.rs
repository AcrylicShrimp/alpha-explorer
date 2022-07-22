use crate::animation::Animation;

#[derive(Debug, Clone)]
pub struct SingleAnimator {
    pub is_pong: bool,
    pub time: f32,
    pub speed: f32,
    pub animation: Option<Animation>,
}

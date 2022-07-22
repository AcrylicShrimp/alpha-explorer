pub trait Animate {
    fn ty(&self) -> &'static str;
    fn animate(
        &mut self,
        _time_line: &crate::animation::AnimationTimeLine,
        _key_frame: &crate::animation::AnimationKeyFrame,
        _normalized_time_in_key_frame: f32,
    ) {
    }
}

use super::Event;

pub trait NativeEvent
where
    Self: Event,
{
    fn name() -> &'static str;
}

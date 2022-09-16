#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIScaleMode {
    Constant,
    Stretch,
    Fit,
    Fill,
    MatchWidth,
    MatchHeight,
}

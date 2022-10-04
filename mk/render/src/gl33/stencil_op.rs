#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    Increase,
    IncreaseWrap,
    Decrease,
    DecreaseWrap,
    Invert,
}

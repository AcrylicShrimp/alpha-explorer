pub trait Interpolatable {
    fn interpolate(lhs: Self, rhs: Self, t: f32) -> Self;
}

macro_rules! impl_interpolatable_numeric {
    ($ty:ty) => {
        impl Interpolatable for $ty {
            fn interpolate(lhs: Self, rhs: Self, t: f32) -> Self {
                if t <= 0f32 {
                    return lhs;
                }

                if t >= 1f32 {
                    return rhs;
                }

                let lhs = lhs as f32;
                let rhs = rhs as f32;
                (lhs + (rhs - lhs) * t) as $ty
            }
        }
    };
}

macro_rules! impl_interpolatable_nonzero_numeric {
    ($ty:ty) => {
        impl Interpolatable for $ty {
            fn interpolate(lhs: Self, rhs: Self, t: f32) -> Self {
                if t <= 0f32 {
                    return lhs;
                }

                if t >= 1f32 {
                    return rhs;
                }

                let lhs = lhs.get() as f32;
                let rhs = rhs.get() as f32;
                <$ty>::new((lhs + (rhs - lhs) * t) as _).unwrap()
            }
        }
    };
}

impl_interpolatable_numeric!(i8);
impl_interpolatable_numeric!(i16);
impl_interpolatable_numeric!(i32);
impl_interpolatable_numeric!(i64);
impl_interpolatable_numeric!(i128);

impl_interpolatable_numeric!(u8);
impl_interpolatable_numeric!(u16);
impl_interpolatable_numeric!(u32);
impl_interpolatable_numeric!(u64);
impl_interpolatable_numeric!(u128);

impl_interpolatable_numeric!(f32);
impl_interpolatable_numeric!(f64);

impl_interpolatable_numeric!(isize);
impl_interpolatable_numeric!(usize);

impl_interpolatable_nonzero_numeric!(std::num::NonZeroI8);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroI16);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroI32);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroI64);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroI128);

impl_interpolatable_nonzero_numeric!(std::num::NonZeroU8);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroU16);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroU32);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroU64);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroU128);

impl_interpolatable_nonzero_numeric!(std::num::NonZeroIsize);
impl_interpolatable_nonzero_numeric!(std::num::NonZeroUsize);

impl Interpolatable for bool {
    fn interpolate(_lhs: Self, rhs: Self, _t: f32) -> Self {
        rhs
    }
}

impl Interpolatable for &str {
    fn interpolate(_lhs: Self, rhs: Self, _t: f32) -> Self {
        rhs
    }
}

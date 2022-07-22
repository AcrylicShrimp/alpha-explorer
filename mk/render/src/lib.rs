#[macro_export]
macro_rules! ptr_init {
    ($ptr0:ident => $body:expr) => {
        unsafe {
            let mut value0 = std::mem::MaybeUninit::uninit();
            let $ptr0 = value0.as_mut_ptr();
            $body;
            value0.assume_init()
        }
    };
    ($ptr0:ident, $ptr1:ident => $body:expr) => {
        unsafe {
            let mut value0 = std::mem::MaybeUninit::uninit();
            let mut value1 = std::mem::MaybeUninit::uninit();
            let $ptr0 = value0.as_mut_ptr();
            let $ptr1 = value1.as_mut_ptr();
            $body;
            (value0.assume_init(), value1.assume_init())
        }
    };
    ($ptr0:ident, $ptr1:ident, $ptr2:ident => $body:expr) => {
        unsafe {
            let mut value0 = std::mem::MaybeUninit::uninit();
            let mut value1 = std::mem::MaybeUninit::uninit();
            let mut value2 = std::mem::MaybeUninit::uninit();
            let $ptr0 = value0.as_mut_ptr();
            let $ptr1 = value1.as_mut_ptr();
            let $ptr2 = value2.as_mut_ptr();
            $body;
            (
                value0.assume_init(),
                value1.assume_init(),
                value2.assume_init(),
            )
        }
    };
    ($ptr0:ident, $ptr1:ident, $ptr2:ident, $ptr3:ident => $body:expr) => {
        unsafe {
            let mut value0 = std::mem::MaybeUninit::uninit();
            let mut value1 = std::mem::MaybeUninit::uninit();
            let mut value2 = std::mem::MaybeUninit::uninit();
            let mut value3 = std::mem::MaybeUninit::uninit();
            let $ptr0 = value0.as_mut_ptr();
            let $ptr1 = value1.as_mut_ptr();
            let $ptr2 = value2.as_mut_ptr();
            let $ptr3 = value3.as_mut_ptr();
            $body;
            (
                value0.assume_init(),
                value1.assume_init(),
                value2.assume_init(),
                value3.assume_init(),
            )
        }
    };
}

#[cfg(gl33)]
mod gl33;
mod native_handle;
mod object;
mod render_mode;
mod shader_type;
mod texture_format;

#[cfg(gl33)]
pub use crate::gl33::*;
pub use native_handle::*;
pub use object::*;
pub use render_mode::*;
pub use shader_type::*;
pub use texture_format::*;

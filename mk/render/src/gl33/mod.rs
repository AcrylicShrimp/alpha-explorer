#[macro_export]
macro_rules! check_err {
    () => {
        #[cfg(debug_assertions)]
        {
            #[allow(unused_unsafe)]
            let err = unsafe { gl33::GetError() };

            if err != 0 {
                panic!("GL ERR: {}", err);
            }
        }
    };
}

mod buffer;
mod render_request;
mod renderer;
mod shader;
mod stencil_fn;
mod stencil_op;
mod texture;

pub use buffer::*;
pub use render_request::*;
pub use renderer::*;
pub use shader::*;
pub use stencil_fn::*;
pub use stencil_op::*;
pub use texture::*;

pub fn init(f: impl FnMut(&str) -> *const std::ffi::c_void) {
    gl33::load_with(f);

    // NOTE: We're getting a GL error here - we don't know why.
    // Clear the GL error by calling the `glGetError` once here.
    unsafe {
        gl33::GetError();
    }

    // Set basic settings.
    unsafe {
        gl33::ClearColor(0f32, 0f32, 0f32, 0f32);
        gl33::Enable(gl33::BLEND);
        gl33::BlendFunc(gl33::SRC_ALPHA, gl33::ONE_MINUS_SRC_ALPHA);
    }
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl33::ClearColor(r, g, b, a);
    }
}

pub fn clear() {
    unsafe {
        gl33::Clear(gl33::COLOR_BUFFER_BIT);
    }
}

pub fn erase_stencil(s: i32) {
    unsafe {
        gl33::ClearStencil(s);
        gl33::Clear(gl33::STENCIL_BUFFER_BIT);
    }
}

pub fn configure_stencil_fn(stencil_fn: StencilFn, stencil_ref: i32, mask: u32) {
    unsafe {
        gl33::StencilFunc(
            match stencil_fn {
                StencilFn::Never => gl33::NEVER,
                StencilFn::Less => gl33::LESS,
                StencilFn::LessEq => gl33::LEQUAL,
                StencilFn::Greater => gl33::GREATER,
                StencilFn::GreaterEq => gl33::GEQUAL,
                StencilFn::Eq => gl33::EQUAL,
                StencilFn::NotEq => gl33::NOTEQUAL,
                StencilFn::Always => gl33::ALWAYS,
            },
            stencil_ref,
            mask,
        )
    }
}

pub fn configure_stencil_op(stencil_op_pass: StencilOp, stencil_op_fail: StencilOp) {
    let op_to_gl33_op = |op: StencilOp| match op {
        StencilOp::Keep => gl33::KEEP,
        StencilOp::Zero => gl33::ZERO,
        StencilOp::Replace => gl33::REPLACE,
        StencilOp::Increase => gl33::INCR,
        StencilOp::IncreaseWrap => gl33::INCR_WRAP,
        StencilOp::Decrease => gl33::DECR,
        StencilOp::DecreaseWrap => gl33::DECR_WRAP,
        StencilOp::Invert => gl33::INVERT,
    };

    unsafe {
        gl33::StencilOp(
            op_to_gl33_op(stencil_op_fail),
            op_to_gl33_op(stencil_op_pass),
            op_to_gl33_op(stencil_op_pass),
        )
    }
}

pub fn resize(width: u32, height: u32) {
    unsafe {
        gl33::Viewport(0, 0, width as _, height as _);
    }
}

pub fn enable_color_buffer() {
    unsafe {
        gl33::ColorMask(1, 1, 1, 1);
    }
}

pub fn disable_color_buffer() {
    unsafe {
        gl33::ColorMask(0, 0, 0, 0);
    }
}

pub fn enable_stencil() {
    unsafe {
        gl33::Enable(gl33::STENCIL_TEST);
    }
}

pub fn disable_stencil() {
    unsafe {
        gl33::Disable(gl33::STENCIL_TEST);
    }
}

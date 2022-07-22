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
mod texture;

pub use buffer::*;
pub use render_request::*;
pub use renderer::*;
pub use shader::*;
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

pub fn clear() {
    unsafe {
        gl33::Clear(gl33::COLOR_BUFFER_BIT);
    }
}

pub fn resize(width: u32, height: u32) {
    unsafe {
        gl33::Viewport(0, 0, width as _, height as _);
    }
}

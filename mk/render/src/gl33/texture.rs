use crate::{NativeHandle, Object, TextureFormat};
use gl33::types::*;
use std::ptr::null;

#[derive(Debug)]
pub struct Texture {
    handle: GLuint,
    width: u32,
    height: u32,
    format: TextureFormat,
}

impl Texture {
    pub fn with_size_r_u8(width: u32, height: u32) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::R8 as _,
                width as _,
                height as _,
                0,
                gl33::RED,
                gl33::UNSIGNED_BYTE,
                null(),
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RU8,
        }
    }

    pub fn with_size_r_u8_smooth(width: u32, height: u32) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::R8 as _,
                width as _,
                height as _,
                0,
                gl33::RED,
                gl33::UNSIGNED_BYTE,
                null(),
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::LINEAR as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::LINEAR as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RU8,
        }
    }

    pub fn with_size_rg_u8(width: u32, height: u32) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::RG8 as _,
                width as _,
                height as _,
                0,
                gl33::RG,
                gl33::UNSIGNED_BYTE,
                null(),
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RGU8,
        }
    }

    pub fn with_size_rgb_u8(width: u32, height: u32) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::RGB8 as _,
                width as _,
                height as _,
                0,
                gl33::RGB,
                gl33::UNSIGNED_BYTE,
                null(),
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RGBU8,
        }
    }

    pub fn with_size_rgba_u8(width: u32, height: u32) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::RGBA8 as _,
                width as _,
                height as _,
                0,
                gl33::RGBA,
                gl33::UNSIGNED_BYTE,
                null(),
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RGBAU8,
        }
    }

    pub fn from_slice_r_u8(width: u32, height: u32, data: &[u8]) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::R8 as _,
                width as _,
                height as _,
                0,
                gl33::RED,
                gl33::UNSIGNED_BYTE,
                data.as_ptr() as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RU8,
        }
    }

    pub fn from_slice_rg_u8(width: u32, height: u32, data: &[u8]) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::RG8 as _,
                width as _,
                height as _,
                0,
                gl33::RG,
                gl33::UNSIGNED_BYTE,
                data.as_ptr() as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RGU8,
        }
    }

    pub fn from_slice_rgb_u8(width: u32, height: u32, data: &[u8]) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::RGB8 as _,
                width as _,
                height as _,
                0,
                gl33::RGB,
                gl33::UNSIGNED_BYTE,
                data.as_ptr() as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RGBU8,
        }
    }

    pub fn from_slice_rgba_u8(width: u32, height: u32, data: &[u8]) -> Texture {
        let handle = ptr_init!(ptr => gl33::GenTextures(1, ptr));

        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, handle);
            check_err!();
            gl33::TexImage2D(
                gl33::TEXTURE_2D,
                0,
                gl33::RGBA8 as _,
                width as _,
                height as _,
                0,
                gl33::RGBA,
                gl33::UNSIGNED_BYTE,
                data.as_ptr() as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MIN_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_MAG_FILTER,
                gl33::NEAREST as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_S,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
            gl33::TexParameteri(
                gl33::TEXTURE_2D,
                gl33::TEXTURE_WRAP_T,
                gl33::CLAMP_TO_EDGE as _,
            );
            check_err!();
        }

        Texture {
            handle,
            width,
            height,
            format: TextureFormat::RGBAU8,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn format(&self) -> TextureFormat {
        self.format
    }

    pub fn update_texel(&self, x: u32, y: u32, width: u32, height: u32, data: &[u8]) {
        unsafe {
            gl33::BindTexture(gl33::TEXTURE_2D, self.handle);
            check_err!();
            gl33::PixelStorei(gl33::UNPACK_ALIGNMENT, 1);
            check_err!();
            gl33::TexSubImage2D(
                gl33::TEXTURE_2D,
                0,
                x as _,
                y as _,
                width as _,
                height as _,
                match self.format {
                    TextureFormat::RI8 => gl33::RED,
                    TextureFormat::RGI8 => gl33::RG,
                    TextureFormat::RGBI8 => gl33::RGB,
                    TextureFormat::RGBAI8 => gl33::RGBA,
                    TextureFormat::RI16 => gl33::RED,
                    TextureFormat::RGI16 => gl33::RG,
                    TextureFormat::RGBI16 => gl33::RGB,
                    TextureFormat::RGBAI16 => gl33::RGBA,
                    TextureFormat::RI32 => gl33::RED,
                    TextureFormat::RGI32 => gl33::RG,
                    TextureFormat::RGBI32 => gl33::RGB,
                    TextureFormat::RGBAI32 => gl33::RGBA,
                    TextureFormat::RU8 => gl33::RED,
                    TextureFormat::RGU8 => gl33::RG,
                    TextureFormat::RGBU8 => gl33::RGB,
                    TextureFormat::RGBAU8 => gl33::RGBA,
                    TextureFormat::RU16 => gl33::RED,
                    TextureFormat::RGU16 => gl33::RG,
                    TextureFormat::RGBU16 => gl33::RGB,
                    TextureFormat::RGBAU16 => gl33::RGBA,
                    TextureFormat::RU32 => gl33::RED,
                    TextureFormat::RGU32 => gl33::RG,
                    TextureFormat::RGBU32 => gl33::RGB,
                    TextureFormat::RGBAU32 => gl33::RGBA,
                    TextureFormat::RF16 => gl33::RED,
                    TextureFormat::RGF16 => gl33::RG,
                    TextureFormat::RGBF16 => gl33::RGB,
                    TextureFormat::RGBAF16 => gl33::RGBA,
                    TextureFormat::RF32 => gl33::RED,
                    TextureFormat::RGF32 => gl33::RG,
                    TextureFormat::RGBF32 => gl33::RGB,
                    TextureFormat::RGBAF32 => gl33::RGBA,
                },
                match self.format {
                    TextureFormat::RI8 => gl33::BYTE,
                    TextureFormat::RGI8 => gl33::BYTE,
                    TextureFormat::RGBI8 => gl33::BYTE,
                    TextureFormat::RGBAI8 => gl33::BYTE,
                    TextureFormat::RI16 => gl33::SHORT,
                    TextureFormat::RGI16 => gl33::SHORT,
                    TextureFormat::RGBI16 => gl33::SHORT,
                    TextureFormat::RGBAI16 => gl33::SHORT,
                    TextureFormat::RI32 => gl33::INT,
                    TextureFormat::RGI32 => gl33::INT,
                    TextureFormat::RGBI32 => gl33::INT,
                    TextureFormat::RGBAI32 => gl33::INT,
                    TextureFormat::RU8 => gl33::UNSIGNED_BYTE,
                    TextureFormat::RGU8 => gl33::UNSIGNED_BYTE,
                    TextureFormat::RGBU8 => gl33::UNSIGNED_BYTE,
                    TextureFormat::RGBAU8 => gl33::UNSIGNED_BYTE,
                    TextureFormat::RU16 => gl33::UNSIGNED_SHORT,
                    TextureFormat::RGU16 => gl33::UNSIGNED_SHORT,
                    TextureFormat::RGBU16 => gl33::UNSIGNED_SHORT,
                    TextureFormat::RGBAU16 => gl33::UNSIGNED_SHORT,
                    TextureFormat::RU32 => gl33::UNSIGNED_INT,
                    TextureFormat::RGU32 => gl33::UNSIGNED_INT,
                    TextureFormat::RGBU32 => gl33::UNSIGNED_INT,
                    TextureFormat::RGBAU32 => gl33::UNSIGNED_INT,
                    TextureFormat::RF16 => gl33::FLOAT,
                    TextureFormat::RGF16 => gl33::FLOAT,
                    TextureFormat::RGBF16 => gl33::FLOAT,
                    TextureFormat::RGBAF16 => gl33::FLOAT,
                    TextureFormat::RF32 => gl33::FLOAT,
                    TextureFormat::RGF32 => gl33::FLOAT,
                    TextureFormat::RGBF32 => gl33::FLOAT,
                    TextureFormat::RGBAF32 => gl33::FLOAT,
                },
                data.as_ptr() as _,
            );
            check_err!();
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl33::DeleteTextures(1, &self.handle as _);
            check_err!();
        }
    }
}

impl Object for Texture {
    fn handle(&self) -> NativeHandle {
        NativeHandle(self.handle)
    }
}

use crate::{NativeHandle, Object};
use gl33::types::*;
use std::mem::size_of;
use std::ptr::null;

#[derive(Debug)]
pub struct Buffer {
    handle: GLuint,
    size: usize,
}

impl Buffer {
    pub fn empty() -> Self {
        Self {
            handle: ptr_init!(ptr => gl33::GenBuffers(1, ptr)),
            size: 0,
        }
    }

    pub fn with_size(size: usize) -> Self {
        let handle = ptr_init!(ptr => gl33::GenBuffers(1, ptr));

        unsafe {
            gl33::BindBuffer(gl33::ARRAY_BUFFER, handle);
            gl33::BufferData(gl33::ARRAY_BUFFER, size as _, null(), gl33::STATIC_DRAW);
        }

        Self { handle, size }
    }

    pub fn from_slice<T>(data: &[T]) -> Self {
        let handle = ptr_init!(ptr => gl33::GenBuffers(1, ptr));
        let size = size_of::<T>() * data.len();

        unsafe {
            gl33::BindBuffer(gl33::ARRAY_BUFFER, handle);
            check_err!();
            gl33::BufferData(
                gl33::ARRAY_BUFFER,
                size as _,
                data.as_ptr() as _,
                gl33::STATIC_DRAW,
            );
            check_err!();
        }

        Self { handle, size }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn update<T>(&self, offset: u32, data: &[T])
    where
        T: 'static,
    {
        unsafe {
            gl33::BindBuffer(gl33::ARRAY_BUFFER, self.handle);
            check_err!();
            gl33::BufferSubData(
                gl33::ARRAY_BUFFER,
                offset as _,
                (size_of::<T>() * data.len()) as _,
                data.as_ptr() as _,
            );
            check_err!();
        }
    }

    pub fn replace<T>(&mut self, data: &[T])
    where
        T: 'static,
    {
        self.size = size_of::<T>() * data.len();

        unsafe {
            gl33::BindBuffer(gl33::ARRAY_BUFFER, self.handle);
            check_err!();
            gl33::BufferData(
                gl33::ARRAY_BUFFER,
                self.size as _,
                data.as_ptr() as _,
                gl33::STATIC_DRAW,
            );
            check_err!();
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl33::DeleteBuffers(1, &self.handle as _);
            check_err!();
        }
    }
}

impl Object for Buffer {
    fn handle(&self) -> NativeHandle {
        NativeHandle(self.handle)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::empty()
    }
}

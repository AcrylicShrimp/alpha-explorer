use crate::{
    NativeHandle, Object, ShaderAttribute, ShaderAttributeType, ShaderUniform, ShaderUniformBlock,
    ShaderUniformType,
};
use gl33::types::*;
use std::any::type_name;
use std::ffi::CString;
use std::mem::forget;
use std::ptr::{null, null_mut};
use std::str::from_utf8;

#[derive(Debug)]
pub struct Shader {
    handle: GLuint,
    format_handle: GLuint,
    uniforms: Vec<ShaderUniform>,
    uniform_blocks: Vec<ShaderUniformBlock>,
    attributes: Vec<ShaderAttribute>,
}

impl Shader {
    pub fn from_source(
        vertex: &str,
        fragment: &str,
    ) -> (
        Result<Self, String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) {
        let (vertex_shader, vertex_shader_log) =
            Shader::compile_shader(vertex, gl33::VERTEX_SHADER);
        let vertex_shader = match vertex_shader {
            Ok(shader) => shader,
            Err(err) => return (Err(err), vertex_shader_log, None, None),
        };

        let (fragment_shader, fragment_shader_log) =
            Shader::compile_shader(fragment, gl33::FRAGMENT_SHADER);
        let fragment_shader = match fragment_shader {
            Ok(shader) => shader,
            Err(err) => return (Err(err), vertex_shader_log, fragment_shader_log, None),
        };

        let handle = unsafe { gl33::CreateProgram() };
        let format_handle = ptr_init!(ptr => gl33::GenVertexArrays(1, ptr));

        unsafe {
            gl33::BindVertexArray(format_handle);
            check_err!();
        }

        unsafe {
            gl33::AttachShader(handle, vertex_shader);
            check_err!();
            gl33::AttachShader(handle, fragment_shader);
            check_err!();
            gl33::LinkProgram(handle);
            check_err!();
        }

        let len = ptr_init!(ptr => gl33::GetProgramiv(handle, gl33::INFO_LOG_LENGTH, ptr));
        let log = if len == 0 {
            None
        } else {
            let mut buffer = vec![0; len as _];

            unsafe {
                gl33::GetProgramInfoLog(
                    handle,
                    buffer.len() as _,
                    null_mut(),
                    buffer.as_mut_ptr() as _,
                );
                check_err!();
            }

            Some(match String::from_utf8(buffer) {
                Ok(log) => log,
                Err(err) => {
                    return (
                        Err(format!(
                    "invalid utf-8 sequence detected while retrieving a program log for {}: {}",
                    type_name::<Self>(),
                    err
                )),
                        vertex_shader_log,
                        fragment_shader_log,
                        None,
                    )
                }
            })
        };

        let status = ptr_init!(ptr => gl33::GetProgramiv(handle, gl33::LINK_STATUS, ptr));

        if status != gl33::TRUE as _ {
            return (
                Err(format!(
                    "failed to link shader program (handle={})",
                    handle
                )),
                vertex_shader_log,
                fragment_shader_log,
                log,
            );
        }

        let uniforms;
        let uniform_blocks;
        let attributes;

        let len = ptr_init!(
            ptr => gl33::GetProgramiv(handle, gl33::ACTIVE_UNIFORMS, ptr)
        ) as _;
        let mut name_buffer = Vec::with_capacity(ptr_init!(
            ptr => gl33::GetProgramiv(handle, gl33::ACTIVE_UNIFORM_MAX_LENGTH, ptr)
        ) as _);

        uniforms = match (0..len)
            .map(|index| {
                let (ty, count, name_len) = ptr_init!(
                    ty, count, name_len => gl33::GetActiveUniform(
                    handle,
                    index,
                    name_buffer.capacity() as _,
                    name_len,
                    count,
                    ty,
                    name_buffer.as_mut_ptr() as _,
                ));

                unsafe {
                    name_buffer.set_len(name_len as _);
                }

                let name = from_utf8(&name_buffer).map_err(|err| {
                    format!(
                        "invalid utf-8 sequence detected while retrieving a uniform name at index {} for {}: {}",
                        index,
                        type_name::<Self>(),
                        err
                    )
                })?;
                let location = unsafe { gl33::GetUniformLocation(handle, name_buffer.as_ptr() as _) } as _;

                Ok(ShaderUniform{
                    name: name.to_owned(),
                    location,
                    ty: match ty {
                        gl33::FLOAT => ShaderUniformType::F1,
                        gl33::FLOAT_VEC2 => ShaderUniformType::F2,
                        gl33::FLOAT_VEC3 => ShaderUniformType::F3,
                        gl33::FLOAT_VEC4 => ShaderUniformType::F4,
                        gl33::INT => ShaderUniformType::I1,
                        gl33::INT_VEC2 => ShaderUniformType::I2,
                        gl33::INT_VEC3 => ShaderUniformType::I3,
                        gl33::INT_VEC4 => ShaderUniformType::I4,
                        gl33::UNSIGNED_INT => ShaderUniformType::U1,
                        gl33::UNSIGNED_INT_VEC2 => ShaderUniformType::U2,
                        gl33::UNSIGNED_INT_VEC3 => ShaderUniformType::U3,
                        gl33::UNSIGNED_INT_VEC4 => ShaderUniformType::U4,
                        gl33::BOOL => ShaderUniformType::B1,
                        gl33::BOOL_VEC2 => ShaderUniformType::B2,
                        gl33::BOOL_VEC3 => ShaderUniformType::B3,
                        gl33::BOOL_VEC4 => ShaderUniformType::B4,
                        gl33::FLOAT_MAT2 => ShaderUniformType::F22,
                        gl33::FLOAT_MAT3 => ShaderUniformType::F33,
                        gl33::FLOAT_MAT4 => ShaderUniformType::F44,
                        gl33::FLOAT_MAT2x3 => ShaderUniformType::F23,
                        gl33::FLOAT_MAT2x4 => ShaderUniformType::F24,
                        gl33::FLOAT_MAT3x2 => ShaderUniformType::F32,
                        gl33::FLOAT_MAT3x4 => ShaderUniformType::F34,
                        gl33::FLOAT_MAT4x2 => ShaderUniformType::F42,
                        gl33::FLOAT_MAT4x3 => ShaderUniformType::F43,
                        gl33::SAMPLER_2D => ShaderUniformType::Sampler,
                        _ => {
                            return Err(format!(
                                "invalid uniform '{}': {} is not supported uniform type",
                                name, ty
                            ))
                        }
                    },
                    count: count as _,
                })
            })
            .into_iter()
            .collect::<Result<Vec<_>, _>>() {
                Ok(uniforms) => uniforms,
                Err(err) => return (Err(err), vertex_shader_log, fragment_shader_log, log),
            };

        let len = ptr_init!(
            ptr => gl33::GetProgramiv(handle, gl33::ACTIVE_UNIFORM_BLOCKS, ptr)
        ) as _;
        let mut name_buffer = Vec::with_capacity(ptr_init!(
            ptr => gl33::GetProgramiv(handle, gl33::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH, ptr)
        ) as _);

        uniform_blocks = match (0..len)
            .map(|index| {
                let name_len = ptr_init!(
                    name_len => gl33::GetActiveUniformBlockName(
                    handle,
                    index,
                    name_buffer.capacity() as _,
                    name_len,
                    name_buffer.as_mut_ptr() as _,
                ));

                unsafe {
                    name_buffer.set_len(name_len as _);
                }

                let name = from_utf8(&name_buffer).map_err(|err| {
                    format!(
                        "invalid utf-8 sequence detected while retrieving a uniform block name at index {} for {}: {}",
                        index,
                        type_name::<Self>(),
                        err
                    )
                })?;

                Ok(ShaderUniformBlock{name:name.to_owned(), index})
            })
            .into_iter()
            .collect::<Result<Vec<_>, String>>() {
                Ok(uniform_blocks) => uniform_blocks,
                Err(err) => return (Err(err), vertex_shader_log, fragment_shader_log, log),
            };

        let len = ptr_init!(
            ptr => gl33::GetProgramiv(handle, gl33::ACTIVE_ATTRIBUTES, ptr)
        ) as _;
        let mut name_buffer = Vec::with_capacity(ptr_init!(
            ptr => gl33::GetProgramiv(handle, gl33::ACTIVE_ATTRIBUTE_MAX_LENGTH, ptr)
        ) as _);

        attributes = match (0..len)
            .map(|index| {
                let (ty, count, name_len) = ptr_init!(
                    ty, count, name_len => gl33::GetActiveAttrib(
                    handle,
                    index,
                    name_buffer.capacity() as _,
                    name_len,
                    count,
                    ty,
                    name_buffer.as_mut_ptr() as _,
                ));

                unsafe {
                    name_buffer.set_len(name_len as _);
                }
                
                let name = from_utf8(&name_buffer).map_err(|err| {
                    format!(
                        "invalid utf-8 sequence detected while retrieving a attribute name at index {} for {}: {}",
                        index,
                        type_name::<Self>(),
                        err
                    )
                })?;
                let ty = match ty {
                    gl33::FLOAT => ShaderAttributeType::F1,
                    gl33::FLOAT_VEC2 => ShaderAttributeType::F2,
                    gl33::FLOAT_VEC3 => ShaderAttributeType::F3,
                    gl33::FLOAT_VEC4 => ShaderAttributeType::F4,
                    gl33::INT => ShaderAttributeType::I1,
                    gl33::INT_VEC2 => ShaderAttributeType::I2,
                    gl33::INT_VEC3 => ShaderAttributeType::I3,
                    gl33::INT_VEC4 => ShaderAttributeType::I4,
                    gl33::UNSIGNED_INT => ShaderAttributeType::U1,
                    gl33::UNSIGNED_INT_VEC2 => ShaderAttributeType::U2,
                    gl33::UNSIGNED_INT_VEC3 => ShaderAttributeType::U3,
                    gl33::UNSIGNED_INT_VEC4 => ShaderAttributeType::U4,
                    gl33::FLOAT_MAT2 => ShaderAttributeType::F22,
                    gl33::FLOAT_MAT3 => ShaderAttributeType::F33,
                    gl33::FLOAT_MAT4 => ShaderAttributeType::F44,
                    gl33::FLOAT_MAT2x3 => ShaderAttributeType::F23,
                    gl33::FLOAT_MAT2x4 => ShaderAttributeType::F24,
                    gl33::FLOAT_MAT3x2 => ShaderAttributeType::F32,
                    gl33::FLOAT_MAT3x4 => ShaderAttributeType::F34,
                    gl33::FLOAT_MAT4x2 => ShaderAttributeType::F42,
                    gl33::FLOAT_MAT4x3 => ShaderAttributeType::F43,
                    _ => {
                        return Err(format!(
                            "invalid attribute '{}': {} is not supported attribute type",
                            name, ty
                        ))
                    }
                };
                let location = unsafe { gl33::GetAttribLocation(handle, name_buffer.as_ptr() as _) } as _;

                for offset in 0..ty.component_count() {
                    unsafe {
                        gl33::EnableVertexAttribArray(location + offset);
                        check_err!();
                    }
                }

                Ok(ShaderAttribute{name:name.to_owned(), location, ty, count:count as _})
            })
            .into_iter()
            .collect::<Result<Vec<_>, _>>() {
                Ok(attributes) => attributes,
                Err(err) => return (Err(err), vertex_shader_log, fragment_shader_log, log),
            };

        unsafe {
            gl33::DeleteShader(vertex_shader);
            check_err!();
            gl33::DeleteShader(fragment_shader);
            check_err!();
        }

        (
            Ok(Self {
                handle,
                format_handle,
                uniforms,
                uniform_blocks,
                attributes,
            }),
            vertex_shader_log,
            fragment_shader_log,
            log,
        )
    }

    pub fn format_handle(&self) -> NativeHandle {
        NativeHandle(self.format_handle)
    }

    pub fn uniform(&self, name: impl AsRef<str>) -> Option<&ShaderUniform> {
        self.uniforms.iter().find(|uniform| uniform.name == name.as_ref())
    }

    pub fn uniform_block(&self, name: impl AsRef<str>) -> Option<&ShaderUniformBlock> {
        self.uniform_blocks.iter().find(|uniform_block| uniform_block.name == name.as_ref())
    }
    
    pub fn attribute(&self, name: impl AsRef<str>) -> Option<&ShaderAttribute> {
        self.attributes.iter().find(|attribute| attribute.name == name.as_ref())
    }

    fn compile_shader(src: &str, ty: GLenum) -> (Result<GLuint, String>, Option<String>) {
        struct ShaderHandle(pub GLuint);

        impl ShaderHandle {
            pub fn new(ty: GLenum) -> Self {
                Self(unsafe { gl33::CreateShader(ty) })
            }

            pub fn into_handle(self) -> GLuint {
                let handle = self.0;
                forget(self);
                handle
            }
        }

        impl Drop for ShaderHandle {
            fn drop(&mut self) {
                unsafe {
                    gl33::DeleteShader(self.0);
                    check_err!();
                }
            }
        }

        let src = match CString::new(src) {
            Ok(src) => src,
            Err(err) => {
                return (
                    Err(format!(
                        "unexpected null character detected while reading source for {}: {}",
                        type_name::<Self>(),
                        err
                    )),
                    None,
                )
            }
        };
        let handle = ShaderHandle::new(ty);

        unsafe {
            gl33::ShaderSource(handle.0, 1, &src.as_ptr(), null());
            check_err!();
            gl33::CompileShader(handle.0);
            check_err!();
        }

        let len = ptr_init!(ptr => gl33::GetShaderiv(handle.0, gl33::INFO_LOG_LENGTH, ptr));
        let log = if len == 0 {
            None
        } else {
            let mut buffer = vec![0; len as _];

            unsafe {
                gl33::GetShaderInfoLog(
                    handle.0,
                    buffer.len() as _,
                    null_mut(),
                    buffer.as_mut_ptr() as _,
                );
                check_err!();
            }

            Some(match String::from_utf8(buffer) {
                Ok(log) => log,
                Err(err) => {
                    return (
                        Err(format!(
                    "invalid utf-8 sequence detected while retrieving a shader log for {}: {}",
                    type_name::<Self>(),
                    err
                )),
                        None,
                    )
                }
            })
        };

        let status = ptr_init!(ptr => gl33::GetShaderiv(handle.0, gl33::COMPILE_STATUS, ptr));

        (
            if status != gl33::TRUE as _ {
                Err(format!(
                    "failed to compile shader (handle={})",
                    handle.0
                ))
            } else {
                Ok(handle.into_handle())
            },
            log,
        )
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl33::DeleteVertexArrays(1, &self.format_handle);
            check_err!();
            gl33::DeleteProgram(self.handle);
            check_err!();
        }
    }
}

impl Object for Shader {
    fn handle(&self) -> NativeHandle {
        NativeHandle(self.handle)
    }
}

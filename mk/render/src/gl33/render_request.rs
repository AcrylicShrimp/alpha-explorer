use crate::{Buffer, NativeHandle, Object, RenderMode, Shader, ShaderAttributeType, Texture};
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use std::mem::size_of;

#[derive(Debug, Clone)]
enum UniformValue<'bump> {
    B1(bool),
    B2(bool, bool),
    B3(bool, bool, bool),
    B4(bool, bool, bool, bool),
    I1(i32),
    I2(i32, i32),
    I3(i32, i32, i32),
    I4(i32, i32, i32, i32),
    U1(u32),
    U2(u32, u32),
    U3(u32, u32, u32),
    U4(u32, u32, u32, u32),
    F1(f32),
    F2(f32, f32),
    F3(f32, f32, f32),
    F4(f32, f32, f32, f32),
    F22([f32; 4]),
    F23([f32; 6]),
    F24([f32; 8]),
    F32([f32; 6]),
    F33([f32; 9]),
    F34([f32; 12]),
    F42([f32; 8]),
    F43([f32; 12]),
    F44([f32; 16]),
    Sampler(NativeHandle),
    SamplerArray(BumpVec<'bump, NativeHandle>),
}

#[derive(Debug, Clone)]
struct UniformRenderRequest<'bump> {
    pub location: u32,
    pub value: UniformValue<'bump>,
}

#[derive(Debug, Clone)]
struct UniformBlockRenderRequest {
    pub location: u32,
    pub buffer: NativeHandle,
}

#[derive(Debug, Clone)]
struct AttributeRenderRequest {
    pub location: u32,
    pub buffer: NativeHandle,
    pub offset: u32,
    pub ty: ShaderAttributeType,
    pub per_instance: bool,
}

#[derive(Debug, Clone)]
pub struct RenderRequest<'bump> {
    bump: &'bump Bump,
    instance_count: u32,
    primitive_count: u32,
    mode: RenderMode,
    shader: NativeHandle,
    shader_format: NativeHandle,
    uniforms: BumpVec<'bump, UniformRenderRequest<'bump>>,
    uniform_blocks: BumpVec<'bump, UniformBlockRenderRequest>,
    attributes: BumpVec<'bump, AttributeRenderRequest>,
}

impl<'bump> RenderRequest<'bump> {
    pub fn new(
        bump: &'bump Bump,
        instance_count: u32,
        primitive_count: u32,
        mode: RenderMode,
        shader: &Shader,
    ) -> Self {
        Self {
            bump,
            instance_count,
            primitive_count,
            mode,
            shader: shader.handle(),
            shader_format: shader.format_handle(),
            uniforms: BumpVec::new_in(bump),
            uniform_blocks: BumpVec::new_in(bump),
            attributes: BumpVec::new_in(bump),
        }
    }

    pub fn uniform_b1(&mut self, location: u32, b0: bool) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::B1(b0),
        });
    }

    pub fn uniform_b2(&mut self, location: u32, b0: bool, b1: bool) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::B2(b0, b1),
        });
    }

    pub fn uniform_b3(&mut self, location: u32, b0: bool, b1: bool, b2: bool) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::B3(b0, b1, b2),
        });
    }

    pub fn uniform_b4(&mut self, location: u32, b0: bool, b1: bool, b2: bool, b3: bool) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::B4(b0, b1, b2, b3),
        });
    }

    pub fn uniform_i1(&mut self, location: u32, i0: i32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::I1(i0),
        });
    }

    pub fn uniform_i2(&mut self, location: u32, i0: i32, i1: i32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::I2(i0, i1),
        });
    }

    pub fn uniform_i3(&mut self, location: u32, i0: i32, i1: i32, i2: i32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::I3(i0, i1, i2),
        });
    }

    pub fn uniform_i4(&mut self, location: u32, i0: i32, i1: i32, i2: i32, i3: i32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::I4(i0, i1, i2, i3),
        });
    }

    pub fn uniform_u1(&mut self, location: u32, u0: u32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::U1(u0),
        });
    }

    pub fn uniform_u2(&mut self, location: u32, u0: u32, u1: u32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::U2(u0, u1),
        });
    }

    pub fn uniform_u3(&mut self, location: u32, u0: u32, u1: u32, u2: u32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::U3(u0, u1, u2),
        });
    }

    pub fn uniform_u4(&mut self, location: u32, u0: u32, u1: u32, u2: u32, u3: u32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::U4(u0, u1, u2, u3),
        });
    }

    pub fn uniform_f1(&mut self, location: u32, f0: f32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F1(f0),
        });
    }

    pub fn uniform_f2(&mut self, location: u32, f0: f32, f1: f32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F2(f0, f1),
        });
    }

    pub fn uniform_f3(&mut self, location: u32, f0: f32, f1: f32, f2: f32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F3(f0, f1, f2),
        });
    }

    pub fn uniform_f4(&mut self, location: u32, f0: f32, f1: f32, f2: f32, f3: f32) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F4(f0, f1, f2, f3),
        });
    }

    pub fn uniform_f22(&mut self, location: u32, f: [f32; 4]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F22(f),
        });
    }

    pub fn uniform_f23(&mut self, location: u32, f: [f32; 6]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F23(f),
        });
    }

    pub fn uniform_f24(&mut self, location: u32, f: [f32; 8]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F24(f),
        });
    }

    pub fn uniform_f32(&mut self, location: u32, f: [f32; 6]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F32(f),
        });
    }

    pub fn uniform_f33(&mut self, location: u32, f: [f32; 9]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F33(f),
        });
    }

    pub fn uniform_f34(&mut self, location: u32, f: [f32; 12]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F34(f),
        });
    }

    pub fn uniform_f42(&mut self, location: u32, f: [f32; 8]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F42(f),
        });
    }

    pub fn uniform_f43(&mut self, location: u32, f: [f32; 12]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F43(f),
        });
    }

    pub fn uniform_f44(&mut self, location: u32, f: [f32; 16]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::F44(f),
        });
    }

    pub fn uniform_texture(&mut self, location: u32, texture: &Texture) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::Sampler(texture.handle()),
        });
    }

    pub fn uniform_texture_raw(&mut self, location: u32, texture: NativeHandle) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::Sampler(texture),
        });
    }

    pub fn uniform_texture_array(&mut self, location: u32, textures: &[Texture]) {
        self.uniforms.push(UniformRenderRequest {
            location,
            value: UniformValue::SamplerArray(BumpVec::from_iter_in(
                textures.into_iter().map(|texture| texture.handle()),
                self.bump,
            )),
        });
    }

    pub fn uniform_block(&mut self, location: u32, buffer: &Buffer) {
        self.uniform_blocks.push(UniformBlockRenderRequest {
            location,
            buffer: buffer.handle(),
        });
    }

    pub fn attribute(
        &mut self,
        location: u32,
        buffer: &Buffer,
        offset: u32,
        ty: ShaderAttributeType,
    ) {
        self.attributes.push(AttributeRenderRequest {
            location,
            buffer: buffer.handle(),
            offset,
            ty,
            per_instance: false,
        });
    }

    pub fn attribute_per_instance(
        &mut self,
        location: u32,
        buffer: &Buffer,
        offset: u32,
        ty: ShaderAttributeType,
    ) {
        self.attributes.push(AttributeRenderRequest {
            location,
            buffer: buffer.handle(),
            offset,
            ty,
            per_instance: true,
        });
    }

    pub fn render(&self) {
        unsafe {
            gl33::BindVertexArray(self.shader_format.0);
            check_err!();
            gl33::UseProgram(self.shader.0);
            check_err!();
        }

        for uniform_block in &self.uniform_blocks {
            unsafe {
                gl33::UniformBlockBinding(
                    self.shader.0,
                    uniform_block.location,
                    uniform_block.location,
                );
                check_err!();
                gl33::BindBufferBase(
                    gl33::UNIFORM_BUFFER,
                    uniform_block.location,
                    uniform_block.buffer.0,
                );
                check_err!();
            }
        }

        let mut texture_unit = 0;

        for uniform in &self.uniforms {
            unsafe {
                match &uniform.value {
                    &UniformValue::B1(b0) => {
                        gl33::Uniform1ui(uniform.location as _, b0 as _);
                    }
                    &UniformValue::B2(b0, b1) => {
                        gl33::Uniform2ui(uniform.location as _, b0 as _, b1 as _);
                    }
                    &UniformValue::B3(b0, b1, b2) => {
                        gl33::Uniform3ui(uniform.location as _, b0 as _, b1 as _, b2 as _);
                    }
                    &UniformValue::B4(b0, b1, b2, b3) => {
                        gl33::Uniform4ui(uniform.location as _, b0 as _, b1 as _, b2 as _, b3 as _);
                    }
                    &UniformValue::I1(i0) => {
                        gl33::Uniform1i(uniform.location as _, i0);
                    }
                    &UniformValue::I2(i0, i1) => {
                        gl33::Uniform2i(uniform.location as _, i0, i1);
                    }
                    &UniformValue::I3(i0, i1, i2) => {
                        gl33::Uniform3i(uniform.location as _, i0, i1, i2);
                    }
                    &UniformValue::I4(i0, i1, i2, i3) => {
                        gl33::Uniform4i(uniform.location as _, i0, i1, i2, i3);
                    }
                    &UniformValue::U1(u0) => {
                        gl33::Uniform1ui(uniform.location as _, u0);
                    }
                    &UniformValue::U2(u0, u1) => {
                        gl33::Uniform2ui(uniform.location as _, u0, u1);
                    }
                    &UniformValue::U3(u0, u1, u2) => {
                        gl33::Uniform3ui(uniform.location as _, u0, u1, u2);
                    }
                    &UniformValue::U4(u0, u1, u2, u3) => {
                        gl33::Uniform4ui(uniform.location as _, u0, u1, u2, u3);
                    }
                    &UniformValue::F1(f0) => {
                        gl33::Uniform1f(uniform.location as _, f0);
                    }
                    &UniformValue::F2(f0, f1) => {
                        gl33::Uniform2f(uniform.location as _, f0, f1);
                    }
                    &UniformValue::F3(f0, f1, f2) => {
                        gl33::Uniform3f(uniform.location as _, f0, f1, f2);
                    }
                    &UniformValue::F4(f0, f1, f2, f3) => {
                        gl33::Uniform4f(uniform.location as _, f0, f1, f2, f3);
                    }
                    UniformValue::F22(f) => {
                        gl33::UniformMatrix2fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::F23(f) => {
                        gl33::UniformMatrix2x3fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::F24(f) => {
                        gl33::UniformMatrix2x4fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::F32(f) => {
                        gl33::UniformMatrix3x2fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::F33(f) => {
                        gl33::UniformMatrix3fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::F34(f) => {
                        gl33::UniformMatrix3x4fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::F42(f) => {
                        gl33::UniformMatrix4x2fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::F43(f) => {
                        gl33::UniformMatrix4x3fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::F44(f) => {
                        gl33::UniformMatrix4fv(uniform.location as _, 1, 0, f.as_ptr());
                    }
                    UniformValue::Sampler(texture) => {
                        gl33::ActiveTexture(gl33::TEXTURE0 + texture_unit);
                        check_err!();
                        gl33::BindTexture(gl33::TEXTURE_2D, texture.0);
                        check_err!();
                        gl33::Uniform1i(uniform.location as _, texture_unit as _);
                        check_err!();
                        texture_unit += 1;
                    }
                    UniformValue::SamplerArray(textures) => {
                        let mut texture_units = Vec::new();

                        for texture in textures.iter() {
                            gl33::ActiveTexture(gl33::TEXTURE0 + texture_unit);
                            check_err!();
                            gl33::BindTexture(gl33::TEXTURE_2D, texture.0);
                            check_err!();
                            texture_units.push(texture_unit);
                            texture_unit += 1;
                        }

                        gl33::Uniform1iv(
                            uniform.location as _,
                            texture_units.len() as _,
                            texture_units.as_ptr() as _,
                        );
                    }
                }
            }
        }

        let mut buffer_strides = BumpVec::with_capacity_in(4, self.bump);

        for attribute in self.attributes.iter() {
            match buffer_strides
                .iter()
                .position(|(buffer, _): &(NativeHandle, i32)| *buffer == attribute.buffer)
            {
                Some(position) => {
                    buffer_strides[position].1 += attribute.ty.size() as i32;
                }
                None => buffer_strides.push((attribute.buffer, attribute.ty.size() as i32)),
            }
        }

        unsafe {
            for attribute in &self.attributes {
                gl33::BindBuffer(gl33::ARRAY_BUFFER, attribute.buffer.0);
                check_err!();

                match attribute.ty {
                    ShaderAttributeType::I1
                    | ShaderAttributeType::I2
                    | ShaderAttributeType::I3
                    | ShaderAttributeType::I4 => {
                        gl33::VertexAttribIPointer(
                            attribute.location,
                            attribute.ty.component() as _,
                            gl33::INT,
                            buffer_strides
                                .iter()
                                .find(|(buffer, _)| *buffer == attribute.buffer)
                                .unwrap()
                                .1,
                            attribute.offset as _,
                        );
                        check_err!();
                        gl33::VertexAttribDivisor(attribute.location, attribute.per_instance as _);
                        check_err!();
                    }
                    ShaderAttributeType::U1
                    | ShaderAttributeType::U2
                    | ShaderAttributeType::U3
                    | ShaderAttributeType::U4 => {
                        gl33::VertexAttribIPointer(
                            attribute.location,
                            attribute.ty.component() as _,
                            gl33::UNSIGNED_INT,
                            buffer_strides
                                .iter()
                                .find(|(buffer, _)| *buffer == attribute.buffer)
                                .unwrap()
                                .1,
                            attribute.offset as _,
                        );
                        check_err!();
                        gl33::VertexAttribDivisor(attribute.location, attribute.per_instance as _);
                        check_err!();
                    }
                    ShaderAttributeType::F1
                    | ShaderAttributeType::F2
                    | ShaderAttributeType::F3
                    | ShaderAttributeType::F4 => {
                        gl33::VertexAttribPointer(
                            attribute.location,
                            attribute.ty.component() as _,
                            gl33::FLOAT,
                            0,
                            buffer_strides
                                .iter()
                                .find(|(buffer, _)| *buffer == attribute.buffer)
                                .unwrap()
                                .1,
                            attribute.offset as _,
                        );
                        check_err!();
                        gl33::VertexAttribDivisor(attribute.location, attribute.per_instance as _);
                        check_err!();
                    }
                    ShaderAttributeType::F22
                    | ShaderAttributeType::F23
                    | ShaderAttributeType::F24
                    | ShaderAttributeType::F32
                    | ShaderAttributeType::F33
                    | ShaderAttributeType::F34
                    | ShaderAttributeType::F42
                    | ShaderAttributeType::F43
                    | ShaderAttributeType::F44 => {
                        let stride = buffer_strides
                            .iter()
                            .find(|(buffer, _)| *buffer == attribute.buffer)
                            .unwrap()
                            .1;

                        for component_offset in 0..attribute.ty.component_count() {
                            gl33::VertexAttribPointer(
                                attribute.location + component_offset,
                                attribute.ty.component() as _,
                                gl33::FLOAT,
                                0,
                                stride,
                                (attribute.offset
                                    + size_of::<f32>() as u32
                                        * component_offset
                                        * attribute.ty.component())
                                    as _,
                            );
                            check_err!();
                            gl33::VertexAttribDivisor(
                                attribute.location + component_offset,
                                attribute.per_instance as _,
                            );
                            check_err!();
                        }
                    }
                }
            }

            gl33::DrawArraysInstanced(
                match self.mode {
                    RenderMode::Trangles => gl33::TRIANGLES,
                },
                0,
                (self.primitive_count * self.mode.count()) as _,
                self.instance_count as _,
            );
            check_err!();
        }
    }
}

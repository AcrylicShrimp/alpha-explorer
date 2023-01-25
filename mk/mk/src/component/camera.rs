use crate::{
    gfx::{ClearMode, Color, Layer, RenderManager},
    handles::{BindGroupHandle, BufferHandle},
};
use specs::{prelude::*, Component};
use std::{mem::size_of, num::NonZeroU64};
use wgpu::{BindGroupEntry, BindingResource, BufferAddress, BufferBinding};

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Camera {
    pub layer: Layer,
    pub order: isize,
    pub clear_mode: ClearMode,
    pub clear_color: Color,
    transform_buffer: BufferHandle,
    bind_group: BindGroupHandle,
}

impl Camera {
    pub fn new(
        render_mgr: &RenderManager,
        layer: Layer,
        order: isize,
        clear_mode: ClearMode,
        clear_color: Color,
    ) -> Self {
        let transform_buffer = render_mgr
            .create_uniform_buffer_without_contents(size_of::<[f32; 12]>() as BufferAddress);
        let bind_group = render_mgr.create_bind_group(
            render_mgr.camera_bind_group_layout(),
            &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &transform_buffer,
                    offset: 0,
                    size: Some(NonZeroU64::new(transform_buffer.size()).unwrap()),
                }),
            }],
        );

        Self {
            layer,
            order,
            clear_mode,
            clear_color,
            transform_buffer,
            bind_group,
        }
    }

    pub fn tranform_buffer(&self) -> &BufferHandle {
        &self.transform_buffer
    }

    pub fn bind_group(&self) -> &BindGroupHandle {
        &self.bind_group
    }
}

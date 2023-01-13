use crate::{
    component::SpriteRendererBindGroupAllocator,
    gfx::{
        low::{
            DeviceAllocation, FrameMemoryAllocator, RenderPipelineAllocator,
            RenderPipelineFactoryProvider,
        },
        Texture,
    },
    handles::*,
    EngineContext, GfxContext,
};
use std::{borrow::Cow, iter::once, num::NonZeroU32};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BufferAddress, BufferUsages, CommandEncoder, CommandEncoderDescriptor,
    Extent3d, ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, ImageSubresourceRange, Origin3d,
    Queue, Sampler, SamplerDescriptor, ShaderModuleDescriptor, ShaderSource, SurfaceTexture,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};
use winit::dpi::PhysicalSize;

use super::low::HostAllocation;

pub struct RenderManager {
    gfx_context: GfxContext,
    pipeline_allocator: RenderPipelineAllocator,
    frame_memory_allocator: FrameMemoryAllocator,
    sprite_renderer_bind_group_allocator: SpriteRendererBindGroupAllocator,
    // stencil_texture: StencilTexture,
    // common_shader_input_buffer: Buffer,
}

impl RenderManager {
    pub fn new(gfx_context: GfxContext) -> Self {
        Self {
            frame_memory_allocator: FrameMemoryAllocator::new(&gfx_context),
            gfx_context,
            pipeline_allocator: RenderPipelineAllocator::new(),
            sprite_renderer_bind_group_allocator: SpriteRendererBindGroupAllocator::new(),
            // stencil_texture: StencilTexture::new(&gfx_context.device, &gfx_context.surface_config),
            // common_shader_input_buffer: Buffer::from_slice(&[
            //     0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,
            // ]),
        }
    }

    pub fn queue(&self) -> &Queue {
        &self.gfx_context.queue
    }

    // pub fn stencil_texture(&self) -> &StencilTexture {
    //     &self.stencil_texture
    // }

    pub fn pipeline_allocator(&self) -> &RenderPipelineAllocator {
        &self.pipeline_allocator
    }

    pub fn register_pipeline_factory<T>(&mut self)
    where
        T: RenderPipelineFactoryProvider,
    {
        self.pipeline_allocator
            .register_factory::<T>(&self.gfx_context)
    }

    pub fn allocate_pipeline<T>(&mut self, shader: &ShaderHandle) -> PipelineHandle
    where
        T: RenderPipelineFactoryProvider,
    {
        PipelineHandle::wrap(
            self.pipeline_allocator
                .allocate::<T>(&self.gfx_context, shader.inner().clone()),
        )
    }

    pub fn allocate_sprite_renderer_bind_group(
        &mut self,
        old_sprite: Option<SpriteHandle>,
        new_sprite: &SpriteHandle,
    ) -> BindGroupHandle {
        self.sprite_renderer_bind_group_allocator.allocate(
            &self.gfx_context,
            &self.pipeline_allocator,
            old_sprite,
            new_sprite,
        )
    }

    pub fn create_render_output(&mut self) -> (SurfaceTexture, TextureView) {
        let surface_texture = self.gfx_context.surface.get_current_texture().unwrap();
        let surface_texture_view = surface_texture.texture.create_view(&Default::default());

        // Beginning of a frame; let's release single-framed memory allocations.
        self.frame_memory_allocator.release();

        (surface_texture, surface_texture_view)
    }

    pub fn create_encoder(&self) -> CommandEncoder {
        self.gfx_context
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None })
    }

    pub fn create_bind_group(
        &self,
        layout: &BindGroupLayout,
        entries: &[BindGroupEntry],
    ) -> BindGroupHandle {
        BindGroupHandle::new(
            self.gfx_context
                .device
                .create_bind_group(&BindGroupDescriptor {
                    label: None,
                    layout,
                    entries,
                }),
        )
    }

    pub fn create_bind_group_layout(&self, entries: &[BindGroupLayoutEntry]) -> BindGroupLayout {
        self.gfx_context
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries,
            })
    }

    pub fn create_vertex_buffer<T>(&self, contents: &[T]) -> BufferHandle
    where
        T: Sized,
    {
        let (_lhs, contents, _rhs) = unsafe { contents.align_to() };
        debug_assert!(_lhs.len() == 0);
        debug_assert!(_rhs.len() == 0);

        BufferHandle::new(
            self.gfx_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents,
                    usage: BufferUsages::VERTEX,
                }),
        )
    }

    pub fn create_host_buffer<T>(&mut self, contents: &[T]) -> HostAllocation
    where
        T: Sized,
    {
        self.frame_memory_allocator.allocate_host_buffer(contents)
    }

    pub fn create_single_frame_host_buffer<T>(&mut self, contents: &[T]) -> HostAllocation
    where
        T: Sized,
    {
        self.frame_memory_allocator.allocate_host_buffer(contents)
    }

    pub fn create_single_frame_host_buffer_without_contents(
        &mut self,
        size: usize,
    ) -> HostAllocation {
        self.frame_memory_allocator
            .allocate_host_buffer_without_contents(size)
    }

    pub fn create_single_frame_vertex_buffer<T>(&mut self, contents: &[T]) -> DeviceAllocation
    where
        T: Sized,
    {
        self.frame_memory_allocator
            .allocate_vertex_buffer(&self.gfx_context, contents)
    }

    pub fn create_single_frame_vertex_buffer_without_contents(
        &mut self,
        size: BufferAddress,
    ) -> DeviceAllocation {
        self.frame_memory_allocator
            .allocate_vertex_buffer_without_contents(&self.gfx_context, size)
    }

    pub fn create_uniform_buffer<T>(&self, contents: &[T]) -> BufferHandle
    where
        T: Sized,
    {
        let (_lhs, contents, _rhs) = unsafe { contents.align_to() };
        debug_assert!(_lhs.len() == 0);
        debug_assert!(_rhs.len() == 0);

        BufferHandle::new(
            self.gfx_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents,
                    usage: BufferUsages::UNIFORM,
                }),
        )
    }

    pub fn create_single_frame_uniform_buffer<T>(&mut self, contents: &[T]) -> DeviceAllocation
    where
        T: Sized,
    {
        self.frame_memory_allocator
            .allocate_uniform_buffer(&self.gfx_context, contents)
    }

    pub fn write_single_frame_device_buffer_contents<T>(
        &mut self,
        allocation: &DeviceAllocation,
        contents: &[T],
    ) where
        T: Sized,
    {
        self.frame_memory_allocator.write_device_allocation(
            &self.gfx_context,
            allocation,
            contents,
        );
    }

    pub fn create_sampler(&self, descriptor: &SamplerDescriptor) -> Sampler {
        self.gfx_context.device.create_sampler(descriptor)
    }

    pub fn create_shader(&self, source: impl AsRef<str>) -> ShaderHandle {
        ShaderHandle::new(
            self.gfx_context
                .device
                .create_shader_module(ShaderModuleDescriptor {
                    label: None,
                    source: ShaderSource::Wgsl(Cow::Borrowed(source.as_ref())),
                }),
        )
    }

    pub fn create_glyph_texture(&self, width: u16, height: u16) -> TextureHandle {
        let texture = self.gfx_context.device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: width as u32,
                height: height as u32,
                ..Default::default()
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R8Snorm,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
        });
        let mut encoder = self.create_encoder();
        encoder.clear_texture(
            &texture,
            &ImageSubresourceRange {
                aspect: TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: None,
                base_array_layer: 0,
                array_layer_count: None,
            },
        );
        self.gfx_context.queue.submit(once(encoder.finish()));
        TextureHandle::new(Texture {
            view: texture.create_view(&TextureViewDescriptor {
                ..Default::default()
            }),
            sampler: self.create_sampler(&Default::default()),
            texture,
            width,
            height,
        })
    }

    pub fn create_sprite_texture(&self, width: u16, height: u16, data: &[u8]) -> TextureHandle {
        let texture = self.gfx_context.device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: width as u32,
                height: height as u32,
                ..Default::default()
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
        });
        self.gfx_context.queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * width as u32),
                rows_per_image: NonZeroU32::new(height as u32),
            },
            Extent3d {
                width: width as u32,
                height: height as u32,
                ..Default::default()
            },
        );
        TextureHandle::new(Texture {
            view: texture.create_view(&TextureViewDescriptor {
                ..Default::default()
            }),
            sampler: self.create_sampler(&Default::default()),
            texture,
            width,
            height,
        })
    }

    pub fn update_glyph_texture(
        &self,
        texture: &TextureHandle,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        data: &[u8],
    ) {
        let buffer =
            self.gfx_context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: data,
                    usage: wgpu::BufferUsages::COPY_SRC,
                });
        let mut encoder = self.create_encoder();
        encoder.copy_buffer_to_texture(
            ImageCopyBuffer {
                buffer: &buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: NonZeroU32::new(width),
                    rows_per_image: NonZeroU32::new(height),
                },
            },
            ImageCopyTexture {
                texture: &texture.texture,
                mip_level: 0,
                origin: Origin3d { x, y, z: 0 },
                aspect: TextureAspect::All,
            },
            Extent3d {
                width,
                height,
                ..Default::default()
            },
        );
        self.gfx_context.queue.submit(once(encoder.finish()));
    }

    pub fn resize_gfx(&mut self, size: PhysicalSize<u32>) {
        self.gfx_context.surface_config.width = size.width;
        self.gfx_context.surface_config.height = size.height;
        self.gfx_context
            .surface
            .configure(&self.gfx_context.device, &self.gfx_context.surface_config);
    }

    pub fn update_uniforms(&self, _context: &EngineContext) {
        // let time_mgr = context.time_mgr();
        // let screen_mgr = context.screen_mgr();

        // self.common_shader_input_buffer.update(
        //     0,
        //     &[
        //         time_mgr.dt(),
        //         1f32 / time_mgr.dt(),
        //         time_mgr.time(),
        //         1f32 / time_mgr.time(),
        //         screen_mgr.width() as f32,
        //         screen_mgr.height() as f32,
        //         1f32 / screen_mgr.width() as f32,
        //         1f32 / screen_mgr.height() as f32,
        //     ],
        // );
    }

    pub fn submit_frame_memory_allocation(&mut self) -> CommandEncoder {
        self.frame_memory_allocator.submit(&self.gfx_context)
    }

    // pub fn apply_common_shader_input(&self, shader: &Shader, req: &mut RenderRequest) {
    //     // TODO: Add shader type checking logic to alert if types have no match.

    //     // if let Some(uniform) = shader.uniform("Common") {
    //     //     req.uniform_block(uniform.location, &self.common_shader_input_buffer);
    //     // }
    // }
}

mod sprite_renderer_bind_group_allocator;

pub use sprite_renderer_bind_group_allocator::*;

use crate::{
    gfx::{
        low::{RenderPipelineFactory, RenderPipelineFactoryProvider, RenderPipelineLayoutFactory},
        Color, Layer, RenderManager,
    },
    handles::*,
    GfxContext,
};
use specs::{prelude::*, Component};
use std::{
    mem::{replace, size_of},
    num::NonZeroU64,
};
use wgpu::{
    vertex_attr_array, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState,
    BufferAddress, BufferBindingType, ColorTargetState, ColorWrites, DepthStencilState, FrontFace,
    PolygonMode, PrimitiveState, PrimitiveTopology, SamplerBindingType, ShaderModule, ShaderStages,
    TextureSampleType, TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexStepMode,
};

#[derive(Component)]
pub struct SpriteRenderer {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub shader: ShaderHandle,
    sprite: SpriteHandle,
    bind_group: BindGroupHandle,
}

impl SpriteRenderer {
    pub fn new(
        render_mgr: &mut RenderManager,
        layer: Layer,
        order: i32,
        color: Color,
        shader: ShaderHandle,
        sprite: SpriteHandle,
    ) -> Self {
        Self {
            bind_group: render_mgr.allocate_sprite_renderer_bind_group(None, &sprite),
            layer,
            order,
            color,
            shader,
            sprite,
        }
    }

    pub fn sprite(&self) -> &SpriteHandle {
        &self.sprite
    }

    pub fn bind_group(&self) -> &BindGroupHandle {
        &self.bind_group
    }

    pub fn set_sprite(&mut self, render_mgr: &mut RenderManager, sprite: SpriteHandle) {
        let old_sprite = replace(&mut self.sprite, sprite);
        self.bind_group =
            render_mgr.allocate_sprite_renderer_bind_group(Some(old_sprite), &self.sprite);
    }
}

pub struct SpriteRenderPipelineFactoryProvider;

impl RenderPipelineFactoryProvider for SpriteRenderPipelineFactoryProvider {
    fn pipeline_layout_factory(
        _gfx_context: &GfxContext,
    ) -> Option<Box<dyn RenderPipelineLayoutFactory>> {
        Some(Box::new(SpriteRenderPipelineLayoutFactory))
    }

    fn pipeline_factory(_gfx_context: &GfxContext) -> Box<dyn RenderPipelineFactory> {
        Box::new(SpriteRenderPipelineFactory)
    }
}

pub struct SpriteRenderPipelineLayoutFactory;

impl SpriteRenderPipelineLayoutFactory {
    pub const SET_0_BIND_GROUP_LAYOUTS: [BindGroupLayoutEntry; 1] = [BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX_FRAGMENT,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: Some(unsafe {
                NonZeroU64::new_unchecked((size_of::<f32>() * 12) as u64)
            }),
        },
        count: None,
    }];
    pub const SET_1_BIND_GROUP_LAYOUTS: [BindGroupLayoutEntry; 2] = [
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Texture {
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Sampler(SamplerBindingType::Filtering),
            count: None,
        },
    ];
}

impl RenderPipelineLayoutFactory for SpriteRenderPipelineLayoutFactory {
    fn bind_group_layouts(&self, _gfx_context: &GfxContext) -> Vec<BindGroupLayoutDescriptor> {
        vec![
            BindGroupLayoutDescriptor {
                label: None,
                entries: &Self::SET_0_BIND_GROUP_LAYOUTS,
            },
            BindGroupLayoutDescriptor {
                label: None,
                entries: &Self::SET_1_BIND_GROUP_LAYOUTS,
            },
        ]
    }

    fn push_constant_ranges(&self, _gfx_context: &GfxContext) -> Vec<wgpu::PushConstantRange> {
        vec![]
    }
}

pub struct SpriteRenderPipelineFactory;

impl SpriteRenderPipelineFactory {
    pub const PER_VERTEX_STRIDE: BufferAddress = (size_of::<f32>() * 4) as BufferAddress;
    pub const PER_VERTEX_ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2
    ];

    pub const PER_INSTANCE_STRIDE: BufferAddress = (size_of::<f32>() * 19) as BufferAddress;
    pub const PER_INSTANCE_ATTRIBS: [VertexAttribute; 6] = vertex_attr_array![
        2 => Float32x3,
        3 => Float32x3,
        4 => Float32x3,
        5 => Float32x2,
        6 => Float32x4,
        7 => Float32x4
    ];
}

impl RenderPipelineFactory for SpriteRenderPipelineFactory {
    fn vertex_buffers(
        &self,
        _gfx_context: &GfxContext,
        _shader: &ShaderModule,
    ) -> Vec<VertexBufferLayout> {
        vec![
            VertexBufferLayout {
                array_stride: Self::PER_VERTEX_STRIDE,
                step_mode: VertexStepMode::Vertex,
                attributes: &Self::PER_VERTEX_ATTRIBS,
            },
            VertexBufferLayout {
                array_stride: Self::PER_INSTANCE_STRIDE,
                step_mode: VertexStepMode::Instance,
                attributes: &Self::PER_INSTANCE_ATTRIBS,
            },
        ]
    }

    fn primitive_state(&self, _gfx_context: &GfxContext, _shader: &ShaderModule) -> PrimitiveState {
        PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false,
        }
    }

    fn depth_stencil(
        &self,
        _gfx_context: &GfxContext,
        _shader: &ShaderModule,
    ) -> Option<DepthStencilState> {
        // let stencil_face_state = StencilFaceState {
        //     compare: CompareFunction::GreaterEqual,
        //     fail_op: StencilOperation::Keep,
        //     depth_fail_op: StencilOperation::Keep,
        //     pass_op: StencilOperation::Keep,
        // };
        // Some(DepthStencilState {
        //     format: TextureFormat::Depth24PlusStencil8,
        //     depth_write_enabled: false,
        //     depth_compare: CompareFunction::Always,
        //     stencil: StencilState {
        //         front: stencil_face_state,
        //         back: stencil_face_state,
        //         read_mask: 0xFF,
        //         write_mask: 0xFF,
        //     },
        //     bias: Default::default(),
        // })
        None
    }

    fn multisample(
        &self,
        _gfx_context: &GfxContext,
        _shader: &ShaderModule,
    ) -> wgpu::MultisampleState {
        Default::default()
    }

    fn fragment_targets(
        &self,
        gfx_context: &GfxContext,
        _shader: &ShaderModule,
    ) -> Vec<Option<ColorTargetState>> {
        vec![Some(ColorTargetState {
            format: gfx_context.surface_config.format,
            blend: Some(BlendState::ALPHA_BLENDING),
            write_mask: ColorWrites::ALL,
        })]
    }
}

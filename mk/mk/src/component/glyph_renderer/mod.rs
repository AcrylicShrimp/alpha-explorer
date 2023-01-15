mod glyph_renderer_bind_group_allocator;

pub use glyph_renderer_bind_group_allocator::*;

use crate::{
    gfx::{
        low::{RenderPipelineFactory, RenderPipelineFactoryProvider, RenderPipelineLayoutFactory},
        Color, GlyphLayoutConfig, GlyphManager, GlyphSprite, Layer, RenderManager,
    },
    handles::*,
    structure::Size,
    GfxContext,
};
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use specs::{prelude::*, Component};
use std::{mem::size_of, num::NonZeroU64};
use wgpu::*;

#[derive(Component)]
pub struct GlyphRenderer {
    pub layer: Layer,
    pub order: i32,
    pub color: Color,
    pub shader: ShaderHandle,
    pub thickness: f32,
    pub smoothness: f32,
    font: FontHandle,
    font_size: f32,
    config: GlyphLayoutConfig,
    layout: Layout,
    glyphs: Vec<Glyph>,
    text: String,
}

impl GlyphRenderer {
    pub fn new(
        layer: Layer,
        order: i32,
        color: Color,
        shader: ShaderHandle,
        thickness: f32,
        smoothness: f32,
        font: FontHandle,
        font_size: f32,
    ) -> Self {
        Self {
            layer,
            order,
            color,
            shader,
            thickness,
            smoothness,
            font,
            font_size,
            config: GlyphLayoutConfig::default(),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            glyphs: Vec::new(),
            text: String::new(),
        }
    }

    pub fn font(&self) -> &FontHandle {
        &self.font
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn config(&self) -> &GlyphLayoutConfig {
        &self.config
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn glyphs(&self) -> &[Glyph] {
        &self.glyphs
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn set_font(&mut self, font: FontHandle) {
        self.font = font;
        self.layout.clear();
        self.layout.append(
            &[self.font.inner().as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.layout.clear();
        self.layout.append(
            &[self.font.inner().as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn set_config(&mut self, config: GlyphLayoutConfig) {
        self.config = config;
        self.layout.reset(&LayoutSettings {
            x: 0f32,
            y: 0f32,
            max_width: None,
            max_height: None,
            horizontal_align: self.config.horizontal_align,
            vertical_align: self.config.vertical_align,
            wrap_style: self.config.wrap_style,
            wrap_hard_breaks: self.config.wrap_hard_breaks,
        });
        self.layout.append(
            &[self.font.inner().as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn set_text(
        &mut self,
        glyph_mgr: &mut GlyphManager,
        render_mgr: &mut RenderManager,
        text: String,
    ) {
        self.text = text;
        self.layout.clear();
        self.layout.append(
            &[self.font.inner().as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );

        self.update_glyphs(glyph_mgr, render_mgr);
    }

    pub fn compute_size(&self) -> Size {
        let mut width = 0f32;

        for glyph in self.layout.glyphs() {
            width = width.max(glyph.x + glyph.width as f32);
        }

        Size::new(width, self.layout.height())
    }

    fn update_glyphs(&mut self, glyph_mgr: &mut GlyphManager, render_mgr: &mut RenderManager) {
        let sprites = self
            .glyphs
            .iter()
            .map(|glyph| glyph.sprite.clone())
            .collect::<Vec<_>>();
        render_mgr.deallocate_glyph_renderer_bind_group(&sprites);
        self.glyphs.clear();
        for glyph in self.layout.glyphs() {
            let sprite = glyph_mgr.glyph(render_mgr, &self.font, glyph.key);
            let bind_group = render_mgr.allocate_glyph_renderer_bind_group(sprite);
            self.glyphs.push(Glyph::new(sprite.clone(), bind_group));
        }
    }
}

pub struct Glyph {
    sprite: GlyphSprite,
    bind_group: BindGroupHandle,
}

impl Glyph {
    pub fn new(sprite: GlyphSprite, bind_group: BindGroupHandle) -> Self {
        Self { sprite, bind_group }
    }

    pub fn sprite(&self) -> &GlyphSprite {
        &self.sprite
    }

    pub fn bind_group(&self) -> &BindGroupHandle {
        &self.bind_group
    }
}

pub struct GlyphRenderPipelineFactoryProvider;

impl RenderPipelineFactoryProvider for GlyphRenderPipelineFactoryProvider {
    fn pipeline_layout_factory(
        _gfx_context: &GfxContext,
    ) -> Option<Box<dyn RenderPipelineLayoutFactory>> {
        Some(Box::new(GlyphRenderPipelineLayoutFactory))
    }

    fn pipeline_factory(_gfx_context: &GfxContext) -> Box<dyn RenderPipelineFactory> {
        Box::new(GlyphRenderPipelineFactory)
    }
}

pub struct GlyphRenderPipelineLayoutFactory;

impl GlyphRenderPipelineLayoutFactory {
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

impl RenderPipelineLayoutFactory for GlyphRenderPipelineLayoutFactory {
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

pub struct GlyphRenderPipelineFactory;

impl GlyphRenderPipelineFactory {
    pub const PER_VERTEX_STRIDE: BufferAddress = (size_of::<f32>() * 4) as BufferAddress;
    pub const PER_VERTEX_ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2
    ];

    pub const PER_INSTANCE_STRIDE: BufferAddress = (size_of::<f32>() * 21) as BufferAddress;
    pub const PER_INSTANCE_ATTRIBS: [VertexAttribute; 8] = vertex_attr_array![
        2 => Float32x3,
        3 => Float32x3,
        4 => Float32x3,
        5 => Float32x2,
        6 => Float32x4,
        7 => Float32,
        8 => Float32,
        9 => Float32x4
    ];
}

impl RenderPipelineFactory for GlyphRenderPipelineFactory {
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
    ) -> Option<wgpu::DepthStencilState> {
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

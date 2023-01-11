use crate::GfxContext;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::Hash,
    sync::Arc,
};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, ColorTargetState, DepthStencilState, FragmentState,
    MultisampleState, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState, PushConstantRange,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, VertexBufferLayout, VertexState,
};

pub trait RenderPipelineFactoryProvider
where
    Self: Any,
{
    fn pipeline_layout_factory(
        gfx_context: &GfxContext,
    ) -> Option<Box<dyn RenderPipelineLayoutFactory>>;
    fn pipeline_factory(gfx_context: &GfxContext) -> Box<dyn RenderPipelineFactory>;
}

pub trait RenderPipelineLayoutFactory {
    fn bind_group_layouts(&self, gfx_context: &GfxContext) -> Vec<BindGroupLayoutDescriptor>;
    fn push_constant_ranges(&self, gfx_context: &GfxContext) -> Vec<PushConstantRange>;
}

pub trait RenderPipelineFactory {
    fn vertex_buffers(
        &self,
        gfx_context: &GfxContext,
        shader: &ShaderModule,
    ) -> Vec<VertexBufferLayout>;
    fn primitive_state(&self, gfx_context: &GfxContext, shader: &ShaderModule) -> PrimitiveState;
    fn depth_stencil(
        &self,
        gfx_context: &GfxContext,
        shader: &ShaderModule,
    ) -> Option<DepthStencilState>;
    fn multisample(&self, gfx_context: &GfxContext, shader: &ShaderModule) -> MultisampleState;
    fn fragment_targets(
        &self,
        gfx_context: &GfxContext,
        shader: &ShaderModule,
    ) -> Vec<Option<ColorTargetState>>;
}

pub struct RenderPipelineAllocator {
    cache: HashMap<CacheKey, Arc<RenderPipeline>>,
    layout_and_factories: HashMap<TypeId, LayoutAndFactory>,
}

impl RenderPipelineAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn layout_and_factory<T>(&self) -> &LayoutAndFactory
    where
        T: RenderPipelineFactoryProvider,
    {
        self.layout_and_factories.get(&TypeId::of::<T>()).unwrap()
    }

    pub fn register_factory<T>(&mut self, gfx_context: &GfxContext)
    where
        T: RenderPipelineFactoryProvider,
    {
        let (bind_group_layouts, pipeline_layout) = match T::pipeline_layout_factory(gfx_context) {
            Some(layout_factory) => {
                let bind_group_layouts = layout_factory
                    .bind_group_layouts(gfx_context)
                    .iter()
                    .map(|bind_group_layout| {
                        gfx_context
                            .device
                            .create_bind_group_layout(bind_group_layout)
                    })
                    .collect::<Vec<_>>();
                let pipeline_layout =
                    gfx_context
                        .device
                        .create_pipeline_layout(&PipelineLayoutDescriptor {
                            label: None,
                            bind_group_layouts: bind_group_layouts
                                .iter()
                                .collect::<Vec<_>>()
                                .as_slice(),
                            push_constant_ranges: &layout_factory.push_constant_ranges(gfx_context),
                        });
                (Some(bind_group_layouts), Some(pipeline_layout))
            }
            None => (None, None),
        };
        self.layout_and_factories.insert(
            TypeId::of::<T>(),
            LayoutAndFactory::new(
                bind_group_layouts,
                pipeline_layout,
                T::pipeline_factory(gfx_context),
            ),
        );
    }

    pub fn allocate<T>(
        &mut self,
        gfx_context: &GfxContext,
        shader: Arc<ShaderModule>,
    ) -> Arc<RenderPipeline>
    where
        T: RenderPipelineFactoryProvider,
    {
        let cache_key = CacheKey::new::<T>(shader);

        if let Some(pipeline) = self.cache.get(&cache_key) {
            return pipeline.clone();
        }

        let LayoutAndFactory {
            pipeline_layout,
            factory,
            ..
        } = self.layout_and_factories.get(&TypeId::of::<T>()).unwrap();
        let pipeline = gfx_context
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: pipeline_layout.as_ref(),
                vertex: VertexState {
                    module: &cache_key.shader,
                    entry_point: "vs_main",
                    buffers: &factory.vertex_buffers(gfx_context, &cache_key.shader),
                },
                primitive: factory.primitive_state(gfx_context, &cache_key.shader),
                depth_stencil: factory.depth_stencil(gfx_context, &cache_key.shader),
                multisample: factory.multisample(gfx_context, &cache_key.shader),
                fragment: Some(FragmentState {
                    module: &cache_key.shader,
                    entry_point: "fs_main",
                    targets: &factory.fragment_targets(gfx_context, &cache_key.shader),
                }),
                multiview: None,
            });

        self.cache
            .entry(cache_key)
            .or_insert(Arc::new(pipeline))
            .clone()
    }
}

impl Default for RenderPipelineAllocator {
    fn default() -> Self {
        Self {
            cache: HashMap::with_capacity(32),
            layout_and_factories: HashMap::with_capacity(8),
        }
    }
}

#[derive(Debug, Clone)]
struct CacheKey {
    pub type_id: TypeId,
    pub shader: Arc<ShaderModule>,
}

impl CacheKey {
    pub fn new<T>(shader: Arc<ShaderModule>) -> Self
    where
        T: Any,
    {
        Self {
            type_id: TypeId::of::<T>(),
            shader,
        }
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && Arc::ptr_eq(&self.shader, &other.shader)
    }
}

impl Eq for CacheKey {}

impl Hash for CacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        Arc::as_ptr(&self.shader).hash(state);
    }
}

pub struct LayoutAndFactory {
    pub bind_group_layouts: Option<Vec<BindGroupLayout>>,
    pub pipeline_layout: Option<PipelineLayout>,
    pub factory: Box<dyn RenderPipelineFactory>,
}

impl LayoutAndFactory {
    pub fn new(
        bind_group_layouts: Option<Vec<BindGroupLayout>>,
        pipeline_layout: Option<PipelineLayout>,
        factory: Box<dyn RenderPipelineFactory>,
    ) -> Self {
        Self {
            bind_group_layouts,
            pipeline_layout,
            factory,
        }
    }
}

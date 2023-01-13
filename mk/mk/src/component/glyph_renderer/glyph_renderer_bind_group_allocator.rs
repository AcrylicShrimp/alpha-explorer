use crate::{
    component::SpriteRenderPipelineFactoryProvider,
    gfx::{low::RenderPipelineAllocator, Texture},
    handles::*,
    GfxContext,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    sync::{Arc, Weak},
};
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindingResource};

pub struct GlyphRendererBindGroupAllocator {
    cache: HashMap<CacheKey, BindGroupHandle>,
}

impl GlyphRendererBindGroupAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(
        &mut self,
        gfx_context: &GfxContext,
        pipeline_allocator: &RenderPipelineAllocator,
        old_sprite: Option<SpriteHandle>,
        new_sprite: &SpriteHandle,
    ) -> BindGroupHandle {
        if let Some(old_sprite) = old_sprite {
            let old_texture = Arc::downgrade(&old_sprite.into_inner().texture());

            if old_texture.strong_count() == 0 {
                self.cache.remove(&CacheKey::from_weak(old_texture));
            }
        }

        match self
            .cache
            .entry(CacheKey::from_strong(&new_sprite.texture()))
        {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => entry
                .insert(BindGroupHandle::new(
                    gfx_context.device.create_bind_group(&BindGroupDescriptor {
                        label: None,
                        layout: &pipeline_allocator
                            .layout_and_factory::<SpriteRenderPipelineFactoryProvider>()
                            .bind_group_layouts
                            .as_ref()
                            .unwrap()[1],
                        entries: &[
                            BindGroupEntry {
                                binding: 0,
                                resource: BindingResource::TextureView(&new_sprite.texture().view),
                            },
                            BindGroupEntry {
                                binding: 1,
                                resource: BindingResource::Sampler(&new_sprite.texture().sampler),
                            },
                        ],
                    }),
                ))
                .clone(),
        }
    }
}

impl Default for SpriteRendererBindGroupAllocator {
    fn default() -> Self {
        Self {
            cache: HashMap::with_capacity(64),
        }
    }
}

struct CacheKey {
    pub texture: Weak<Texture>,
}

impl CacheKey {
    pub fn from_weak(texture: Weak<Texture>) -> Self {
        Self { texture }
    }

    pub fn from_strong(texture: &Arc<Texture>) -> Self {
        Self {
            texture: Arc::downgrade(texture),
        }
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.texture, &other.texture)
    }
}

impl Eq for CacheKey {}

impl Hash for CacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.texture.as_ptr().hash(state);
    }
}

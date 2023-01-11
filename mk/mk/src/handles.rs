macro_rules! define_handle {
    ($name:ident($type:ty)) => {
        #[derive(Clone)]
        pub struct $name(std::sync::Arc<$type>);

        impl $name {
            pub(crate) fn new(inner: $type) -> Self {
                Self(std::sync::Arc::new(inner))
            }

            pub(crate) fn wrap(inner: std::sync::Arc<$type>) -> Self {
                Self(inner)
            }

            pub fn inner(&self) -> &std::sync::Arc<$type> {
                &self.0
            }

            pub fn into_inner(self) -> std::sync::Arc<$type> {
                self.0
            }

            pub fn as_ptr(&self) -> *const $type {
                std::sync::Arc::as_ptr(&self.0)
            }
        }

        impl std::ops::Deref for $name {
            type Target = std::sync::Arc<$type>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                std::sync::Arc::ptr_eq(&self.0, &other.0)
            }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::sync::Arc::as_ptr(&self.0).hash(state);
            }
        }
    };
}

define_handle!(AudioClipHandle(crate::audio::AudioClip));

define_handle!(FontHandle(fontdue::Font));

define_handle!(BufferHandle(wgpu::Buffer));
define_handle!(PipelineHandle(wgpu::RenderPipeline));
define_handle!(ShaderHandle(wgpu::ShaderModule));

define_handle!(SpriteHandle(crate::gfx::Sprite));
define_handle!(TextureHandle(crate::gfx::Texture));

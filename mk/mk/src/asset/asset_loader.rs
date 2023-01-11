use crate::asset::{AssetLoadError, BaseAssetLoader};
use crate::EngineContext;
use std::any::{Any, TypeId};
use std::path::Path;
use std::sync::Arc;

pub struct AssetLoader<T>
where
    T: 'static + Any + Send + Sync,
{
    loader: Box<dyn Fn(&EngineContext, &Path, &Path) -> Result<T, AssetLoadError> + Sync>,
}

impl<T> AssetLoader<T>
where
    T: 'static + Any + Send + Sync,
{
    pub fn new<F>(loader: F) -> Self
    where
        F: 'static + Fn(&EngineContext, &Path, &Path) -> Result<T, AssetLoadError> + Sync,
    {
        Self {
            loader: Box::new(loader),
        }
    }

    pub fn load<P: AsRef<Path>>(
        &self,
        context: &EngineContext,
        base: P,
        path: P,
    ) -> Result<Arc<T>, AssetLoadError> {
        Ok(Arc::new(self.loader.as_ref()(
            context,
            base.as_ref(),
            path.as_ref(),
        )?))
    }
}

impl<T> BaseAssetLoader for AssetLoader<T>
where
    T: 'static + Any + Send + Sync,
{
    fn asset_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

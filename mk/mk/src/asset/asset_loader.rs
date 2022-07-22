use crate::asset::{AssetLoadError, AssetManager, BaseAssetLoader};
use std::any::{Any, TypeId};
use std::path::Path;
use std::sync::Arc;

pub struct AssetLoader<T>
where
    T: 'static + Any + Send + Sync,
{
    loader: Box<dyn Fn(&AssetManager, &Path, &Path) -> Result<Arc<T>, AssetLoadError> + Sync>,
}

impl<T> AssetLoader<T>
where
    T: 'static + Any + Send + Sync,
{
    pub fn new<F>(loader: F) -> Self
    where
        F: 'static + Fn(&AssetManager, &Path, &Path) -> Result<Arc<T>, AssetLoadError> + Sync,
    {
        Self {
            loader: Box::new(loader),
        }
    }

    pub fn load<P: AsRef<Path>>(
        &self,
        asset_mgr: &AssetManager,
        base: P,
        path: P,
    ) -> Result<Arc<T>, AssetLoadError> {
        self.loader.as_ref()(asset_mgr, base.as_ref(), path.as_ref())
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

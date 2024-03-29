use crate::asset::{
    AssetCacheManager, AssetLoadError, AssetLoader, BaseAssetCacheManager, BaseAssetLoader,
};
use crate::engine::use_context;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct AssetManager {
    base: PathBuf,
    types: HashMap<TypeId, (Box<dyn BaseAssetCacheManager>, Box<dyn BaseAssetLoader>)>,
}

impl AssetManager {
    pub fn new(base: PathBuf) -> AssetManager {
        AssetManager {
            base,
            types: HashMap::new(),
        }
    }

    pub fn register_loader<T>(&mut self, loader: AssetLoader<T>)
    where
        T: 'static + Clone + Any + Send + Sync,
    {
        self.types.insert(
            TypeId::of::<T>(),
            (Box::new(AssetCacheManager::<T>::new()), Box::new(loader)),
        );
    }

    // TODO: Provide async-way to load assets.
    pub fn load<T>(&self, path: impl AsRef<Path>) -> Result<T, AssetLoadError>
    where
        T: 'static + Clone + Any + Send + Sync,
    {
        match self.types.get(&TypeId::of::<T>()) {
            Some((cache, loader)) => {
                let cache = cache.downcast_ref::<AssetCacheManager<T>>().unwrap();

                match cache.load(&path) {
                    Some(asset) => Ok(asset.deref().clone()),
                    None => {
                        let asset = loader.downcast_ref::<AssetLoader<T>>().unwrap().load(
                            use_context(),
                            self.base.as_path(),
                            path.as_ref(),
                        )?;

                        cache.cache(path.as_ref().to_path_buf(), Arc::downgrade(&asset));
                        Ok(asset.deref().clone())
                    }
                }
            }
            None => Err(AssetLoadError::unsupported::<T>()),
        }
    }
}

use crate::asset::BaseAssetCacheManager;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct AssetCacheManager<T>
where
    T: 'static + Any + Sync + Send,
{
    caches: RefCell<HashMap<PathBuf, Weak<T>>>,
}

impl<T> AssetCacheManager<T>
where
    T: 'static + Any + Sync + Send,
{
    pub fn new() -> Self {
        Self {
            caches: HashMap::new().into(),
        }
    }

    pub fn load<P: AsRef<Path>>(&self, path: P) -> Option<Arc<T>> {
        let mut caches = self.caches.borrow_mut();
        match caches.get(path.as_ref()) {
            Some(cache) => match cache.upgrade() {
                None => {
                    caches.remove(path.as_ref());
                    None
                }
                cache => cache,
            },
            None => None,
        }
    }

    pub fn cache(&self, path: PathBuf, asset: Weak<T>) {
        let mut caches = self.caches.borrow_mut();
        caches.insert(path, asset);
    }
}

impl<T> BaseAssetCacheManager for AssetCacheManager<T>
where
    T: 'static + Any + Sync + Send,
{
    fn asset_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

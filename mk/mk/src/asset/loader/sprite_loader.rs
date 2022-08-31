use crate::{
    asset::{AssetLoadError, AssetLoader},
    render::{Sprite, SpriteError},
};
use std::sync::Arc;

impl From<SpriteError> for AssetLoadError {
    fn from(err: SpriteError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_loader() -> AssetLoader<Arc<Sprite>> {
    AssetLoader::new(|_asset_mgr, base, path| {
        Ok(Arc::new(Sprite::from_file(
            &base.join("sprites").join(path),
            None,
        )?))
    })
}

use crate::{
    asset::{AssetLoadError, AssetLoader},
    gfx::{SpriteAtlas, SpriteAtlasError},
};
use std::sync::Arc;

impl From<SpriteAtlasError> for AssetLoadError {
    fn from(err: SpriteAtlasError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_atlas_loader() -> AssetLoader<Arc<SpriteAtlas>> {
    AssetLoader::new(|_context, base, path| {
        Ok(Arc::new(SpriteAtlas::from_file(
            &base.join("sprites").join(path),
            None,
        )?))
    })
}

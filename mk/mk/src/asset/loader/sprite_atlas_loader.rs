use crate::asset::{AssetLoadError, AssetLoader};
use crate::render::{SpriteAtlas, SpriteAtlasError};

impl From<SpriteAtlasError> for AssetLoadError {
    fn from(err: SpriteAtlasError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_atlas_loader() -> AssetLoader<SpriteAtlas> {
    AssetLoader::new(|_asset_mgr, base, path| {
        Ok(SpriteAtlas::from_file(&base.join("sprites").join(path), None)?.into())
    })
}

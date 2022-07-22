use crate::asset::{AssetLoadError, AssetLoader};
use crate::render::{Sprite, SpriteError};

impl From<SpriteError> for AssetLoadError {
    fn from(err: SpriteError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_loader() -> AssetLoader<Sprite> {
    AssetLoader::new(|_asset_mgr, base, path| {
        Ok(Sprite::from_file(&base.join("sprites").join(path), None)?.into())
    })
}

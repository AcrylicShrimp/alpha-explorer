use crate::asset::{AssetLoadError, AssetLoader};
use crate::render::{SpriteNinePatch, SpriteNinePatchError};

impl From<SpriteNinePatchError> for AssetLoadError {
    fn from(err: SpriteNinePatchError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_nine_patch_loader() -> AssetLoader<SpriteNinePatch> {
    AssetLoader::new(|_asset_mgr, base, path| {
        Ok(SpriteNinePatch::from_file(&base.join("nine-patches").join(path), None)?.into())
    })
}

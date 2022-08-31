use crate::{
    asset::{AssetLoadError, AssetLoader},
    render::{SpriteNinePatch, SpriteNinePatchError},
};
use std::sync::Arc;

impl From<SpriteNinePatchError> for AssetLoadError {
    fn from(err: SpriteNinePatchError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_nine_patch_loader() -> AssetLoader<Arc<SpriteNinePatch>> {
    AssetLoader::new(|_asset_mgr, base, path| {
        Ok(Arc::new(SpriteNinePatch::from_file(
            &base.join("nine-patches").join(path),
            None,
        )?))
    })
}

use crate::{
    asset::{AssetLoadError, AssetLoader},
    gfx::{SpriteAtlasGrid, SpriteAtlasGridError},
};
use std::sync::Arc;

impl From<SpriteAtlasGridError> for AssetLoadError {
    fn from(err: SpriteAtlasGridError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_atlas_grid_loader() -> AssetLoader<Arc<SpriteAtlasGrid>> {
    AssetLoader::new(|_context, base, path| {
        Ok(Arc::new(SpriteAtlasGrid::from_file(
            &base.join("sprites").join(path),
            None,
        )?))
    })
}

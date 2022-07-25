use crate::asset::{AssetLoadError, AssetLoader};
use crate::render::{LuaRcSpriteAtlasGrid, SpriteAtlasGrid, SpriteAtlasGridError};

impl From<SpriteAtlasGridError> for AssetLoadError {
    fn from(err: SpriteAtlasGridError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_atlas_grid_loader() -> AssetLoader<LuaRcSpriteAtlasGrid> {
    AssetLoader::new(|_asset_mgr, base, path| {
        Ok(LuaRcSpriteAtlasGrid::wrap(SpriteAtlasGrid::from_file(
            &base.join("sprites").join(path),
            None,
        )?))
    })
}

use crate::asset::{AssetLoadError, AssetLoader};
use crate::render::{LuaRcSprite, Sprite, SpriteError};

impl From<SpriteError> for AssetLoadError {
    fn from(err: SpriteError) -> Self {
        Self::other(err)
    }
}

pub fn sprite_loader() -> AssetLoader<LuaRcSprite> {
    AssetLoader::new(|_asset_mgr, base, path| {
        Ok(LuaRcSprite::wrap(Sprite::from_file(
            &base.join("sprites").join(path),
            None,
        )?))
    })
}

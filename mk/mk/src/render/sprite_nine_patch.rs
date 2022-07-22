use crate::render::{LuaRcSprite, LuaRcTexture, Sprite, SpriteChannel, TexelMapping, Texture};
use codegen::LuaRc;
use image::{open as open_image, ColorType, GenericImageView, ImageError};
use serde::Deserialize;
use serde_json::{from_str, Error as JSONError};
use std::error::Error;
use std::fmt::Display;
use std::fs::{metadata as fs_metadata, read_to_string};
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub enum SpriteNinePatchError {
    IOError(IOError),
    JSONError(JSONError),
    ImageError(ImageError),
}

impl Display for SpriteNinePatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => err.fmt(f),
            Self::JSONError(err) => err.fmt(f),
            Self::ImageError(err) => err.fmt(f),
        }
    }
}

impl Error for SpriteNinePatchError {}

impl From<IOError> for SpriteNinePatchError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

impl From<JSONError> for SpriteNinePatchError {
    fn from(err: JSONError) -> Self {
        Self::JSONError(err)
    }
}

impl From<ImageError> for SpriteNinePatchError {
    fn from(err: ImageError) -> Self {
        Self::ImageError(err)
    }
}

#[derive(Deserialize)]
struct NinePatchMetadataJSON {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

#[derive(LuaRc, Debug)]
pub struct SpriteNinePatch {
    #[lua_userdata(LuaRcTexture)]
    texture: Arc<Texture>,
    #[lua_userdata(LuaRcSprite)]
    sprite_lt: Arc<Sprite>,
    #[lua_userdata(LuaRcSprite)]
    sprite_ct: Arc<Sprite>,
    #[lua_userdata(LuaRcSprite)]
    sprite_rt: Arc<Sprite>,
    #[lua_userdata(LuaRcSprite)]
    sprite_lm: Arc<Sprite>,
    #[lua_userdata(LuaRcSprite)]
    sprite_cm: Arc<Sprite>,
    #[lua_userdata(LuaRcSprite)]
    sprite_rm: Arc<Sprite>,
    #[lua_userdata(LuaRcSprite)]
    sprite_lb: Arc<Sprite>,
    #[lua_userdata(LuaRcSprite)]
    sprite_cb: Arc<Sprite>,
    #[lua_userdata(LuaRcSprite)]
    sprite_rb: Arc<Sprite>,
}

unsafe impl Send for SpriteNinePatch {}
unsafe impl Sync for SpriteNinePatch {}

impl SpriteNinePatch {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        channel: Option<SpriteChannel>,
    ) -> Result<Self, SpriteNinePatchError> {
        let mut image_path = Err(IOError::new(IOErrorKind::NotFound, "cannot find a image"));

        for ext in ["png", "jpg", "jpeg", "gif"] {
            let path = path.as_ref().with_extension(ext);
            match fs_metadata(&path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        image_path = Ok(path);
                        break;
                    }
                }
                Err(..) => continue,
            }
        }

        let image = open_image(image_path?)?;
        let (width, height) = image.dimensions();
        let texture = Arc::new(match channel {
            Some(channel) => match channel {
                SpriteChannel::R => {
                    Texture::from_slice_r_u8(width, height, image.to_luma8().as_raw())
                }
                SpriteChannel::RG => {
                    Texture::from_slice_rg_u8(width, height, image.to_luma_alpha8().as_raw())
                }
                SpriteChannel::RGB => {
                    Texture::from_slice_rgb_u8(width, height, image.to_rgb8().as_raw())
                }
                SpriteChannel::RGBA => {
                    Texture::from_slice_rgba_u8(width, height, image.to_rgba8().as_raw())
                }
            },
            None => match image.color() {
                ColorType::L8 | ColorType::L16 => {
                    Texture::from_slice_r_u8(width, height, image.to_luma8().as_raw())
                }
                ColorType::La8 | ColorType::La16 => {
                    Texture::from_slice_rg_u8(width, height, image.to_luma_alpha8().as_raw())
                }
                ColorType::Rgb8 | ColorType::Rgb16 => {
                    Texture::from_slice_rgb_u8(width, height, image.to_rgb8().as_raw())
                }
                ColorType::Rgba8 | ColorType::Rgba16 => {
                    Texture::from_slice_rgba_u8(width, height, image.to_rgba8().as_raw())
                }
                _ => unreachable!(),
            },
        });

        let metadata: NinePatchMetadataJSON =
            from_str(&read_to_string(path.as_ref().with_extension("meta.json"))?)?;

        let sprite_lt = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new((0, 0), (metadata.left, metadata.top)),
        )
        .into();
        let sprite_ct = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new((metadata.left, 0), (width - metadata.right, metadata.top)),
        )
        .into();
        let sprite_rt = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new((width - metadata.right, 0), (width, metadata.top)),
        )
        .into();
        let sprite_lm = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new((0, metadata.top), (metadata.left, height - metadata.bottom)),
        )
        .into();
        let sprite_cm = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new(
                (metadata.left, metadata.top),
                (width - metadata.right, height - metadata.bottom),
            ),
        )
        .into();
        let sprite_rm = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new(
                (width - metadata.right, metadata.top),
                (width, height - metadata.bottom),
            ),
        )
        .into();
        let sprite_lb = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new((0, height - metadata.bottom), (metadata.left, height)),
        )
        .into();
        let sprite_cb = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new(
                (metadata.left, height - metadata.bottom),
                (width - metadata.right, height),
            ),
        )
        .into();
        let sprite_rb = Sprite::from_atlas(
            texture.clone(),
            TexelMapping::new(
                (width - metadata.right, height - metadata.bottom),
                (width, height),
            ),
        )
        .into();

        Ok(Self {
            texture,
            sprite_lt,
            sprite_ct,
            sprite_rt,
            sprite_lm,
            sprite_cm,
            sprite_rm,
            sprite_lb,
            sprite_cb,
            sprite_rb,
        })
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }

    pub fn sprite_lt(&self) -> &Arc<Sprite> {
        &self.sprite_lt
    }

    pub fn sprite_ct(&self) -> &Arc<Sprite> {
        &self.sprite_ct
    }

    pub fn sprite_rt(&self) -> &Arc<Sprite> {
        &self.sprite_rt
    }

    pub fn sprite_lm(&self) -> &Arc<Sprite> {
        &self.sprite_lm
    }

    pub fn sprite_cm(&self) -> &Arc<Sprite> {
        &self.sprite_cm
    }

    pub fn sprite_rm(&self) -> &Arc<Sprite> {
        &self.sprite_rm
    }

    pub fn sprite_lb(&self) -> &Arc<Sprite> {
        &self.sprite_lb
    }

    pub fn sprite_cb(&self) -> &Arc<Sprite> {
        &self.sprite_cb
    }

    pub fn sprite_rb(&self) -> &Arc<Sprite> {
        &self.sprite_rb
    }
}

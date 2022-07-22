use crate::render::{LuaRcSprite, LuaRcTexture, Sprite, SpriteChannel, TexelMapping, Texture};
use codegen::LuaRc;
use image::{open as open_image, ColorType, GenericImageView, ImageError};
use mlua::prelude::*;
use serde::Deserialize;
use serde_json::{from_str, Error as JSONError};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::{metadata as fs_metadata, read_to_string};
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub enum SpriteAtlasError {
    IOError(IOError),
    JSONError(JSONError),
    ImageError(ImageError),
}

impl Display for SpriteAtlasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => err.fmt(f),
            Self::JSONError(err) => err.fmt(f),
            Self::ImageError(err) => err.fmt(f),
        }
    }
}

impl Error for SpriteAtlasError {}

impl From<IOError> for SpriteAtlasError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

impl From<JSONError> for SpriteAtlasError {
    fn from(err: JSONError) -> Self {
        Self::JSONError(err)
    }
}

impl From<ImageError> for SpriteAtlasError {
    fn from(err: ImageError) -> Self {
        Self::ImageError(err)
    }
}

#[derive(Deserialize)]
struct AtlasItemJSON {
    #[serde(rename = "min-x")]
    min_x: u32,
    #[serde(rename = "min-y")]
    min_y: u32,
    #[serde(rename = "max-x")]
    max_x: u32,
    #[serde(rename = "max-y")]
    max_y: u32,
}

type AtlasMetadataJSON = HashMap<String, AtlasItemJSON>;

#[derive(LuaRc, Debug)]
pub struct SpriteAtlas {
    #[lua_userdata(LuaRcTexture)]
    texture: Arc<Texture>,
    #[lua_userfunc(get=lua_get_sprites)]
    sprites: HashMap<String, Arc<Sprite>>,
}

unsafe impl Send for SpriteAtlas {}
unsafe impl Sync for SpriteAtlas {}

impl SpriteAtlas {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        channel: Option<SpriteChannel>,
    ) -> Result<Self, SpriteAtlasError> {
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

        let metadata: AtlasMetadataJSON =
            from_str(&read_to_string(path.as_ref().with_extension("meta.json"))?)?;

        Ok(Self {
            sprites: metadata
                .into_iter()
                .map(|(name, item)| {
                    (
                        name,
                        Sprite::from_atlas(
                            texture.clone(),
                            TexelMapping::new((item.min_x, item.min_y), (item.max_x, item.max_y)),
                        )
                        .into(),
                    )
                })
                .collect(),
            texture,
        })
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }

    pub fn sprites(&self) -> &HashMap<String, Arc<Sprite>> {
        &self.sprites
    }

    fn lua_get_sprites<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.sprites
            .iter()
            .map(|(k, v)| (k.clone(), LuaRcSprite::from(v.clone())))
            .collect::<HashMap<_, _>>()
            .to_lua(lua)
    }
}

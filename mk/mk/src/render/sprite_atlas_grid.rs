use crate::render::{LuaRcSprite, LuaRcTexture, Sprite, SpriteChannel, TexelMapping, Texture};
use codegen::LuaRc;
use image::{open as open_image, ColorType, GenericImageView, ImageError};
use mlua::prelude::*;
use serde::Deserialize;
use serde_json::{from_str, Error as JSONError};
use std::error::Error;
use std::fmt::Display;
use std::fs::{metadata as fs_metadata, read_to_string};
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub enum SpriteAtlasGridError {
    IOError(IOError),
    JSONError(JSONError),
    ImageError(ImageError),
}

impl Display for SpriteAtlasGridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => err.fmt(f),
            Self::JSONError(err) => err.fmt(f),
            Self::ImageError(err) => err.fmt(f),
        }
    }
}

impl Error for SpriteAtlasGridError {}

impl From<IOError> for SpriteAtlasGridError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

impl From<JSONError> for SpriteAtlasGridError {
    fn from(err: JSONError) -> Self {
        Self::JSONError(err)
    }
}

impl From<ImageError> for SpriteAtlasGridError {
    fn from(err: ImageError) -> Self {
        Self::ImageError(err)
    }
}

#[derive(Deserialize)]
struct AtlasGridMetadataJSON {
    #[serde(rename = "grid-width")]
    grid_width: u32,
    #[serde(rename = "grid-height")]
    grid_height: u32,
}

#[derive(LuaRc, Debug)]
pub struct SpriteAtlasGrid {
    #[lua_userdata(LuaRcTexture)]
    texture: Arc<Texture>,
    #[lua_userfunc(get=lua_get_sprites)]
    sprites: Vec<Arc<Sprite>>,
}

unsafe impl Send for SpriteAtlasGrid {}
unsafe impl Sync for SpriteAtlasGrid {}

impl SpriteAtlasGrid {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        channel: Option<SpriteChannel>,
    ) -> Result<Self, SpriteAtlasGridError> {
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

        let metadata: AtlasGridMetadataJSON =
            from_str(&read_to_string(path.as_ref().with_extension("meta.json"))?)?;

        let mut x = 0;
        let mut next_x = metadata.grid_width;
        let mut y = 0;
        let mut next_y = metadata.grid_height;
        let mut sprites = Vec::with_capacity(
            ((width / metadata.grid_width) * (height / metadata.grid_height)) as _,
        );

        while next_y <= height {
            while next_x <= width {
                sprites.push(
                    Sprite::from_atlas(
                        texture.clone(),
                        TexelMapping::new((x, y), (next_x, next_y)),
                    )
                    .into(),
                );

                x = next_x;
                next_x += metadata.grid_width;
            }

            x = 0;
            next_x = metadata.grid_width;
            y = next_y;
            next_y += metadata.grid_height;
        }

        Ok(Self { texture, sprites })
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }

    pub fn sprites(&self) -> &Vec<Arc<Sprite>> {
        &self.sprites
    }

    fn lua_get_sprites<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.sprites
            .iter()
            .map(|v| LuaRcSprite::from(v.clone()))
            .collect::<Vec<_>>()
            .to_lua(lua)
    }
}

use crate::render::{LuaRcTexture, Texture};
use codegen::LuaRc;
use image::{open as open_image, ColorType, GenericImageView, ImageError};
use mlua::prelude::*;
use std::error::Error;
use std::fmt::Display;
use std::fs::metadata as fs_metadata;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub enum SpriteError {
    IOError(IOError),
    ImageError(ImageError),
}

impl Display for SpriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => err.fmt(f),
            SpriteError::ImageError(err) => err.fmt(f),
        }
    }
}

impl Error for SpriteError {}

impl From<IOError> for SpriteError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

impl From<ImageError> for SpriteError {
    fn from(err: ImageError) -> Self {
        Self::ImageError(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteChannel {
    R,
    RG,
    RGB,
    RGBA,
}

impl<'lua> FromLua<'lua> for SpriteChannel {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(channel) => match channel.to_str()? {
                "r" => Ok(SpriteChannel::R),
                "rg" => Ok(SpriteChannel::RG),
                "rgb" => Ok(SpriteChannel::RGB),
                "rgba" => Ok(SpriteChannel::RGBA),
                _ => Err(format!(
                    "{:?} is invalid value for the type {}",
                    channel, "SpriteChannel",
                )
                .to_lua_err()),
            },
            _ => {
                return Err(
                    format!("the type {} must be a {}", "SpriteChannel", "string").to_lua_err(),
                );
            }
        }
    }
}

impl<'lua> ToLua<'lua> for SpriteChannel {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::String(lua.create_string(match self {
            SpriteChannel::R => "r",
            SpriteChannel::RG => "rg",
            SpriteChannel::RGB => "rgb",
            SpriteChannel::RGBA => "rgba",
        })?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TexelMapping {
    min: (u32, u32),
    max: (u32, u32),
}

impl TexelMapping {
    pub fn new(min: (u32, u32), max: (u32, u32)) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> (u32, u32) {
        self.min
    }

    pub fn max(&self) -> (u32, u32) {
        self.max
    }

    pub fn width(&self) -> u32 {
        self.max.0 - self.min.0
    }

    pub fn height(&self) -> u32 {
        self.max.1 - self.min.1
    }
}

impl<'lua> FromLua<'lua> for TexelMapping {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(mapping) => {
                let min = mapping.get::<_, LuaTable>("min")?;
                let max = mapping.get::<_, LuaTable>("max")?;
                Ok(TexelMapping::new(
                    (min.get::<_, u32>(1)?, min.get::<_, u32>(2)?),
                    (max.get::<_, u32>(1)?, max.get::<_, u32>(2)?),
                ))
            }
            _ => {
                return Err(
                    format!("the type {} must be a {}", "TexelMapping", "table").to_lua_err()
                );
            }
        }
    }
}

impl<'lua> ToLua<'lua> for TexelMapping {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Table(lua.create_table_from([
            ("min", lua.create_sequence_from([self.min.0, self.min.1])?),
            ("max", lua.create_sequence_from([self.max.0, self.max.1])?),
        ])?))
    }
}

#[derive(LuaRc, Debug)]
pub struct Sprite {
    channel: SpriteChannel,
    #[lua_userdata(LuaRcTexture)]
    texture: Arc<Texture>,
    texel_mapping: TexelMapping,
}

unsafe impl Send for Sprite {}
unsafe impl Sync for Sprite {}

impl Sprite {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        channel: Option<SpriteChannel>,
    ) -> Result<Self, SpriteError> {
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
        let (channel, texture) = match channel {
            Some(channel) => match channel {
                SpriteChannel::R => (
                    SpriteChannel::R,
                    Texture::from_slice_r_u8(width, height, image.to_luma8().as_raw()),
                ),
                SpriteChannel::RG => (
                    SpriteChannel::RG,
                    Texture::from_slice_rg_u8(width, height, image.to_luma_alpha8().as_raw()),
                ),
                SpriteChannel::RGB => (
                    SpriteChannel::RGB,
                    Texture::from_slice_rgb_u8(width, height, image.to_rgb8().as_raw()),
                ),
                SpriteChannel::RGBA => (
                    SpriteChannel::RGBA,
                    Texture::from_slice_rgba_u8(width, height, image.to_rgba8().as_raw()),
                ),
            },
            None => match image.color() {
                ColorType::L8 | ColorType::L16 => (
                    SpriteChannel::R,
                    Texture::from_slice_r_u8(width, height, image.to_luma8().as_raw()),
                ),
                ColorType::La8 | ColorType::La16 => (
                    SpriteChannel::RG,
                    Texture::from_slice_rg_u8(width, height, image.to_luma_alpha8().as_raw()),
                ),
                ColorType::Rgb8 | ColorType::Rgb16 => (
                    SpriteChannel::RGB,
                    Texture::from_slice_rgb_u8(width, height, image.to_rgb8().as_raw()),
                ),
                ColorType::Rgba8 | ColorType::Rgba16 => (
                    SpriteChannel::RGBA,
                    Texture::from_slice_rgba_u8(width, height, image.to_rgba8().as_raw()),
                ),
                _ => unreachable!(),
            },
        };

        Ok(Self {
            channel,
            texture: texture.into(),
            texel_mapping: TexelMapping::new((0, 0), (width, height)),
        })
    }

    pub fn from_atlas(texture: Arc<Texture>, texel_mapping: TexelMapping) -> Self {
        Self {
            channel: match texture.format().component() {
                1 => SpriteChannel::R,
                2 => SpriteChannel::RG,
                3 => SpriteChannel::RGB,
                4 => SpriteChannel::RGBA,
                _ => unreachable!(),
            },
            texture,
            texel_mapping,
        }
    }

    pub fn channel(&self) -> SpriteChannel {
        self.channel
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }

    pub fn texel_mapping(&self) -> &TexelMapping {
        &self.texel_mapping
    }

    pub fn width(&self) -> u32 {
        let lhs = self.texel_mapping.max().0;
        let rhs = self.texel_mapping.min().0;

        if lhs < rhs {
            rhs - lhs
        } else {
            lhs - rhs
        }
    }

    pub fn height(&self) -> u32 {
        let lhs = self.texel_mapping.max().1;
        let rhs = self.texel_mapping.min().1;

        if lhs < rhs {
            rhs - lhs
        } else {
            lhs - rhs
        }
    }
}

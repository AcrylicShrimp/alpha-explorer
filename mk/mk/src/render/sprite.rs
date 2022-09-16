use crate::render::Texture;
use image::{open as open_image, ColorType, GenericImageView, ImageError};
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

impl Display for SpriteChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SpriteChannel({})",
            match *self {
                SpriteChannel::R => "R",
                SpriteChannel::RG => "RG",
                SpriteChannel::RGB => "RGB",
                SpriteChannel::RGBA => "RGBA",
            }
        )
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

impl Display for TexelMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TexelMapping(min=({}, {}), max=({}, {}))",
            self.min.0, self.min.1, self.max.0, self.max.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    texture: Arc<Texture>,
    channel: SpriteChannel,
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
            texture: texture.into(),
            channel,
            texel_mapping: TexelMapping::new((0, 0), (width, height)),
        })
    }

    pub fn from_atlas(texture: impl Into<Arc<Texture>>, texel_mapping: TexelMapping) -> Self {
        let texture = texture.into();
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
        rhs.abs_diff(lhs)
    }

    pub fn height(&self) -> u32 {
        let lhs = self.texel_mapping.max().1;
        let rhs = self.texel_mapping.min().1;
        rhs.abs_diff(lhs)
    }
}

impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sprite({}x{})", self.width(), self.height())
    }
}

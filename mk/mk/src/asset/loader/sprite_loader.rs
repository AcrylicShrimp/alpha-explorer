use crate::{
    asset::{AssetLoadError, AssetLoader},
    gfx::{RenderManager, Sprite, SpriteTexelMapping},
    handles::*,
};
use anyhow::anyhow;
use image::{open as open_image, GenericImageView, ImageError};
use std::{
    fs::metadata as fs_metadata,
    io::{Error as IOError, ErrorKind as IOErrorKind},
    path::Path,
};
use wgpu::{SamplerDescriptor, TextureViewDescriptor};

impl From<ImageError> for AssetLoadError {
    fn from(err: ImageError) -> Self {
        AssetLoadError::other(err)
    }
}

pub fn sprite_loader() -> AssetLoader<SpriteHandle> {
    AssetLoader::new(|context, base, path| {
        Ok(from_file(
            &base.join("sprites").join(path),
            &context.render_mgr(),
        )?)
    })
}

fn from_file<P: AsRef<Path>>(
    path: P,
    render_mgr: &RenderManager,
) -> Result<SpriteHandle, AssetLoadError> {
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

    if (u16::MAX as u32) < width || (u16::MAX as u32) < height {
        return Err(AssetLoadError::other(anyhow!(
            "image {}x{} is too big",
            width,
            height
        )));
    }

    let texture =
        render_mgr.create_sprite_texture(width as u16, height as u16, image.to_rgba8().as_raw());
    let view = texture.texture.create_view(&TextureViewDescriptor {
        ..Default::default()
    });
    let sampler = render_mgr.create_sampler(&SamplerDescriptor {
        ..Default::default()
    });

    Ok(SpriteHandle::new(Sprite::new(
        texture,
        view,
        sampler,
        SpriteTexelMapping::new(0, width as u16, 0, height as u16),
        None,
    )))
}

use crate::{
    asset::{AssetLoadError, AssetLoader},
    handles::FontHandle,
};
use fontdue::{Font, FontSettings};
use std::{
    fs::{metadata as fs_metadata, read as fs_read},
    io::{Error as IOError, ErrorKind as IOErrorKind},
};

impl From<&'static str> for AssetLoadError {
    fn from(err: &'static str) -> Self {
        Self::other(err)
    }
}

pub fn font_loader() -> AssetLoader<FontHandle> {
    AssetLoader::new(|_context, base, path| {
        let path = base.join("fonts").join(path);
        let mut font_path = Err(IOError::new(IOErrorKind::NotFound, "cannot find a font"));

        for ext in ["ttf", "ttc", "otf"] {
            let path = path.with_extension(ext);
            match fs_metadata(&path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        font_path = Ok(path);
                        break;
                    }
                }
                Err(..) => continue,
            }
        }

        Ok(FontHandle::new(Font::from_bytes(
            fs_read(font_path?)?,
            FontSettings::default(),
        )?))
    })
}

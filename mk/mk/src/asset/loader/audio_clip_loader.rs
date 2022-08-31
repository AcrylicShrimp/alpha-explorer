use crate::asset::AssetLoader;
use crate::audio::AudioClip;
use std::{
    fs::{metadata as fs_metadata, OpenOptions},
    io::{BufReader, Error as IOError, ErrorKind as IOErrorKind},
    sync::Arc,
};

pub fn audio_clip_loader() -> AssetLoader<Arc<AudioClip>> {
    AssetLoader::new(|_asset_mgr, base, path| {
        let path = base.join("audios").join(path);
        let mut audio_clip_path = Err(IOError::new(
            IOErrorKind::NotFound,
            "cannot find a audio clip",
        ));

        for ext in ["wav", "mp3", "ogg", "flac", "aac"] {
            let path = path.with_extension(ext);
            match fs_metadata(&path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        audio_clip_path = Ok(path);
                        break;
                    }
                }
                Err(..) => continue,
            }
        }

        Ok(Arc::new(AudioClip::new(
            rodio::Decoder::new(BufReader::with_capacity(
                1024 * 32,
                OpenOptions::new().read(true).open(&audio_clip_path?)?,
            ))
            .unwrap(),
        )))
    })
}

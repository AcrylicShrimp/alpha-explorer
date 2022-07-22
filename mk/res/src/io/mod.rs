#[cfg(feature = "asset_loader")]
pub mod asset_loader;
#[cfg(feature = "asset_loader")]
pub mod decoder;

#[cfg(feature = "meta_loader")]
pub mod meta_loader;

#[cfg(feature = "writer")]
pub mod encoder;
#[cfg(feature = "writer")]
pub mod writer;

use std::io::Error as IOError;

#[cfg(any(feature = "asset_loader", feature = "writer"))]
pub(in crate) fn chunk_to_filename(chunk: crate::ResourceChunkID) -> String {
    format!("assets{}.res", chunk)
}

pub(in crate) fn read_file_all(
    file: &std::fs::File,
    offset: u64,
    buffer: &mut [u8],
) -> Result<(), IOError> {
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::prelude::FileExt;
        file.read_exact_at(buffer, offset)?;
    }
    #[cfg(target_family = "windows")]
    {
        use std::io::ErrorKind;
        use std::os::windows::prelude::FileExt;
        if file.seek_read(buffer, offset)? != buffer.len() {
            return Err(IOError::from(ErrorKind::UnexpectedEof));
        }
    }
    #[cfg(target_family = "wasm")]
    {
        use std::os::unix::prelude::FileExt;
        file.read_exact_at(buffer, offset)?;
    }

    Ok(())
}

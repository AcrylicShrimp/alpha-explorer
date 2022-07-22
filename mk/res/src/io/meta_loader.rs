use crate::ResourcesMeta;
use bincode::{options, Error as BincodeError, Options};

pub enum MetaLoadError {
    BincodeError(BincodeError),
    UnsupportedVersion,
}

impl From<BincodeError> for MetaLoadError {
    fn from(err: BincodeError) -> Self {
        Self::BincodeError(err)
    }
}

pub fn load_resource_meta(meta: &[u8]) -> Result<ResourcesMeta, MetaLoadError> {
    let meta: ResourcesMeta = options()
        .with_no_limit()
        .with_little_endian()
        .with_varint_encoding()
        .reject_trailing_bytes()
        .deserialize(meta)?;

    if meta.version != 1 {
        return Err(MetaLoadError::UnsupportedVersion);
    }

    Ok(meta)
}

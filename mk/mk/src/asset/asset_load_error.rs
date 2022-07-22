use std::any::type_name;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult};
use std::io::Error as IOError;

#[derive(Debug)]
pub enum AssetLoadError {
    UnsupportedAssetType(&'static str),
    IOError(IOError),
    Other(Box<dyn Error>),
}

impl AssetLoadError {
    pub fn unsupported<T: 'static>() -> Self {
        Self::UnsupportedAssetType(type_name::<T>())
    }

    pub fn other<T: 'static + Into<Box<dyn Error>>>(err: T) -> Self {
        Self::Other(err.into())
    }
}

impl From<IOError> for AssetLoadError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

impl Display for AssetLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        match self {
            AssetLoadError::UnsupportedAssetType(err) => {
                write!(f, "unsupported asset type: {}", err)
            }
            AssetLoadError::IOError(err) => write!(f, "io error: {}", err),
            AssetLoadError::Other(err) => write!(f, "unknown error: {}", err),
        }
    }
}

impl Error for AssetLoadError {}

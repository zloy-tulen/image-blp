use thiserror::Error;
use crate::types::BlpVersion;

#[derive(Debug, Error)]
pub enum Error {
    #[error("BLP supports width up to 65,535, the width: {0}")]
    WidthTooHigh(u32),
    #[error("BLP supports height up to 65,535, the width: {0}")]
    HeightTooHigh(u32),
    #[error("External mipmaps are not supported for the version {0}")]
    ExternalMipmapsNotSupported(BlpVersion),
    #[error("Invalid offset {1} for mipmap {0}, filled bytes {2}")]
    InvalidOffset(u32, u32, u32),
    #[error("Size of mipmap {0} in header {1} doesn't match actual {2}")]
    InvalidMipmapSize(u32, u32, u32),
    #[error("Failed to proceed {0}, due: {1}")]
    FileSystem(std::path::PathBuf, std::io::Error),
    #[error("Name of root file is malformed: {0}")]
    FileNameInvalid(std::path::PathBuf),
}
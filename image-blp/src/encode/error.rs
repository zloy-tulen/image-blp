use crate::types::BlpVersion;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("BLP supports width up to 65,535, the width: {0}")]
    WidthTooHigh(u32),
    #[error("BLP supports height up to 65,535, the width: {0}")]
    HeightTooHigh(u32),
    #[error("External mipmaps are not supported for the version {0}")]
    ExternalMipmapsNotSupported(BlpVersion),
    #[error("Invalid offset {offset} for mipmap {mipmap}, filled bytes {filled}")]
    InvalidOffset {
        mipmap: usize,
        offset: usize,
        filled: usize,
    },
    #[error("Size of mipmap {mipmap} in header {in_header} doesn't match actual {actual}")]
    InvalidMipmapSize {
        mipmap: usize,
        in_header: usize,
        actual: usize,
    },
    #[error("Failed to proceed {0}, due: {1}")]
    FileSystem(std::path::PathBuf, std::io::Error),
    #[error("Name of root file is malformed: {0}")]
    FileNameInvalid(std::path::PathBuf),
}

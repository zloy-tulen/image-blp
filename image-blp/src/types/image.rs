pub use super::version::BlpVersion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpHeader {
    pub version: BlpVersion,
}

/// Parsed information from BLP file. The structure of the type 
/// strictly follows how the file is stored on the disk for 
/// easy encoding/decoding and further transformations. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpImage {
    pub header: BlpHeader,
}
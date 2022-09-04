use super::direct::*;
use super::header::*;
use super::jpeg::*;
pub use super::version::BlpVersion;

/// Parsed information from BLP file. The structure of the type
/// strictly follows how the file is stored on the disk for
/// easy encoding/decoding and further transformations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpImage {
    pub header: BlpHeader,
    pub content: BlpContent,
}

impl BlpImage {
    /// If the image is encoded jpeg, return the content
    pub fn get_content_jpeg(&self) -> Option<&BlpJpeg> {
        self.content.get_jpeg()
    }

    /// If the image is direct encoded, return the content
    pub fn get_content_direct(&self) -> Option<&BlpDirect> {
        self.content.get_direct()
    }
}

/// Collects all possible content types with actual data
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlpContent {
    Jpeg(BlpJpeg),
    Direct(BlpDirect),
}

impl BlpContent {
    pub fn tag(&self) -> BlpContentTag {
        match self {
            BlpContent::Jpeg { .. } => BlpContentTag::Jpeg,
            BlpContent::Direct { .. } => BlpContentTag::Direct,
        }
    }

    pub fn get_jpeg(&self) -> Option<&BlpJpeg> {
        match self {
            BlpContent::Jpeg(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_direct(&self) -> Option<&BlpDirect> {
        match self {
            BlpContent::Direct(v) => Some(v),
            _ => None,
        }
    }
}

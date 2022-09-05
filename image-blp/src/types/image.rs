use super::direct::*;
use super::dxtn::*;
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

    /// If the image is DXT1 encoded, return the content
    pub fn get_content_dxt1(&self) -> Option<&BlpDxt1> {
        self.content.get_dxt1()
    }

    /// If the image is DXT3 encoded, return the content
    pub fn get_content_dxt3(&self) -> Option<&BlpDxt3> {
        self.content.get_dxt3()
    }

    /// If the image is DXT5 encoded, return the content
    pub fn get_content_dxt5(&self) -> Option<&BlpDxt5> {
        self.content.get_dxt5()
    }
}

/// Collects all possible content types with actual data
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlpContent {
    Jpeg(BlpJpeg),
    /// Used with direct type for BLP0/BLP1 and raw compression in BLP2
    Direct(BlpDirect),
    /// BLP2 DXT1 compression (no alpha)
    Dxt1(BlpDxt1),
    /// BLP2 DXT3 compression (with alpha)
    Dxt3(BlpDxt3),
    /// BLP2 DXT5 compression (with alpha)
    Dxt5(BlpDxt5),
}

impl BlpContent {
    pub fn tag(&self) -> BlpContentTag {
        match self {
            BlpContent::Jpeg { .. } => BlpContentTag::Jpeg,
            BlpContent::Direct { .. } => BlpContentTag::Direct,
            BlpContent::Dxt1 { .. } => BlpContentTag::Direct,
            BlpContent::Dxt3 { .. } => BlpContentTag::Direct,
            BlpContent::Dxt5 { .. } => BlpContentTag::Direct,
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

    pub fn get_dxt1(&self) -> Option<&BlpDxt1> {
        match self {
            BlpContent::Dxt1(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_dxt3(&self) -> Option<&BlpDxt3> {
        match self {
            BlpContent::Dxt3(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_dxt5(&self) -> Option<&BlpDxt5> {
        match self {
            BlpContent::Dxt5(v) => Some(v),
            _ => None,
        }
    }
}

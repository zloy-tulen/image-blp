use super::direct::*;
use super::header::*;
use super::jpeg::*;
pub use super::version::BlpVersion;

/// Maximum width that BLP image can have due limitation
/// of mipmaping storage.
pub const BLP_MAX_WIDTH: u32 = 65535;
/// Maximum height that BLP image can have due limitation
/// of mipmaping storage.
pub const BLP_MAX_HEIGHT: u32 = 65535;

/// Parsed information from BLP file. The structure of the type
/// strictly follows how the file is stored on the disk for
/// easy encoding/decoding and further transformations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlpImage {
    pub header: BlpHeader,
    pub content: BlpContent,
}

impl BlpImage {
    /// Get total amount of images encoded in the content
    pub fn image_count(&self) -> usize {
        match &self.content {
            BlpContent::Dxt1(v) => v.images.len(),
            BlpContent::Dxt3(v) => v.images.len(),
            BlpContent::Dxt5(v) => v.images.len(),
            BlpContent::Raw1(v) => v.images.len(),
            BlpContent::Raw3(v) => v.images.len(),
            BlpContent::Jpeg(v) => v.images.len(),
        }
    }

    /// If the image is encoded jpeg, return the content
    pub fn content_jpeg(&self) -> Option<&BlpJpeg> {
        self.content.jpeg()
    }

    /// If the image is direct encoded with BLP1 format, return the content
    pub fn content_raw1(&self) -> Option<&BlpRaw1> {
        self.content.raw1()
    }

    /// If the image is direct encoded with raw3 BLP2 format, return the content
    pub fn content_raw3(&self) -> Option<&BlpRaw3> {
        self.content.raw3()
    }

    /// If the image is DXT1 encoded, return the content
    pub fn content_dxt1(&self) -> Option<&BlpDxtn> {
        self.content.dxt1()
    }

    /// If the image is DXT3 encoded, return the content
    pub fn ontent_dxt3(&self) -> Option<&BlpDxtn> {
        self.content.dxt3()
    }

    /// If the image is DXT5 encoded, return the content
    pub fn content_dxt5(&self) -> Option<&BlpDxtn> {
        self.content.dxt5()
    }
}

/// Collects all possible content types with actual data
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlpContent {
    Jpeg(BlpJpeg),
    /// Used with direct type for BLP0/BLP1 and raw compression in BLP2
    Raw1(BlpRaw1),
    /// Used with direct type for BLP2, encodes RGBA bitmap.
    Raw3(BlpRaw3),
    /// BLP2 DXT1 compression (no alpha)
    Dxt1(BlpDxtn),
    /// BLP2 DXT3 compression (with alpha)
    Dxt3(BlpDxtn),
    /// BLP2 DXT5 compression (with alpha)
    Dxt5(BlpDxtn),
}

impl BlpContent {
    pub fn tag(&self) -> BlpContentTag {
        match self {
            BlpContent::Jpeg { .. } => BlpContentTag::Jpeg,
            BlpContent::Raw1 { .. } => BlpContentTag::Direct,
            BlpContent::Raw3 { .. } => BlpContentTag::Direct,
            BlpContent::Dxt1 { .. } => BlpContentTag::Direct,
            BlpContent::Dxt3 { .. } => BlpContentTag::Direct,
            BlpContent::Dxt5 { .. } => BlpContentTag::Direct,
        }
    }

    pub fn jpeg(&self) -> Option<&BlpJpeg> {
        match self {
            BlpContent::Jpeg(v) => Some(v),
            _ => None,
        }
    }

    pub fn raw1(&self) -> Option<&BlpRaw1> {
        match self {
            BlpContent::Raw1(v) => Some(v),
            _ => None,
        }
    }

    pub fn raw3(&self) -> Option<&BlpRaw3> {
        match self {
            BlpContent::Raw3(v) => Some(v),
            _ => None,
        }
    }

    pub fn dxt1(&self) -> Option<&BlpDxtn> {
        match self {
            BlpContent::Dxt1(v) => Some(v),
            _ => None,
        }
    }

    pub fn dxt3(&self) -> Option<&BlpDxtn> {
        match self {
            BlpContent::Dxt3(v) => Some(v),
            _ => None,
        }
    }

    pub fn dxt5(&self) -> Option<&BlpDxtn> {
        match self {
            BlpContent::Dxt5(v) => Some(v),
            _ => None,
        }
    }
}

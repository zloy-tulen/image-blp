pub use super::version::BlpVersion;
use std::fmt;

/// The content field determines how the image data is stored. CONTENT_JPEG
/// uses non-standard JPEG (JFIF) file compression of BGRA colour component
/// values rather than the usual Y′CbCr color component values.
/// CONTENT_DIRECT refers to a variety of storage formats which can be
/// directly read as pixel values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlpContent {
    Jpeg,
    Direct,
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownContent(u32);

impl fmt::Display for UnknownContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown content field value: {}", self.0)
    }
}

impl TryFrom<u32> for BlpContent {
    type Error = UnknownContent;

    fn try_from(val: u32) -> Result<BlpContent, Self::Error> {
        match val {
            0 => Ok(BlpContent::Jpeg),
            1 => Ok(BlpContent::Direct),
            _ => Err(UnknownContent(val)),
        }
    }
}

impl From<BlpContent> for u32 {
    fn from(val: BlpContent) -> u32 {
        match val {
            BlpContent::Jpeg => 0,
            BlpContent::Direct => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlpHeader {
    pub version: BlpVersion,
    pub content: BlpContent,
}

/// Parsed information from BLP file. The structure of the type
/// strictly follows how the file is stored on the disk for
/// easy encoding/decoding and further transformations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpImage {
    pub header: BlpHeader,
}

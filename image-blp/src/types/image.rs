pub use super::version::BlpVersion;
use std::fmt;

/// The content field determines how the image data is stored. CONTENT_JPEG
/// uses non-standard JPEG (JFIF) file compression of BGRA colour component
/// values rather than the usual Y′CbCr color component values.
/// CONTENT_DIRECT refers to a variety of storage formats which can be
/// directly read as pixel values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlpContentTag {
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

impl TryFrom<u32> for BlpContentTag {
    type Error = UnknownContent;

    fn try_from(val: u32) -> Result<BlpContentTag, Self::Error> {
        match val {
            0 => Ok(BlpContentTag::Jpeg),
            1 => Ok(BlpContentTag::Direct),
            _ => Err(UnknownContent(val)),
        }
    }
}

impl From<BlpContentTag> for u32 {
    fn from(val: BlpContentTag) -> u32 {
        match val {
            BlpContentTag::Jpeg => 0,
            BlpContentTag::Direct => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlpHeader {
    pub version: BlpVersion,
    pub content: BlpContentTag,
    pub flags: BlpFlags,
    pub width: u32,
    pub height: u32,
}

impl BlpHeader {
    /// Calculate needed count of mipmaps for the defined size
    pub fn mipmaps_count(&self) -> u32 {
        let width_n = (self.width as f32).log2() as u32;
        let height_n = (self.height as f32).log2() as u32;
        width_n.max(height_n)
    }

    /// Returns 'true' if the header defines that the image has mipmaps
    pub fn has_mipmaps(&self) -> bool {
        self.flags.has_mipmaps()
    }
}

impl Default for BlpHeader {
    fn default() -> Self {
        BlpHeader {
            version: BlpVersion::Blp1,
            content: BlpContentTag::Jpeg,
            flags: Default::default(),
            width: 1,
            height: 1,
        }
    }
}

/// Part of header that depends on the version
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlpFlags {
    /// For version >= 2
    Blp2 {
        encoding_type: u8, // not documented
        alpha_bits: u8,
        sample_type: u8, // not documented
        has_mipmaps: u8,
    },
    /// For version < 2
    Old {
        alpha_bits: u32,
        extra: u32,       // no purpose, default is 5
        has_mipmaps: u32, // boolean
    },
}

impl Default for BlpFlags {
    fn default() -> Self {
        BlpFlags::Old {
            alpha_bits: 8,
            extra: 8,
            has_mipmaps: 1,
        }
    }
}

impl BlpFlags {
    /// Returns 'true' if the header defines that the image has mipmaps
    pub fn has_mipmaps(&self) -> bool {
        match self {
            BlpFlags::Blp2 { has_mipmaps, .. } => *has_mipmaps != 0,
            BlpFlags::Old { has_mipmaps, .. } => *has_mipmaps != 0,
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpJpeg {
    /// JPEG header that is appended to each mipmap level data
    pub header: Vec<u8>,
    /// Image itself and all mipmaps levels. If there are no mipmaps,
    /// the length of the vector is 1.
    pub images: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpDirect {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mipmap_count() {
        let header = BlpHeader {
            width: 512,
            height: 256,
            ..Default::default()
        };
        assert_eq!(header.mipmaps_count(), 9);

        let header = BlpHeader {
            width: 1,
            height: 4,
            ..Default::default()
        };
        assert_eq!(header.mipmaps_count(), 2);

        let header = BlpHeader {
            width: 1,
            height: 7,
            ..Default::default()
        };
        assert_eq!(header.mipmaps_count(), 2);
    }
}

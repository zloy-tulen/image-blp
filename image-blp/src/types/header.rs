pub use super::locator::MipmapLocator;
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
    pub mipmap_locator: MipmapLocator,
}

impl BlpHeader {
    /// Calculate needed count of mipmaps for the defined size
    pub fn mipmaps_count(&self) -> u32 {
        if self.has_mipmaps() {
            let width_n = (self.width as f32).log2() as u32;
            let height_n = (self.height as f32).log2() as u32;
            width_n.max(height_n)
        } else {
            0
        }
    }

    /// Returns 'true' if the header defines that the image has mipmaps
    pub fn has_mipmaps(&self) -> bool {
        self.flags.has_mipmaps()
    }

    /// Return expected size of mipmap for the given mipmap level.
    /// 0 level means original image.
    pub fn mipmap_size(&self, i: u32) -> (u32, u32) {
        if i == 0 {
            (self.width, self.height)
        } else {
            ((self.width >> i).max(1), (self.height >> i).max(1))
        }
    }

    /// Return expected count of pixels in mipmap at the level i.
    /// 0 level means original image.
    pub fn mipmap_pixels(&self, i: u32) -> u32 {
        let (w, h) = self.mipmap_size(i);
        w * h
    }

    /// Return alpha bits count in encoding
    pub fn alpha_bits(&self) -> u32 {
        self.flags.alpha_bits()
    }

    /// Return offsets and sizes of internal mipmaps. For external returns [None]
    pub fn internal_mipmaps(&self) -> Option<([u32; 16], [u32; 16])> {
        match self.mipmap_locator {
            MipmapLocator::Internal { offsets, sizes } => Some((offsets, sizes)),
            MipmapLocator::External => None,
        }
    }

    /// Get size of header in bytes. Doesn't count jpeg header or color map.
    pub fn size(version: BlpVersion) -> usize {
        4 // magic
        + 4 // content
        + 4 // flags or alpha_bits
        + 4 // width 
        + 4 // height
        + if version < BlpVersion::Blp2 {8} else {0} // extra and has_mipmaps
        + if version > BlpVersion::Blp0 {16*4*2} else {0} // mipmap locator
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
            mipmap_locator: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Compression {
    Raw1,
    Raw3,
    Dxtc,
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownCompression(u8);

impl fmt::Display for UnknownCompression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown compression field value: {}", self.0)
    }
}

impl TryFrom<u8> for Compression {
    type Error = UnknownCompression;

    fn try_from(val: u8) -> Result<Compression, Self::Error> {
        match val {
            1 => Ok(Compression::Raw1),
            2 => Ok(Compression::Dxtc),
            3 => Ok(Compression::Raw3),
            _ => Err(UnknownCompression(val)),
        }
    }
}

impl From<Compression> for u8 {
    fn from(val: Compression) -> u8 {
        match val {
            Compression::Raw1 => 1,
            Compression::Dxtc => 2,
            Compression::Raw3 => 3,
        }
    }
}

/// Part of header that depends on the version
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlpFlags {
    /// For version >= 2
    Blp2 {
        compression: Compression,
        alpha_bits: u8, // 0, 1, 7, or 8
        alpha_type: u8, // 0, 1, 7, or 8
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

    /// Get count of bits alpha channel is encoded in content
    pub fn alpha_bits(&self) -> u32 {
        match self {
            BlpFlags::Blp2 { compression, .. } if *compression == Compression::Raw3 => 4,
            BlpFlags::Blp2 { alpha_bits, .. } => *alpha_bits as u32,
            BlpFlags::Old { alpha_bits, .. } => *alpha_bits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mipmap_count() {
        let header = BlpHeader {
            width: 512,
            height: 256,
            version: BlpVersion::Blp0,
            ..Default::default()
        };
        assert_eq!(header.mipmaps_count(), 9);

        let header = BlpHeader {
            width: 512,
            height: 256,
            version: BlpVersion::Blp1,
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
            width: 4,
            height: 7,
            ..Default::default()
        };
        assert_eq!(header.mipmaps_count(), 2);

        let header = BlpHeader {
            width: 768,
            height: 128,
            ..Default::default()
        };
        assert_eq!(header.mipmaps_count(), 9);
    }
}

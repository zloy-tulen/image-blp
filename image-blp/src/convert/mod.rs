mod dxtn;
pub mod error;
mod jpeg;
mod mipmap;
mod palette;
mod raw1;
mod raw3;

use crate::types::*;
use ::image::{imageops::FilterType, DynamicImage};
use dxtn::*;
pub use error::Error;
use jpeg::*;
use raw1::*;
use raw3::*;
pub use texpresso::Algorithm as DxtAlgorithm;

/// Convert from parsed raw BLP image to useful [DynamicImage]
pub fn blp_to_image(image: &BlpImage, mipmap_level: usize) -> Result<DynamicImage, Error> {
    match &image.content {
        BlpContent::Raw1(content) => raw1_to_image(&image.header, content, mipmap_level),
        BlpContent::Raw3(content) => raw3_to_image(&image.header, content, mipmap_level),
        BlpContent::Jpeg(content) => jpeg_to_image(content, mipmap_level),
        BlpContent::Dxt1(content) => dxtn_to_image(&image.header, content, mipmap_level),
        BlpContent::Dxt3(content) => dxtn_to_image(&image.header, content, mipmap_level),
        BlpContent::Dxt5(content) => dxtn_to_image(&image.header, content, mipmap_level),
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum BlpTarget {
    Blp0(BlpOldFormat),
    Blp1(BlpOldFormat),
    Blp2(Blp2Format),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlpOldFormat {
    Raw1 { alpha_bits: AlphaBits },
    Jpeg { has_alpha: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaBits {
    NoAlpha,
    Bit1,
    Bit4,
    Bit8,
}

impl From<AlphaBits> for u32 {
    fn from(value: AlphaBits) -> u32 {
        match value {
            AlphaBits::NoAlpha => 0,
            AlphaBits::Bit1 => 1,
            AlphaBits::Bit4 => 4,
            AlphaBits::Bit8 => 8,
        }
    }
}

impl From<AlphaBits> for u8 {
    fn from(value: AlphaBits) -> u8 {
        match value {
            AlphaBits::NoAlpha => 0,
            AlphaBits::Bit1 => 1,
            AlphaBits::Bit4 => 4,
            AlphaBits::Bit8 => 8,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Blp2Format {
    Raw1 {
        alpha_bits: AlphaBits,
    },
    Raw3,
    Jpeg {
        has_alpha: bool,
    },
    Dxt1 {
        has_alpha: bool,
        compress_algorithm: DxtAlgorithm,
    },
    Dxt3 {
        has_alpha: bool,
        compress_algorithm: DxtAlgorithm,
    },
    Dxt5 {
        has_alpha: bool,
        compress_algorithm: DxtAlgorithm,
    },
}

/// Convert from unpacked pixels into BLP image ready for writing down
pub fn image_to_blp(
    image: DynamicImage,
    make_mipmaps: bool,
    target: BlpTarget,
    mipmap_filter: FilterType,
) -> Result<BlpImage, Error> {
    if image.width() > BLP_MAX_WIDTH {
        return Err(Error::WidthTooLarge(image.width()));
    }
    if image.height() > BLP_MAX_HEIGHT {
        return Err(Error::HeightTooLarge(image.height()));
    }

    match target {
        BlpTarget::Blp0(format) => match format {
            BlpOldFormat::Raw1 { alpha_bits } => {
                let header = BlpHeader {
                    version: BlpVersion::Blp0,
                    content: BlpContentTag::Direct,
                    flags: BlpFlags::Old {
                        alpha_bits: alpha_bits.into(),
                        extra: 4,
                        has_mipmaps: if make_mipmaps { 1 } else { 0 },
                    },
                    width: image.width(),
                    height: image.height(),
                    mipmap_locator: MipmapLocator::External,
                };
                let blp_raw1 =
                    image_to_raw1(image, alpha_bits.into(), make_mipmaps, mipmap_filter)?;
                Ok(BlpImage {
                    header,
                    content: BlpContent::Raw1(blp_raw1),
                })
            }
            BlpOldFormat::Jpeg { has_alpha } => {
                let alpha_bits = if has_alpha { 8 } else { 0 };
                let blp_jpeg = image_to_jpeg(&image, make_mipmaps, alpha_bits, mipmap_filter)?;
                Ok(BlpImage {
                    header: BlpHeader {
                        version: BlpVersion::Blp0,
                        content: BlpContentTag::Jpeg,
                        flags: BlpFlags::Old {
                            alpha_bits: alpha_bits as u32,
                            extra: 5,
                            has_mipmaps: if make_mipmaps { 1 } else { 0 },
                        },
                        width: image.width(),
                        height: image.height(),
                        mipmap_locator: MipmapLocator::External,
                    },
                    content: BlpContent::Jpeg(blp_jpeg),
                })
            }
        },
        BlpTarget::Blp1(format) => match format {
            BlpOldFormat::Raw1 { alpha_bits } => {
                let width = image.width();
                let height = image.height();
                let blp_raw1 =
                    image_to_raw1(image, alpha_bits.into(), make_mipmaps, mipmap_filter)?;
                let header = BlpHeader {
                    version: BlpVersion::Blp1,
                    content: BlpContentTag::Direct,
                    flags: BlpFlags::Old {
                        alpha_bits: alpha_bits.into(),
                        extra: 4,
                        has_mipmaps: if make_mipmaps { 1 } else { 0 },
                    },
                    width,
                    height,
                    mipmap_locator: blp_raw1.mipmap_locator(BlpVersion::Blp1),
                };
                Ok(BlpImage {
                    header,
                    content: BlpContent::Raw1(blp_raw1),
                })
            }
            BlpOldFormat::Jpeg { has_alpha } => {
                let alpha_bits = if has_alpha { 8 } else { 0 };
                let blp_jpeg = image_to_jpeg(&image, make_mipmaps, alpha_bits, mipmap_filter)?;
                Ok(BlpImage {
                    header: BlpHeader {
                        version: BlpVersion::Blp1,
                        content: BlpContentTag::Jpeg,
                        flags: BlpFlags::Old {
                            alpha_bits: alpha_bits as u32,
                            extra: 5,
                            has_mipmaps: if make_mipmaps { 1 } else { 0 },
                        },
                        width: image.width(),
                        height: image.height(),
                        mipmap_locator: blp_jpeg.mipmap_locator(BlpVersion::Blp1),
                    },
                    content: BlpContent::Jpeg(blp_jpeg),
                })
            }
        },
        BlpTarget::Blp2(format) => match format {
            Blp2Format::Raw1 { alpha_bits } => {
                let width = image.width();
                let height = image.height();
                let blp_raw1 =
                    image_to_raw1(image, alpha_bits.into(), make_mipmaps, mipmap_filter)?;
                let header = BlpHeader {
                    version: BlpVersion::Blp2,
                    content: BlpContentTag::Direct,
                    flags: BlpFlags::Blp2 {
                        compression: Compression::Raw1,
                        alpha_bits: alpha_bits.into(),
                        alpha_type: 0,
                        has_mipmaps: if make_mipmaps { 1 } else { 0 },
                    },
                    width,
                    height,
                    mipmap_locator: blp_raw1.mipmap_locator(BlpVersion::Blp2),
                };
                Ok(BlpImage {
                    header,
                    content: BlpContent::Raw1(blp_raw1),
                })
            }
            Blp2Format::Raw3 => {
                let width = image.width();
                let height = image.height();
                let blp_raw3 = image_to_raw3(image, make_mipmaps, mipmap_filter)?;
                Ok(BlpImage {
                    header: BlpHeader {
                        version: BlpVersion::Blp2,
                        content: BlpContentTag::Direct,
                        flags: BlpFlags::Blp2 {
                            compression: Compression::Raw3,
                            alpha_bits: 8,
                            alpha_type: 0,
                            has_mipmaps: if make_mipmaps { 1 } else { 0 },
                        },
                        width,
                        height,
                        mipmap_locator: blp_raw3.mipmap_locator(BlpVersion::Blp2),
                    },
                    content: BlpContent::Raw3(blp_raw3),
                })
            }
            Blp2Format::Jpeg { has_alpha } => {
                let alpha_bits = if has_alpha { 8 } else { 0 };
                let blp_jpeg = image_to_jpeg(&image, make_mipmaps, alpha_bits, mipmap_filter)?;
                Ok(BlpImage {
                    header: BlpHeader {
                        version: BlpVersion::Blp2,
                        content: BlpContentTag::Jpeg,
                        flags: BlpFlags::Blp2 {
                            compression: Compression::Jpeg,
                            alpha_bits: 8,
                            alpha_type: 0,
                            has_mipmaps: if make_mipmaps { 1 } else { 0 },
                        },
                        width: image.width(),
                        height: image.height(),
                        mipmap_locator: blp_jpeg.mipmap_locator(BlpVersion::Blp2),
                    },
                    content: BlpContent::Jpeg(blp_jpeg),
                })
            }
            Blp2Format::Dxt1 {
                has_alpha,
                compress_algorithm,
            } => {
                let width = image.width();
                let height = image.height();
                let alpha_bits = if has_alpha { 1 } else { 0 };
                let blp_dxtn = image_to_dxtn(
                    image,
                    DxtnFormat::Dxt1,
                    make_mipmaps,
                    mipmap_filter,
                    compress_algorithm,
                )?;
                Ok(BlpImage {
                    header: BlpHeader {
                        version: BlpVersion::Blp2,
                        content: BlpContentTag::Direct,
                        flags: BlpFlags::Blp2 {
                            compression: Compression::Dxtc,
                            alpha_bits,
                            alpha_type: 0,
                            has_mipmaps: if make_mipmaps { 1 } else { 0 },
                        },
                        width,
                        height,
                        mipmap_locator: blp_dxtn.mipmap_locator(BlpVersion::Blp2),
                    },
                    content: BlpContent::Dxt1(blp_dxtn),
                })
            }
            Blp2Format::Dxt3 {
                has_alpha,
                compress_algorithm,
            } => {
                let width = image.width();
                let height = image.height();
                let alpha_bits = if has_alpha { 8 } else { 0 };
                let blp_dxtn = image_to_dxtn(
                    image,
                    DxtnFormat::Dxt3,
                    make_mipmaps,
                    mipmap_filter,
                    compress_algorithm,
                )?;
                Ok(BlpImage {
                    header: BlpHeader {
                        version: BlpVersion::Blp2,
                        content: BlpContentTag::Direct,
                        flags: BlpFlags::Blp2 {
                            compression: Compression::Dxtc,
                            alpha_bits,
                            alpha_type: 1,
                            has_mipmaps: if make_mipmaps { 1 } else { 0 },
                        },
                        width,
                        height,
                        mipmap_locator: blp_dxtn.mipmap_locator(BlpVersion::Blp2),
                    },
                    content: BlpContent::Dxt3(blp_dxtn),
                })
            }
            Blp2Format::Dxt5 {
                has_alpha,
                compress_algorithm,
            } => {
                let width = image.width();
                let height = image.height();
                let alpha_bits = if has_alpha { 8 } else { 0 };
                let blp_dxtn = image_to_dxtn(
                    image,
                    DxtnFormat::Dxt5,
                    make_mipmaps,
                    mipmap_filter,
                    compress_algorithm,
                )?;
                Ok(BlpImage {
                    header: BlpHeader {
                        version: BlpVersion::Blp2,
                        content: BlpContentTag::Direct,
                        flags: BlpFlags::Blp2 {
                            compression: Compression::Dxtc,
                            alpha_bits,
                            alpha_type: 7,
                            has_mipmaps: if make_mipmaps { 1 } else { 0 },
                        },
                        width,
                        height,
                        mipmap_locator: blp_dxtn.mipmap_locator(BlpVersion::Blp2),
                    },
                    content: BlpContent::Dxt5(blp_dxtn),
                })
            }
        },
    }
}

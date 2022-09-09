mod dxtn;
pub mod error;
mod jpeg;
mod mipmap;
mod raw1;
mod raw3;

use crate::types::*;
use ::image::{imageops::FilterType, DynamicImage};
use dxtn::*;
pub use error::Error;
use jpeg::*;
use raw1::*;
use raw3::*;

/// Convert from parsed raw BLP image to useful [DynamicImage]
pub fn blp_to_image(image: &BlpImage, mipmap_level: usize) -> Result<DynamicImage, Error> {
    match &image.content {
        BlpContent::Raw1(content) => raw1_to_image(&content, mipmap_level),
        BlpContent::Raw3(content) => raw3_to_image(&content, mipmap_level),
        BlpContent::Jpeg(content) => jpeg_to_image(&content, mipmap_level),
        BlpContent::Dxt1(content) => dxt1_to_image(&content, mipmap_level),
        BlpContent::Dxt3(content) => dxt3_to_image(&content, mipmap_level),
        BlpContent::Dxt5(content) => dxt5_to_image(&content, mipmap_level),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlpTarget {
    Blp0(BlpOldFormat),
    Blp1(BlpOldFormat),
    Blp2(Blp2Format),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlpOldFormat {
    Raw1 { alpha_bits: AlphaBits },
    Jpeg { has_alpha: bool },
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlphaBits {
    NoAlpha,
    Bit1,
    Bit4,
    Bit8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Blp2Format {
    Raw1 { alpha_bits: AlphaBits },
    Raw3,
    Jpeg { has_alpha: bool },
    Dxt1,
    Dxt3,
    Dxt5,
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
            BlpOldFormat::Raw1 { alpha_bits } => unimplemented!("raw1 blp0"),
            BlpOldFormat::Jpeg { has_alpha } => unimplemented!("jpeg blp0"),
        },
        BlpTarget::Blp1(format) => match format {
            BlpOldFormat::Raw1 { alpha_bits } => unimplemented!("raw1 blp1"),
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
            Blp2Format::Raw1 { alpha_bits } => unimplemented!("raw1 blp2"),
            Blp2Format::Raw3 => unimplemented!("raw3 blp2"),
            Blp2Format::Jpeg { has_alpha } => unimplemented!("jpeg blp2"),
            Blp2Format::Dxt1 => unimplemented!("dxt1 blp2"),
            Blp2Format::Dxt3 => unimplemented!("dxt3 blp2"),
            Blp2Format::Dxt5 => unimplemented!("dxt5 blp2"),
        },
    }
}

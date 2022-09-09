use super::error::Error;
use crate::types::*;
use ::image::DynamicImage;

pub fn raw1_to_image(image: &BlpRaw1, mipmap_level: usize) -> Result<DynamicImage, Error> {
    unimplemented!();
}

pub fn image_to_raw1(image: &DynamicImage, make_mipmaps: bool) -> Result<BlpRaw1, Error> {
    unimplemented!();
}
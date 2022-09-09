use super::error::Error;
use crate::types::*;
use ::image::DynamicImage;

pub fn raw3_to_image(image: &BlpRaw3, mipmap_level: usize) -> Result<DynamicImage, Error> {
    unimplemented!();
}

pub fn image_to_raw3(image: &DynamicImage, make_mipmaps: bool) -> Result<BlpRaw3, Error> {
    unimplemented!();
}
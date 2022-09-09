use super::error::Error;
use crate::types::*;
use ::image::DynamicImage;

pub fn dxt1_to_image(image: &BlpDxt1, mipmap_level: usize) -> Result<DynamicImage, Error> {
    unimplemented!();
}

pub fn image_to_dxt1(image: &DynamicImage, make_mipmaps: bool) -> Result<BlpDxt1, Error> {
    unimplemented!();
}

pub fn dxt3_to_image(image: &BlpDxt3, mipmap_level: usize) -> Result<DynamicImage, Error> {
    unimplemented!();
}

pub fn image_to_dxt3(image: &DynamicImage, make_mipmaps: bool) -> Result<BlpDxt3, Error> {
    unimplemented!();
}

pub fn dxt5_to_image(image: &BlpDxt5, mipmap_level: usize) -> Result<DynamicImage, Error> {
    unimplemented!();
}

pub fn image_to_dxt5(image: &DynamicImage, make_mipmaps: bool) -> Result<BlpDxt5, Error> {
    unimplemented!();
}
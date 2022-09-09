use thiserror::Error;
use ::image::error::ImageError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("There is no body in the BLP image!")]
    MissingRootImage,
    #[error("Convertation error: {0}")]
    Convert(#[from] ImageError),
    #[error("Maximum value for width is 65,535")]
    WidthTooLarge(u32),
    #[error("Maximum value for height is 65,535")]
    HeightTooLarge(u32),
}
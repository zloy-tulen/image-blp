use ::image::error::ImageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("There is no image in the BLP mipmaps level {0}!")]
    MissingImage(usize),
    #[error("Convertation error: {0}")]
    Convert(#[from] ImageError),
    #[error("Maximum value for width is 65,535")]
    WidthTooLarge(u32),
    #[error("Maximum value for height is 65,535")]
    HeightTooLarge(u32),
    #[error(
        "Header sizes for mipmap {0} are {1}x{2}, but there are {3} pixels actually in content."
    )]
    MismatchSizes(usize, u32, u32, usize),
    #[error(
        "Header sizes for mipmap {0} are {1}x{2}, but there are {3} alpha values actually in content."
    )]
    MismatchAlphaSizes(usize, u32, u32, usize),
    #[error("There are invalid alpha bits for the Raw1 format. Got {0}, expected: 0, 1, 4, 8.")]
    Raw1InvalidAlphaBits(u32),
    #[error("Color map length {0}, 256 expected!")]
    ColorMapLengthInvalid(usize),
    #[error("Expected palette of 255 colors, but got {0}")]
    PaletteWrongSize(usize),
    #[error("Failed to process bytes from DXT1 decomporession")]
    Dxt1RawConvertFail,
}

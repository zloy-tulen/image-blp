#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpJpeg {
    /// JPEG header that is appended to each mipmap level data
    pub header: Vec<u8>,
    /// Image itself and all mipmaps levels. If there are no mipmaps,
    /// the length of the vector is 1.
    pub images: Vec<Vec<u8>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpRaw3 {
    /// The cmap field array is the colour look up table used for an indexed
    /// colour model. Each element represents 24 bit RGB colour component values
    /// in the order of 0xBBGGRR. The final byte is alignment padding and will
    /// not alter the decoded image in any way. One might be able to improve the
    /// file compressibility by carefully choosing padding values.
    pub cmap: Vec<u32>,
    /// Image itself and all mipmaps levels. If there are no mipmaps,
    /// the length of the vector is 1.
    pub images: Vec<Raw3Image>,
}

/// Each mipmap contains what appears to be
/// 32 bit BGRA data. `alpha_bits` seems to represent a set of bit flags
/// rather than depth, as all images of this type seem to have 4 bytes per
/// pixel regardless of depth, and it has been seen to exceed 8. Their
/// meaning is unknown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw3Image {
    pub pixels: Vec<u32>,
}
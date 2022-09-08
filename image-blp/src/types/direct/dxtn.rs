#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpDxt1 {
    pub images: Vec<Dxt1Image>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dxt1Image {
    pub blocks: Vec<Dxt1Block>,
}

/// Each block is 64 bits and begins with two 16 bit values, and are used to
/// derived a 4 color palette.
///
/// The values are interpreted as 565 RGB colors, with the least significant
/// bits corresponding to blue, to create the first two colors in the
/// palette.
///
/// If the first value is less than or equal to the second, the final entry
/// of the palette is reserved. If `alpha_bits` is 0, the reserved color is
/// black. If `alpha_bits` is 1, the reserved color is transparent.
///
/// The remaining colors are created by interpolating between the first two
/// colors in the palette.
///
/// The remaining 32 bits are 16 2-bit values acting as a lookups to specify
/// the colors in the block.
/// 
/// See more at [wiki](http://en.wikipedia.org/wiki/S3TC)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dxt1Block {
    pub color1: u16,
    pub color2: u16,
    pub color_indecies: u32, 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpDxt3 {
    pub images: Vec<Dxt3Image>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dxt3Image {
    pub blocks: Vec<Dxt3Block>,
}

/// Each block is 128 bits and begins identically to DXT1, except that no
/// special color is reserved in the palette.
///
/// It is followed by 16 4-bit values corresponding to the alpha values for
/// each of the pixels in the block.
/// 
/// See more at [wiki](http://en.wikipedia.org/wiki/S3TC)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dxt3Block {
    pub color1: u16,
    pub color2: u16,
    pub color_indecies: u32, 
    pub alphas: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlpDxt5 {
    pub images: Vec<Dxt5Image>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dxt5Image {
    pub blocks: Vec<Dxt5Block>,
}

/// Each block is 128 bits and begins with two 8-bit values to create an 8
/// element lookup table for alpha values.
///
/// The first two elements in the lookup table are copies of those values.
///
/// If the first value is less than or equal to the second, the final two
/// entries of the lookup table are reserved for transparent and opaque.
///
/// The remaining entries are created by interpolating between the first two
/// entries in the lookup table.
///
/// The next 48 bits make up 16 3-bit values acting as lookups specifying
/// the alpha values for each of the pixels in the block.
///
/// The remaining 64 bits are identical to DXT1, except that no special
/// color is reserved in the palette.
/// 
/// See more at [wiki](http://en.wikipedia.org/wiki/S3TC)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dxt5Block {
    pub alpha1: u8,
    pub alpha2: u8,
    pub alpha_indecies1: u32,
    pub alpha_indecies2: u16,
    pub color1: u16,
    pub color2: u16,
    pub color_indecies: u32, 
}

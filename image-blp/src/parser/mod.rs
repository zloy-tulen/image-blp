pub mod error;
pub mod types;
mod header;

#[cfg(test)]
mod tests;

use super::types::*;
pub use error::Error;
use header::parse_header;
use log::*;
use nom::{
    error::context,
    multi::count,
    number::complete::{le_u32, le_u8},
    Err, IResult,
};

/// Binary parser for BLP format that produces [Error] when something went wrong
pub type Parser<'a, T> = IResult<&'a [u8], T, Error<&'a [u8]>>;

/// Parse BLP file from slice and fail if we require parse external files (case BLP0)
pub fn parse_blp(input: &[u8]) -> Parser<BlpImage> {
    parse_blp_with_externals(input, no_mipmaps)
}

/// Helper for `parse_blp` when no external mipmaps are needed
pub fn no_mipmaps<'a>(_: u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>> {
    Ok(None)
}

/// Parse BLP file from slice and use user provided callback to read mipmaps
pub fn parse_blp_with_externals<'a, F>(
    root_input: &'a [u8],
    external_mipmaps: F,
) -> Parser<'a, BlpImage>
where
    F: FnMut(u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>> + Clone,
{
    // Parse header
    let (input, header) = context("header", parse_header)(root_input)?;

    // Parse image content
    let (input, content) = context("image content", |input| {
        parse_content(
            &header,
            external_mipmaps.clone(),
            root_input,
            input,
        )
    })(input)?;

    Ok((input, BlpImage { header, content }))
}

fn parse_content<'a, F>(
    blp_header: &BlpHeader,
    external_mipmaps: F,
    original_input: &'a [u8],
    input: &'a [u8],
) -> Parser<'a, BlpContent>
where
    F: FnMut(u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>> + Clone,
{
    match blp_header.content {
        BlpContentTag::Jpeg => {
            let (input, content) = context("jpeg content", |input| {
                parse_jpeg_contentt(
                    blp_header,
                    external_mipmaps.clone(),
                    original_input,
                    input,
                )
            })(input)?;
            Ok((input, BlpContent::Jpeg(content)))
        }
        BlpContentTag::Direct => {
            let (input, content) = context("direct content", |input| {
                parse_direct_content(
                    blp_header,
                    external_mipmaps.clone(),
                    original_input,
                    input,
                )
            })(input)?;
            Ok((input, BlpContent::Direct(content)))
        }
    }
}

fn parse_jpeg_contentt<'a, F>(
    blp_header: &BlpHeader,
    mut external_mipmaps: F,
    original_input: &'a [u8],
    input: &'a [u8],
) -> Parser<'a, BlpJpeg>
where
    F: FnMut(u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>>,
{
    let (input, header_size) = le_u32(input)?;
    let (input, header) = count(le_u8, header_size as usize)(input)?;
    let mut images = vec![];

    match blp_header.mipmap_locator {
        MipmapLocator::External => {
            let image0_bytes_opt =
                external_mipmaps(0).map_err(|e| Err::Failure(Error::ExternalMipmap(0, e)))?;
            let image0_bytes =
                image0_bytes_opt.ok_or_else(|| Err::Failure(Error::MissingImage(0)))?;
            images.push(image0_bytes.to_vec());

            if blp_header.has_mipmaps() {
                // funny that there is no hard limit for number of mipmaps
                for i in 1..blp_header.mipmaps_count() + 1 {
                    let image_bytes_opt = external_mipmaps(i)
                        .map_err(|e| Err::Failure(Error::ExternalMipmap(i, e)))?;
                    let image_bytes =
                        image_bytes_opt.ok_or_else(|| Err::Failure(Error::MissingImage(i)))?;
                    images.push(image_bytes.to_vec());
                }
            }
        }
        MipmapLocator::Internal { offsets, sizes } => {
            let mut read_image = |i: u32| {
                let offset = offsets[i as usize];
                let size = sizes[i as usize];
                if offset as usize >= original_input.len() {
                    error!(
                        "Offset of mipmap {} is out of bounds! {} >= {}",
                        i,
                        offset,
                        original_input.len()
                    );
                    return Err(Err::Failure(Error::<&[u8]>::OutOfBounds(0)));
                }
                if (offset + size) as usize > original_input.len() {
                    error!(
                        "Offset+size of mipmap {} is out of bounds! {} > {}",
                        i,
                        offset + size,
                        original_input.len()
                    );
                    return Err(Err::Failure(Error::OutOfBounds(0)));
                }

                let image_bytes = &original_input[offset as usize..(offset + size) as usize];
                images.push(image_bytes.to_vec());
                Ok(())
            };

            read_image(0)?;
            if blp_header.has_mipmaps() {
                for i in 1..(blp_header.mipmaps_count() + 1).max(16) {
                    read_image(i)?;
                }
            }
        }
    }

    Ok((input, BlpJpeg { header, images }))
}

fn parse_direct_content<'a, F>(
    blp_header: &BlpHeader,
    mut external_mipmaps: F,
    original_input: &'a [u8],
    input: &'a [u8],
) -> Parser<'a, BlpDirect>
where
    F: FnMut(u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>>,
{
    let (input, cmap) = count(le_u32, 256)(input)?;
    let mut images = vec![];

    match blp_header.mipmap_locator {
        MipmapLocator::External => {
            let mut read_mipmap = |i| {
                let image_bytes_opt =
                    external_mipmaps(i).map_err(|e| Err::Failure(Error::ExternalMipmap(i, e)))?;
                let image_bytes =
                    image_bytes_opt.ok_or_else(|| Err::Failure(Error::MissingImage(i)))?;
                let n = blp_header.mipmap_pixels(i);
                let (input, indexed_rgb) = count(le_u8, n as usize)(image_bytes)?;
                let an = (n * blp_header.alpha_bits() + 7) / 8;
                let (_, indexed_alpha) = count(le_u8, an as usize)(input)?;

                images.push(DirectImage {
                    indexed_rgb,
                    indexed_alpha,
                });
                Ok(())
            };
            read_mipmap(0)?;

            if blp_header.has_mipmaps() {
                // funny that there is no hard limit for number of mipmaps
                for i in 1..blp_header.mipmaps_count() + 1 {
                    read_mipmap(i)?;
                }
            }
        }
        MipmapLocator::Internal { offsets, sizes } => {
            let mut read_image = |i: u32| {
                let offset = offsets[i as usize];
                let size = sizes[i as usize];
                if offset as usize >= original_input.len() {
                    error!(
                        "Offset of mipmap {} is out of bounds! {} >= {}",
                        i,
                        offset,
                        original_input.len()
                    );
                    return Err(Err::Failure(Error::<&[u8]>::OutOfBounds(0)));
                }
                if (offset + size) as usize > original_input.len() {
                    error!(
                        "Offset+size of mipmap {} is out of bounds! {} > {}",
                        i,
                        offset + size,
                        original_input.len()
                    );
                    return Err(Err::Failure(Error::OutOfBounds(0)));
                }

                let image_bytes = &original_input[offset as usize..(offset + size) as usize];
                let n = blp_header.mipmap_pixels(i);
                let (input, indexed_rgb) = count(le_u8, n as usize)(image_bytes)?;
                let an = (n * blp_header.alpha_bits() + 7) / 8;
                let (_, indexed_alpha) = count(le_u8, an as usize)(input)?;

                images.push(DirectImage {
                    indexed_rgb,
                    indexed_alpha,
                });
                Ok(())
            };

            read_image(0)?;
            if blp_header.has_mipmaps() {
                for i in 1..(blp_header.mipmaps_count() + 1).max(16) {
                    read_image(i)?;
                }
            }
        }
    }

    Ok((input, BlpDirect { cmap, images }))
}

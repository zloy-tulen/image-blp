mod direct;
pub mod error;
mod header;
mod jpeg;
pub mod types;

#[cfg(test)]
mod tests;

use super::types::*;
use direct::parse_direct_content;
pub use error::Error;
use header::parse_header;
use jpeg::parse_jpeg_content;
use nom::error::context;
use types::Parser;

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
        parse_content(&header, external_mipmaps.clone(), root_input, input)
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
                parse_jpeg_content(blp_header, external_mipmaps.clone(), original_input, input)
            })(input)?;
            Ok((input, BlpContent::Jpeg(content)))
        }
        BlpContentTag::Direct => {
            let (input, content) = context("direct content", |input| {
                parse_direct_content(blp_header, external_mipmaps.clone(), original_input, input)
            })(input)?;
            Ok((input, content))
        }
    }
}

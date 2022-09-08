use super::super::error::Error;
use super::super::types::Parser;
use crate::types::*;
use nom::{multi::count, number::complete::le_u8, Err};

pub fn parse_blp0<'a, F>(
    blp_header: &BlpHeader,
    mut external_mipmaps: F,
    images: &mut Vec<Raw1Image>,
    input: &'a [u8],
) -> Parser<'a, ()>
where
    F: FnMut(u32) -> Result<Option<&'a[u8]>, Box<dyn std::error::Error>>,
{
    let mut read_mipmap = |i| {
        let image_bytes_opt =
            external_mipmaps(i).map_err(|e| Err::Failure(Error::ExternalMipmap(i, e)))?;
        let image_bytes = image_bytes_opt.ok_or_else(|| Err::Failure(Error::MissingImage(i)))?;
        let (_, image) = parse_raw1_image(blp_header, i, &image_bytes)?;
        images.push(image);

        Ok(())
    };
    read_mipmap(0)?;

    if blp_header.has_mipmaps() {
        for i in 1..(blp_header.mipmaps_count() + 1).min(16) {
            read_mipmap(i)?;
        }
    }
    Ok((input, ()))
}

fn parse_raw1_image<'a>(
    blp_header: &BlpHeader,
    mimpmap_number: u32,
    input: &'a [u8],
) -> Parser<'a, Raw1Image> {
    let n = blp_header.mipmap_pixels(mimpmap_number);
    let (input, indexed_rgb) = count(le_u8, n as usize)(input)?;
    let an = (n * blp_header.alpha_bits() + 7) / 8;
    let (input, indexed_alpha) = count(le_u8, an as usize)(input)?;

    Ok((
        input,
        Raw1Image {
            indexed_rgb,
            indexed_alpha,
        },
    ))
}

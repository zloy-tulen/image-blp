pub mod error;

use super::types::*;
pub use error::Error;
use log::*;
use nom::{
    bytes::complete::take,
    multi::count,
    number::complete::{le_u32, le_u8},
    Err, IResult,
};
use std::str;

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
    F: FnMut(u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>>,
{
    // Parse header
    let (input, header) = parse_header(root_input)?;
    // Parse mipmap locator
    let (input, mips_locator) = parse_mipmap_locator(&header, input)?;
    // Parse image content
    let (input, content) =
        parse_content(&header, &mips_locator, external_mipmaps, root_input, input)?;

    Ok((input, BlpImage { header, content }))
}

fn parse_header(input: &[u8]) -> Parser<BlpHeader> {
    let (input, version) = parse_magic(input)?;
    let (input, content_field) = le_u32(input)?;
    let content = content_field.try_into().unwrap_or_else(|_| {
        warn!(
            "Unexpected value for content {}, defaulting to jpeg",
            content_field
        );
        BlpContentTag::Jpeg
    });
    let (input, mut flags) = if version >= BlpVersion::Blp2 {
        let (input, encoding_type) = le_u8(input)?;
        let (input, alpha_bits) = le_u8(input)?;
        let (input, sample_type) = le_u8(input)?;
        let (input, has_mipmaps) = le_u8(input)?;
        (
            input,
            BlpFlags::Blp2 {
                encoding_type,
                alpha_bits,
                sample_type,
                has_mipmaps,
            },
        )
    } else {
        let (input, alpha_bits_raw) = le_u32(input)?;
        let alpha_bits = if content == BlpContentTag::Jpeg
            && (alpha_bits_raw != 0 && alpha_bits_raw != 8)
        {
            warn!("For jpeg content detected non standard alpha bits value {} when 0 or 8 is expected, defaulting to 0", alpha_bits_raw);
            0
        } else if content == BlpContentTag::Direct
            && (alpha_bits_raw != 0
                && alpha_bits_raw != 1
                && alpha_bits_raw != 4
                && alpha_bits_raw != 8)
        {
            warn!("For direct content detected non standard alpha bits value {} when 0, 1, 4 or 8 is expected, defaulting to 0", alpha_bits_raw);
            0
        } else {
            alpha_bits_raw
        };
        (
            input,
            BlpFlags::Old {
                alpha_bits,
                extra: 0,       // filled later
                has_mipmaps: 0, // filled later
            },
        )
    };
    let (input, width) = le_u32(input)?;
    let (input, height) = le_u32(input)?;
    let input = if let BlpFlags::Old {
        extra, has_mipmaps, ..
    } = &mut flags
    {
        let (input, extra_value) = le_u32(input)?;
        let (input, has_mipmaps_value) = le_u32(input)?;
        *extra = extra_value;
        *has_mipmaps = has_mipmaps_value;
        input
    } else {
        input
    };

    Ok((
        input,
        BlpHeader {
            version,
            content,
            flags,
            width,
            height,
        },
    ))
}

fn parse_magic(input: &[u8]) -> Parser<BlpVersion> {
    let mut magic_fixed: [u8; 4] = Default::default();
    let (input, magic) = take(4_u32)(input)?;
    magic_fixed.copy_from_slice(magic);
    let version = BlpVersion::from_magic(magic_fixed).ok_or_else(|| {
        Err::Failure(Error::WrongMagic(
            str::from_utf8(magic)
                .map(|s| s.to_owned())
                .unwrap_or_else(|_| format!("{:?}", magic)),
        ))
    })?;

    Ok((input, version))
}

fn parse_mipmap_locator<'a>(header: &BlpHeader, input: &'a [u8]) -> Parser<'a, MipmapLocator> {
    if header.version >= BlpVersion::Blp1 {
        let mut offsets: [u32; 16] = Default::default();
        let mut sizes: [u32; 16] = Default::default();
        let (input, offsets_vec) = count(le_u32, 16)(input)?;
        offsets.copy_from_slice(&offsets_vec);
        let (input, sizes_vec) = count(le_u32, 16)(input)?;
        sizes.copy_from_slice(&sizes_vec);
        Ok((input, MipmapLocator::Internal { offsets, sizes }))
    } else {
        // For BLP0 mipmaps are located in external files
        Ok((input, MipmapLocator::External))
    }
}

fn parse_content<'a, F>(
    blp_header: &BlpHeader,
    mips_locator: &MipmapLocator,
    external_mipmaps: F,
    original_input: &'a [u8],
    input: &'a [u8],
) -> Parser<'a, BlpContent>
where
    F: FnMut(u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>>,
{
    match blp_header.content {
        BlpContentTag::Jpeg => {
            let (input, content) = parse_jpeg_contentt(
                blp_header,
                mips_locator,
                external_mipmaps,
                original_input,
                input,
            )?;
            Ok((input, BlpContent::Jpeg(content)))
        }
        BlpContentTag::Direct => {
            let (input, content) = parse_direct_content(
                blp_header,
                mips_locator,
                external_mipmaps,
                original_input,
                input,
            )?;
            Ok((input, BlpContent::Direct(content)))
        }
    }
}

fn parse_jpeg_contentt<'a, F>(
    blp_header: &BlpHeader,
    mips_locator: &MipmapLocator,
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

    match mips_locator {
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
    mips_locator: &MipmapLocator,
    mut external_mipmaps: F,
    original_input: &'a [u8],
    input: &'a [u8],
) -> Parser<'a, BlpDirect>
where
    F: FnMut(u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>>,
{
    let (input, cmap) = count(le_u32, 256)(input)?;
    let mut images = vec![];

    match mips_locator {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    #[test]
    fn simplest_direct_blp_alpha() {
        let blp_bytes = include_bytes!("../../../assets/simple_with_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let header = BlpHeader {
            version: BlpVersion::Blp1,
            content: BlpContentTag::Direct,
            flags: BlpFlags::Old {
                alpha_bits: 8,
                extra: 3,
                has_mipmaps: 5,
            },
            width: 2,
            height: 2,
        };
        assert_eq!(parsed.header, header);
    }

    #[test]
    fn simplest_direct_blp_noalpha() {
        let blp_bytes = include_bytes!("../../../assets/simple_without_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let header = BlpHeader {
            version: BlpVersion::Blp1,
            content: BlpContentTag::Direct,
            flags: BlpFlags::Old {
                alpha_bits: 0,
                extra: 5,
                has_mipmaps: 5,
            },
            width: 2,
            height: 2,
        };
        assert_eq!(parsed.header, header);
    }

    #[test]
    fn simplest_jpg_blp() {
        let blp_bytes = include_bytes!("../../../assets/simple_jpg.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let header = BlpHeader {
            version: BlpVersion::Blp1,
            content: BlpContentTag::Jpeg,
            flags: BlpFlags::Old {
                alpha_bits: 8,
                extra: 3,
                has_mipmaps: 5,
            },
            width: 2,
            height: 2,
        };
        assert_eq!(parsed.header, header);
    }

    #[test]
    fn rect_direct_blp() {
        let blp_bytes = include_bytes!("../../../assets/rect_with_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let header = BlpHeader {
            version: BlpVersion::Blp1,
            content: BlpContentTag::Direct,
            flags: BlpFlags::Old {
                alpha_bits: 8,
                extra: 3,
                has_mipmaps: 5,
            },
            width: 2,
            height: 3,
        };
        assert_eq!(parsed.header, header);
    }

    #[test]
    fn rect_jpg_no_alpha_blp() {
        let blp_bytes = include_bytes!("../../../assets/rect_without_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let header = BlpHeader {
            version: BlpVersion::Blp1,
            content: BlpContentTag::Jpeg,
            flags: BlpFlags::Old {
                alpha_bits: 0,
                extra: 5,
                has_mipmaps: 5,
            },
            width: 2,
            height: 3,
        };
        assert_eq!(parsed.header, header);
    }

    #[test]
    fn blp0_test() {
        let blp_bytes = include_bytes!("../../../assets/blp0/WyvernRider.blp");
        let blp_mipmaps = vec![
            include_bytes!("../../../assets/blp0/WyvernRider.b00").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b01").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b02").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b03").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b04").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b05").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b06").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b07").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b08").to_vec(),
            include_bytes!("../../../assets/blp0/WyvernRider.b09").to_vec(),
        ];
        let (_, parsed) = parse_blp_with_externals(blp_bytes, |i| {
            if (i as usize) < blp_mipmaps.len() {
                Ok(Some(&blp_mipmaps[i as usize]))
            } else {
                Ok(None)
            }
        })
        .expect("successfull parsing");
        let header = BlpHeader {
            version: BlpVersion::Blp0,
            content: BlpContentTag::Jpeg,
            flags: BlpFlags::Old {
                alpha_bits: 8,
                extra: 4,
                has_mipmaps: 1,
            },
            width: 512,
            height: 256,
        };
        assert_eq!(parsed.header, header);
        assert_eq!(parsed.get_content_jpeg().expect("jpeg").images.len(), 10);
    }
}

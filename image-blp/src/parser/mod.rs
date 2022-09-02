pub mod error;

use super::types::{BlpContent, BlpFlags, BlpHeader, BlpImage, BlpVersion};
pub use error::Error;
use log::*;
use nom::{
    bytes::complete::take,
    number::complete::{le_u32, le_u8},
    Err, IResult,
};
use std::str;

/// Binary parser for BLP format that produces [Error] when something went wrong
pub type Parser<'a, T> = IResult<&'a [u8], T, Error<&'a [u8]>>;

/// Parse BLP file from slice
pub fn parse_blp(input: &[u8]) -> Parser<BlpImage> {
    let (input, header) = parse_header(input)?;

    Ok((input, BlpImage { header }))
}

fn parse_header(input: &[u8]) -> Parser<BlpHeader> {
    let (input, version) = parse_magic(input)?;
    let (input, content_field) = le_u32(input)?;
    let content = content_field.try_into().unwrap_or_else(|_| {
        warn!(
            "Unexpected value for content {}, defaulting to jpeg",
            content_field
        );
        BlpContent::Jpeg
    });
    let (input, flags) = if version >= BlpVersion::Blp2 {
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
        let alpha_bits = if content == BlpContent::Jpeg
            && (alpha_bits_raw != 0 && alpha_bits_raw != 8)
        {
            warn!("For jpeg content detected non standard alpha bits value {} when 0 or 8 is expected, defaulting to 0", alpha_bits_raw);
            0
        } else if content == BlpContent::Direct
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
        (input, BlpFlags::Old { alpha_bits })
    };
    let (input, width) = le_u32(input)?;
    let (input, height) = le_u32(input)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    #[test]
    fn simplest_direct_blp_alpha() {
        let blp_bytes = include_bytes!("../../../assets/simple_with_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let expected = BlpImage {
            header: BlpHeader {
                version: BlpVersion::Blp1,
                content: BlpContent::Direct,
                flags: BlpFlags::Old { alpha_bits: 8 },
                width: 2,
                height: 2,
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn simplest_direct_blp_noalpha() {
        let blp_bytes = include_bytes!("../../../assets/simple_without_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let expected = BlpImage {
            header: BlpHeader {
                version: BlpVersion::Blp1,
                content: BlpContent::Direct,
                flags: BlpFlags::Old { alpha_bits: 0 },
                width: 2,
                height: 2,
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn simplest_jpg_blp() {
        let blp_bytes = include_bytes!("../../../assets/simple_jpg.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let expected = BlpImage {
            header: BlpHeader {
                version: BlpVersion::Blp1,
                content: BlpContent::Jpeg,
                flags: BlpFlags::Old { alpha_bits: 8 },
                width: 2,
                height: 2,
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn rect_direct_blp() {
        let blp_bytes = include_bytes!("../../../assets/rect_with_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let expected = BlpImage {
            header: BlpHeader {
                version: BlpVersion::Blp1,
                content: BlpContent::Direct,
                flags: BlpFlags::Old { alpha_bits: 8 },
                width: 2,
                height: 3,
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn rect_jpg_no_alpha_blp() {
        let blp_bytes = include_bytes!("../../../assets/rect_without_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let expected = BlpImage {
            header: BlpHeader {
                version: BlpVersion::Blp1,
                content: BlpContent::Jpeg,
                flags: BlpFlags::Old { alpha_bits: 0 },
                width: 2,
                height: 3,
            },
        };
        assert_eq!(parsed, expected);
    }
}

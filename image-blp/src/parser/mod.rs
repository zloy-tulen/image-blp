pub mod error;

use super::types::{BlpHeader, BlpImage, BlpVersion, BlpContent};
pub use error::Error;
use nom::{bytes::complete::take, number::complete::le_u32, IResult, Err};
use std::str;
use log::*;

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
        warn!("Unexpected value for content {}, defaulting to jpeg", content_field);
        BlpContent::Jpeg
    });

    Ok((input, BlpHeader { version, content }))
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

    #[test]
    fn simplest_direct_blp_alpha() {
        let blp_bytes = include_bytes!("../../../assets/simple_with_alpha.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let expected = BlpImage {
            header: BlpHeader {
                version: BlpVersion::Blp1,
                content: BlpContent::Direct,
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
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn simplest_direct_blp_jpg() {
        let blp_bytes = include_bytes!("../../../assets/simple_jpg.blp");
        let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
        let expected = BlpImage {
            header: BlpHeader {
                version: BlpVersion::Blp1,
                content: BlpContent::Jpeg,
            },
        };
        assert_eq!(parsed, expected);
    }
}

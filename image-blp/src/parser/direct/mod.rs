mod blp0;
mod blp1;
mod blp2;

use super::error::Error;
use super::types::Parser;
use crate::types::*;
use blp0::parse_blp0;
use blp1::parse_raw1;
use blp2::parse_raw3;
use log::*;
use nom::{error::context, multi::count, number::complete::le_u32, Err};

pub fn parse_direct_content<'a, F>(
    blp_header: &BlpHeader,
    external_mipmaps: F,
    original_input: &'a [u8],
    input: &'a [u8],
) -> Parser<'a, BlpDirect>
where
    F: FnMut(u32) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>>,
{
    let (input, cmap) = count(le_u32, 256)(input)?;
    let mut images = vec![];

    let (input, _) = match blp_header.flags {
        BlpFlags::Blp2 { compression, .. } => match blp_header.mipmap_locator {
            MipmapLocator::External => {
                error!("BLP2 doesn't support external mipmaps!");
                return Err(Err::Failure(Error::<&[u8]>::Blp2NoExternalMips));
            }
            MipmapLocator::Internal { offsets, sizes } => match compression {
                Compression::Raw1 => context("raw1 format", |input| {
                    parse_raw1(
                        blp_header,
                        original_input,
                        &offsets,
                        &sizes,
                        &mut images,
                        input,
                    )
                })(input)?,
                Compression::Raw3 => context("raw3 format", |input| {
                    parse_raw3(
                        blp_header,
                        original_input,
                        &offsets,
                        &sizes,
                        &mut images,
                        input,
                    )
                })(input)?,
                Compression::Dxtc => unimplemented!("dxtc"),
            },
        },
        BlpFlags::Old { .. } => match blp_header.mipmap_locator {
            MipmapLocator::External => {
                parse_blp0(blp_header, external_mipmaps, &mut images, input)?
            }
            MipmapLocator::Internal { offsets, sizes } => parse_raw1(
                blp_header,
                original_input,
                &offsets,
                &sizes,
                &mut images,
                input,
            )?,
        },
    };

    Ok((input, BlpDirect { cmap, images }))
}

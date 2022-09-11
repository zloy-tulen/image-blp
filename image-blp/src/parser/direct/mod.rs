mod blp0;
mod blp1;
mod blp2;

use super::error::Error;
use super::types::Parser;
use crate::types::*;
use blp0::parse_blp0;
use blp1::parse_raw1;
use blp2::{parse_dxt1, parse_dxt3, parse_dxt5, parse_raw3};
use log::*;
use nom::{error::context, multi::count, number::complete::le_u32, Err};

pub fn parse_direct_content<'a, F>(
    blp_header: &BlpHeader,
    external_mipmaps: F,
    original_input: &'a [u8],
    input: &'a [u8],
) -> Parser<'a, BlpContent>
where
    F: FnMut(usize) -> Result<Option<&'a [u8]>, Box<dyn std::error::Error>>,
{
    let (input, cmap) = context("color palette", count(le_u32, 256))(input)?;

    match blp_header.flags {
        BlpFlags::Blp2 {
            compression,
            alpha_type,
            ..
        } => match blp_header.mipmap_locator {
            MipmapLocator::External => {
                error!("BLP2 doesn't support external mipmaps!");
                Err(Err::Failure(Error::<&[u8]>::Blp2NoExternalMips))
            }
            MipmapLocator::Internal { offsets, sizes } => match compression {
                Compression::Raw1 => {
                    let mut images = vec![];
                    let (input, _) = context("raw1 format", |input| {
                        parse_raw1(
                            blp_header,
                            original_input,
                            &offsets,
                            &sizes,
                            &mut images,
                            input,
                        )
                    })(input)?;
                    Ok((input, BlpContent::Raw1(BlpRaw1 { cmap, images })))
                }
                Compression::Raw3 => {
                    let mut images = vec![];
                    let (input, _) = context("raw3 format", |input| {
                        parse_raw3(
                            // Actually the same at parsing level
                            blp_header,
                            original_input,
                            &offsets,
                            &sizes,
                            &mut images,
                            input,
                        )
                    })(input)?;
                    Ok((input, BlpContent::Raw3(BlpRaw3 { cmap, images })))
                }
                Compression::Dxtc if alpha_type == 0 => {
                    let mut images = vec![];
                    let (input, _) = context("dxt1 format", |input| {
                        parse_dxt1(
                            blp_header,
                            original_input,
                            &offsets,
                            &sizes,
                            &mut images,
                            input,
                        )
                    })(input)?;
                    Ok((input, BlpContent::Dxt1(BlpDxt1 { images })))
                }
                Compression::Dxtc if alpha_type == 1 => {
                    let mut images = vec![];
                    let (input, _) = context("dxt3 format", |input| {
                        parse_dxt3(
                            blp_header,
                            original_input,
                            &offsets,
                            &sizes,
                            &mut images,
                            input,
                        )
                    })(input)?;

                    Ok((input, BlpContent::Dxt3(BlpDxt3 { images })))
                }
                Compression::Dxtc if alpha_type == 7 => {
                    let mut images = vec![];
                    let (input, _) = context("dxt5 format", |input| {
                        parse_dxt5(
                            blp_header,
                            original_input,
                            &offsets,
                            &sizes,
                            &mut images,
                            input,
                        )
                    })(input)?;
                    Ok((input, BlpContent::Dxt5(BlpDxt5 { images })))
                }
                Compression::Dxtc => {
                    error!("Alpha type {} is not supported for BLP2!", alpha_type);
                    Err(Err::Failure(Error::<&[u8]>::Blp2UnknownAlphaType(
                        alpha_type,
                    )))
                }
            },
        },
        BlpFlags::Old { .. } => {
            let mut images = vec![];
            let (input, _) = match blp_header.mipmap_locator {
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
            };
            Ok((input, BlpContent::Raw1(BlpRaw1 { cmap, images })))
        }
    }
}

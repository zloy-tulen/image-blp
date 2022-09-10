use super::super::error::Error;
use super::super::types::Parser;
use crate::types::*;
use log::*;
use nom::{
    error::context,
    multi::count,
    number::complete::{le_u16, le_u32, le_u64, le_u8},
    Err,
};

pub fn parse_raw3<'a>(
    blp_header: &BlpHeader,
    original_input: &'a [u8],
    offsets: &[u32],
    sizes: &[u32],
    images: &mut Vec<Raw3Image>,
    input: &'a [u8],
) -> Parser<'a, ()> {
    let mut read_image = |i: usize| {
        let offset = offsets[i];
        let size = sizes[i];
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

        trace!("Expecting size of image: {}", size);
        let image_bytes = &original_input[offset as usize..(offset + size) as usize];
        trace!("We have {} bytes", image_bytes.len());
        let n = blp_header.mipmap_pixels(i);
        trace!("For mipmap size {:?} we should fetch {} bytes", blp_header.mipmap_size(i), n*4);
        let (_, pixels) = count(le_u32, n as usize)(image_bytes)?;

        images.push(Raw3Image {
            pixels,
        });
        Ok(())
    };

    trace!("Mipmaps count: {}", blp_header.mipmaps_count());
    read_image(0)?;
    if blp_header.has_mipmaps() {
        for i in 1..(blp_header.mipmaps_count() + 1).min(16) {
            if sizes[i as usize] == 0 {
                trace!("Size of mipmap {} is 0 bytes, I stop reading of images", i);
                break;
            }
            read_image(i)?;
        }
    }
    Ok((input, ()))
}

pub fn parse_dxt1<'a>(
    blp_header: &BlpHeader,
    original_input: &'a [u8],
    offsets: &[u32],
    sizes: &[u32],
    images: &mut Vec<Dxt1Image>,
    input: &'a [u8],
) -> Parser<'a, ()> {
    trace!("{:?}", blp_header);

    let mut read_image = |i: usize| {
        let offset = offsets[i];
        let size = sizes[i];
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
        let blocks_n = ((n as f32) / 16.0).ceil() as u32;

        let (_, blocks) =
            context("dxt1 block", count(dxt1_block, blocks_n as usize))(image_bytes)?;

        images.push(Dxt1Image { blocks });
        Ok(())
    };

    read_image(0)?;
    if blp_header.has_mipmaps() {
        trace!("Mipmaps count: {}", blp_header.mipmaps_count());
        for i in 1..(blp_header.mipmaps_count() + 1).min(16) {
            read_image(i)?;
        }
    }
    Ok((input, ()))
}

pub fn parse_dxt3<'a>(
    blp_header: &BlpHeader,
    original_input: &'a [u8],
    offsets: &[u32],
    sizes: &[u32],
    images: &mut Vec<Dxt3Image>,
    input: &'a [u8],
) -> Parser<'a, ()> {
    trace!("{:?}", blp_header);

    let mut read_image = |i: usize| {
        let offset = offsets[i];
        let size = sizes[i];
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
        let blocks_n = ((n as f32) / 16.0).ceil() as u32;

        let (_, blocks) =
            context("dxt3 block", count(dxt3_block, blocks_n as usize))(image_bytes)?;

        images.push(Dxt3Image { blocks });
        Ok(())
    };

    read_image(0)?;
    if blp_header.has_mipmaps() {
        trace!("Mipmaps count: {}", blp_header.mipmaps_count());
        for i in 1..(blp_header.mipmaps_count() + 1).min(16) {
            read_image(i)?;
        }
    }
    Ok((input, ()))
}

pub fn parse_dxt5<'a>(
    blp_header: &BlpHeader,
    original_input: &'a [u8],
    offsets: &[u32],
    sizes: &[u32],
    images: &mut Vec<Dxt5Image>,
    input: &'a [u8],
) -> Parser<'a, ()> {
    trace!("{:?}", blp_header);

    let mut read_image = |i: usize| {
        let offset = offsets[i];
        let size = sizes[i];
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
        let blocks_n = ((n as f32) / 16.0).ceil() as u32;

        let (_, blocks) =
            context("dxt5 block", count(dxt5_block, blocks_n as usize))(image_bytes)?;

        images.push(Dxt5Image { blocks });
        Ok(())
    };

    read_image(0)?;
    if blp_header.has_mipmaps() {
        trace!("Mipmaps count: {}", blp_header.mipmaps_count());
        for i in 1..(blp_header.mipmaps_count() + 1).min(16) {
            read_image(i)?;
        }
    }
    Ok((input, ()))
}

fn dxt1_block(input: &[u8]) -> Parser<Dxt1Block> {
    let (input, color1) = context("color1", le_u16)(input)?;
    let (input, color2) = context("color2", le_u16)(input)?;
    let (input, color_indecies) = context("color_indecies", le_u32)(input)?;
    Ok((
        input,
        Dxt1Block {
            color1,
            color2,
            color_indecies,
        },
    ))
}

fn dxt3_block(input: &[u8]) -> Parser<Dxt3Block> {
    let (input, color1) = context("color1", le_u16)(input)?;
    let (input, color2) = context("color2", le_u16)(input)?;
    let (input, color_indecies) = context("color_indecies", le_u32)(input)?;
    let (input, alphas) = context("alphas", le_u64)(input)?;
    Ok((
        input,
        Dxt3Block {
            color1,
            color2,
            color_indecies,
            alphas,
        },
    ))
}

fn dxt5_block(input: &[u8]) -> Parser<Dxt5Block> {
    let (input, alpha1) = context("alpha1", le_u8)(input)?;
    let (input, alpha2) = context("alpha2", le_u8)(input)?;
    let (input, alpha_indecies1) = context("alpha_indecies1", le_u32)(input)?;
    let (input, alpha_indecies2) = context("alpha_indecies2", le_u16)(input)?;

    let (input, color1) = context("color1", le_u16)(input)?;
    let (input, color2) = context("color2", le_u16)(input)?;
    let (input, color_indecies) = context("color_indecies", le_u32)(input)?;
    Ok((
        input,
        Dxt5Block {
            alpha1,
            alpha2,
            alpha_indecies1,
            alpha_indecies2,
            color1,
            color2,
            color_indecies,
        },
    ))
}

pub mod error;
mod primitives;

use super::types::*;
use error::Error;
use log::*;
use primitives::{push_le_u16, push_le_u32, push_le_u64};
use std::iter::{repeat, zip};

/// BLP file bytes with vector of external mipmaps encoded
pub struct BlpWithMipmaps {
    pub blp_bytes: Vec<u8>,
    pub blp_mipmaps: Vec<Vec<u8>>,
}

/// Encode BLP0 with external mipmaps
pub fn encode_blp0(image: &BlpImage) -> Result<BlpWithMipmaps, Error> {
    encode_blp_with_external(image)
}

/// Encode BLP1 or BLP2 into bytes
pub fn encode_blp(image: &BlpImage) -> Result<Vec<u8>, Error> {
    let res = encode_blp_with_external(image)?;
    Ok(res.blp_bytes)
}

fn encode_blp_with_external(image: &BlpImage) -> Result<BlpWithMipmaps, Error> {
    let mut output = vec![];
    let mut mipmaps = vec![];
    trace!("Encode header");
    encode_header(&image.header, &mut output)?;
    trace!("Encode content");
    encode_content(&image.header, &image.content, &mut output, &mut mipmaps)?;
    Ok(BlpWithMipmaps {
        blp_bytes: output,
        blp_mipmaps: mipmaps,
    })
}

fn encode_header(header: &BlpHeader, output: &mut Vec<u8>) -> Result<(), Error> {
    output.extend(header.version.to_magic());
    push_le_u32(header.content.into(), output);
    match header.flags {
        BlpFlags::Old { alpha_bits, .. } => {
            push_le_u32(alpha_bits, output);
        }
        BlpFlags::Blp2 {
            compression,
            alpha_bits,
            alpha_type,
            has_mipmaps,
        } => {
            output.push(compression.into());
            output.push(alpha_bits);
            output.push(alpha_type);
            output.push(has_mipmaps);
        }
    }

    if header.width > 65535 {
        return Err(Error::WidthTooHigh(header.width));
    }
    push_le_u32(header.width, output);
    if header.height > 65535 {
        return Err(Error::WidthTooHigh(header.height));
    }
    push_le_u32(header.height, output);

    if let BlpFlags::Old {
        extra, has_mipmaps, ..
    } = header.flags
    {
        push_le_u32(extra, output);
        push_le_u32(has_mipmaps, output);
    }

    match header.mipmap_locator {
        MipmapLocator::Internal { offsets, sizes } => {
            for offset in offsets {
                push_le_u32(offset, output);
            }
            for size in sizes {
                push_le_u32(size, output);
            }
        }
        MipmapLocator::External => {
            if header.version > BlpVersion::Blp0 {
                error!("External mipmaps are not supported for versions higher than BLP0!");
                return Err(Error::ExternalMipmapsNotSupported(header.version));
            }
        }
    }
    Ok(())
}

fn encode_content(
    header: &BlpHeader,
    content: &BlpContent,
    output: &mut Vec<u8>,
    mipmaps: &mut Vec<Vec<u8>>,
) -> Result<(), Error> {
    match content {
        BlpContent::Jpeg(jpeg_content) => encode_jpeg(header, jpeg_content, output, mipmaps),
        BlpContent::Raw1(raw1_content) => encode_raw1(header, raw1_content, output, mipmaps),
        BlpContent::Raw3(raw3_content) => encode_raw3(header, raw3_content, output, mipmaps),
        BlpContent::Dxt1(dxt1_content) => encode_dxt1(header, dxt1_content, output),
        BlpContent::Dxt3(dxt3_content) => encode_dxt3(header, dxt3_content, output),
        BlpContent::Dxt5(dxt5_content) => encode_dxt5(header, dxt5_content, output),
    }
}

fn encode_jpeg(
    header: &BlpHeader,
    content: &BlpJpeg,
    output: &mut Vec<u8>,
    mipmaps: &mut Vec<Vec<u8>>,
) -> Result<(), Error> {
    // To produce identical files, reproducting bug that leads to leave 2 bytes
    // of header uncovered by length
    push_le_u32((content.header.len() - 2) as u32, output);
    output.extend(content.header.iter());

    match header.mipmap_locator {
        MipmapLocator::External => {
            for image in content.images.iter() {
                mipmaps.push(image.clone())
            }
        }
        MipmapLocator::Internal { offsets, sizes } => {
            let mut pairs: Vec<(u32, u32)> = zip(offsets, sizes)
                .take((header.mipmaps_count() + 1) as usize)
                .filter(|(_, size)| *size > 0)
                .collect();
            pairs.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).expect("number cmp"));

            trace!(
                "Mipmaps ordered: {:?}, images count: {}",
                pairs,
                content.images.len()
            );
            for (i, ((offset, size), image)) in zip(pairs, content.images.iter()).enumerate() {
                trace!("Writing mipmap {}", i);
                let padding = (if offset as usize >= output.len() {
                    Ok(offset as usize - output.len())
                } else {
                    Err(Error::InvalidOffset(i as u32, offset, output.len() as u32))
                })?;
                if padding > 0 {
                    output.extend(repeat(0).take(padding));
                }
                if image.len() != size as usize {
                    return Err(Error::InvalidMipmapSize(i as u32, size, image.len() as u32));
                }
                output.extend(image);
            }
        }
    }
    Ok(())
}

fn encode_raw<T, F>(
    header: &BlpHeader,
    cmap: &[u32],
    images: &[T],
    mut encoder: F, 
    output: &mut Vec<u8>,
    mipmaps: &mut Vec<Vec<u8>>,
) -> Result<(), Error> 
where 
    F: FnMut(&T, &mut Vec<u8>)
{
    trace!("Header: {:?}", header);

    for c in cmap.iter() {
        push_le_u32(*c, output);
    }

    match header.mipmap_locator {
        MipmapLocator::External => {
            for image in images.iter() {
                let mut image_bytes = vec![];
                encoder(image, &mut image_bytes);
                mipmaps.push(image_bytes);
            }
        }
        MipmapLocator::Internal { offsets, sizes } => {
            let mut pairs: Vec<(u32, u32)> = zip(offsets, sizes)
                .take((header.mipmaps_count() + 1) as usize)
                .filter(|(_, size)| *size > 0)
                .collect();
            pairs.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).expect("number cmp"));

            trace!(
                "Mipmaps ordered: {:?}, images count: {}",
                pairs,
                images.len()
            );
            for (i, ((offset, size), image)) in zip(pairs, images.iter()).enumerate() {
                trace!("Writing mipmap {}", i);
                let padding = (if offset as usize >= output.len() {
                    Ok(offset as usize - output.len())
                } else {
                    Err(Error::InvalidOffset(i as u32, offset, output.len() as u32))
                })?;
                if padding > 0 {
                    output.extend(repeat(0).take(padding));
                }
                let mut image_bytes = vec![];
                encoder(image, &mut image_bytes);
                if image_bytes.len() != size as usize {
                    return Err(Error::InvalidMipmapSize(
                        i as u32,
                        size,
                        image_bytes.len() as u32,
                    ));
                }
                output.extend(image_bytes);
            }
        }
    }
    Ok(())
}

fn encode_raw1(
    header: &BlpHeader,
    content: &BlpRaw1,
    output: &mut Vec<u8>,
    mipmaps: &mut Vec<Vec<u8>>,
) -> Result<(), Error> {
    encode_raw(header, &content.cmap, &content.images, |image, output| encode_raw1_image(image, output), output, mipmaps)
}

fn encode_raw3(
    header: &BlpHeader,
    content: &BlpRaw3,
    output: &mut Vec<u8>,
    mipmaps: &mut Vec<Vec<u8>>,
) -> Result<(), Error> {
    encode_raw(header, &content.cmap, &content.images, |image, output| encode_raw3_image(image, output), output, mipmaps)
}

fn encode_raw1_image(image: &Raw1Image, output: &mut Vec<u8>) {
    output.extend(image.indexed_rgb.iter());
    output.extend(image.indexed_alpha.iter());
}

fn encode_raw3_image(image: &Raw3Image, output: &mut Vec<u8>) {
    for pixel in image.pixels.iter() {
        push_le_u32(*pixel, output);
    }
}

fn encode_dxtn<F, T>(
    header: &BlpHeader,
    images: &[T],
    mut writer: F,
    output: &mut Vec<u8>,
) -> Result<(), Error>
where
    F: FnMut(&T, &mut Vec<u8>),
{
    trace!("Header: {:?}", header);
    let (offsets, sizes) = if let MipmapLocator::Internal { offsets, sizes } = header.mipmap_locator
    {
        (offsets, sizes)
    } else {
        return Err(Error::ExternalMipmapsNotSupported(BlpVersion::Blp2));
    };

    let mut pairs: Vec<(u32, u32)> = zip(offsets, sizes)
        .take((header.mipmaps_count() + 1) as usize)
        .filter(|(_, size)| *size > 0)
        .collect();
    pairs.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).expect("number cmp"));

    trace!(
        "Mipmaps ordered: {:?}, images count: {}",
        pairs,
        images.len()
    );

    for (i, ((offset, size), image)) in zip(pairs, images.iter()).enumerate() {
        trace!("Writing mipmap {}", i);
        let padding = (if offset as usize >= output.len() {
            Ok(offset as usize - output.len())
        } else {
            Err(Error::InvalidOffset(i as u32, offset, output.len() as u32))
        })?;
        if padding > 0 {
            output.extend(repeat(0).take(padding));
        }
        let mut image_bytes = vec![];
        writer(image, &mut image_bytes);
        if image_bytes.len() != size as usize {
            return Err(Error::InvalidMipmapSize(
                i as u32,
                size,
                image_bytes.len() as u32,
            ));
        }
        output.extend(image_bytes);
    }
    Ok(())
}

fn encode_dxt1(header: &BlpHeader, content: &BlpDxt1, output: &mut Vec<u8>) -> Result<(), Error> {
    encode_dxtn(
        header,
        &content.images,
        |img, v| encode_dxt1_image(img, v),
        output,
    )
}

fn encode_dxt3(header: &BlpHeader, content: &BlpDxt3, output: &mut Vec<u8>) -> Result<(), Error> {
    encode_dxtn(
        header,
        &content.images,
        |img, v| encode_dxt3_image(img, v),
        output,
    )
}

fn encode_dxt5(header: &BlpHeader, content: &BlpDxt5, output: &mut Vec<u8>) -> Result<(), Error> {
    encode_dxtn(
        header,
        &content.images,
        |img, v| encode_dxt5_image(img, v),
        output,
    )
}

fn encode_dxt1_image(image: &Dxt1Image, output: &mut Vec<u8>) {
    for block in image.blocks.iter() {
        push_le_u16(block.color1, output);
        push_le_u16(block.color2, output);
        push_le_u32(block.color_indecies, output);
    }
}

fn encode_dxt3_image(image: &Dxt3Image, output: &mut Vec<u8>) {
    for block in image.blocks.iter() {
        push_le_u16(block.color1, output);
        push_le_u16(block.color2, output);
        push_le_u32(block.color_indecies, output);
        push_le_u64(block.alphas, output);
    }
}

fn encode_dxt5_image(image: &Dxt5Image, output: &mut Vec<u8>) {
    for block in image.blocks.iter() {
        output.push(block.alpha1);
        output.push(block.alpha2);
        push_le_u32(block.alpha_indecies1, output);
        push_le_u16(block.alpha_indecies2, output);
        push_le_u16(block.color1, output);
        push_le_u16(block.color2, output);
        push_le_u32(block.color_indecies, output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        let offsets = vec![356, 123, 567, 421];
        let sizes = vec![1, 2, 3, 4];
        let mut pairs: Vec<(u32, u32)> = zip(offsets, sizes).collect();
        pairs.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).expect("number cmp"));
        assert_eq!(pairs, vec![(123, 2), (356, 1), (421, 4), (567, 3)]);
    }
}

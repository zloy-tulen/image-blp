use super::error::Error;
use ::image::{imageops::FilterType, DynamicImage, ImageFormat};
use std::io::Cursor;

pub fn generate_mipmaps(image: DynamicImage, filter: FilterType) -> Result<Vec<Vec<u8>>, Error> {
    let mut root_img = vec![];
    image.write_to(&mut Cursor::new(&mut root_img), ImageFormat::Jpeg)?;
    let mut mipmaps = vec![root_img];

    let mut current_image = image;
    loop {
        let width = current_image.width();
        let height = current_image.height();
        if (width == 1 && height == 1) || mipmaps.len() >= 16 {
            break;
        }
        let new_width = width >> 1;
        let new_height = height >> 1;
        current_image = current_image.resize_exact(new_width, new_height, filter);
        let mut image_bytes = vec![];
        current_image.write_to(&mut Cursor::new(&mut image_bytes), ImageFormat::Jpeg)?;
        mipmaps.push(image_bytes);
    }
    Ok(mipmaps)
}

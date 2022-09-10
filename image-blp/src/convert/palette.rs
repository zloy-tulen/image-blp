use super::error::Error;
use ::image::{RgbImage, RgbaImage};
use rscolorq::{color::Rgb, spatial_color_quant, Matrix2d, Params};
use log::*;

/// Create image and palette for rgb image
pub fn quantize_rgba(img: RgbaImage) -> Result<(Vec<u8>, Vec<u32>), Error> {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let palette_size = 255; // last color in cmap is not used

    // Create the quantized palette index buffer
    let mut quantized_image = Matrix2d::new(width, height);

    // Build the quantization parameters, verify if accepting user input
    let mut conditions = Params::new();
    conditions.palette_size(palette_size);
    conditions.verify_parameters()?;

    // Convert the input image buffer from Rgb<u8> to Rgb<f64>
    let image = Matrix2d::from_vec(
        img.pixels()
            .map(|&c| Rgb {
                red: c[0] as f64 / 255.0,
                green: c[1] as f64 / 255.0,
                blue: c[2] as f64 / 255.0,
            })
            .collect(),
        width,
        height,
    );

    let mut palette = Vec::with_capacity(palette_size as usize);

    trace!("Called spatial_color_quant");
    spatial_color_quant(&image, &mut quantized_image, &mut palette, &conditions)?;
    trace!("Finished spatial_color_quant");

    // Convert the Rgb<f64> palette to Rgb<u8>
    let palette = palette
        .iter()
        .map(|&c| {
            let color = 255.0 * c;
            let red = (color.red.round() as u8) as u32;
            let green = (color.green.round() as u8) as u32;
            let blue = (color.blue.round() as u8) as u32;
            red + (green << 8) + (blue << 16)
        })
        .collect::<Vec<u32>>();

    Ok((quantized_image.into_raw_vec(), palette))
}

/// Quantize image with already known palette
pub fn quantize_rgba_known(img: RgbaImage, palette_raw: &[u32]) -> Result<Vec<u8>, Error> {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let palette_size = palette_raw.len() as u8;
    if palette_size != 255 {
        return Err(Error::PaletteWrongSize(palette_raw.len()));
    }
    let palette = palette_raw
        .iter()
        .map(|c| rscolorq::color::Rgb {
            red: ((c & 0xFF) as f64) / 255.0,
            green: (((c >> 8) & 0xFF) as f64) / 255.0,
            blue: (((c >> 16) & 0xFF) as f64) / 255.0,
        })
        .collect();
    // Create the quantized palette index buffer
    let mut quantized_image = Matrix2d::new(width, height);

    // Build the quantization parameters, verify if accepting user input
    let mut conditions = Params::new();
    conditions.palette(palette);
    conditions.palette_size(palette_size);
    conditions.verify_parameters()?;

    // Convert the input image buffer from Rgb<u8> to Rgb<f64>
    let image = Matrix2d::from_vec(
        img.pixels()
            .map(|&c| Rgb {
                red: c[0] as f64 / 255.0,
                green: c[1] as f64 / 255.0,
                blue: c[2] as f64 / 255.0,
            })
            .collect(),
        width,
        height,
    );

    let mut palette = Vec::with_capacity(palette_size as usize);

    spatial_color_quant(&image, &mut quantized_image, &mut palette, &conditions)?;

    Ok(quantized_image.into_raw_vec())
}

/// Create image and palette for rgb image
pub fn quantize_rgb(img: RgbImage) -> Result<(Vec<u8>, Vec<u32>), Error> {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let palette_size = 255; // last color in cmap is not used

    // Create the quantized palette index buffer
    let mut quantized_image = Matrix2d::new(width, height);

    // Build the quantization parameters, verify if accepting user input
    let mut conditions = Params::new();
    conditions.palette_size(palette_size);
    conditions.verify_parameters()?;

    // Convert the input image buffer from Rgb<u8> to Rgb<f64>
    let image = Matrix2d::from_vec(
        img.pixels()
            .map(|&c| Rgb {
                red: c[0] as f64 / 255.0,
                green: c[1] as f64 / 255.0,
                blue: c[2] as f64 / 255.0,
            })
            .collect(),
        width,
        height,
    );

    let mut palette = Vec::with_capacity(palette_size as usize);

    spatial_color_quant(&image, &mut quantized_image, &mut palette, &conditions)?;

    // Convert the Rgb<f64> palette to Rgb<u8>
    let palette = palette
        .iter()
        .map(|&c| {
            let color = 255.0 * c;
            let red = (color.red.round() as u8) as u32;
            let green = (color.green.round() as u8) as u32;
            let blue = (color.blue.round() as u8) as u32;
            red + (green << 8) + (blue << 16)
        })
        .collect::<Vec<u32>>();

    Ok((quantized_image.into_raw_vec(), palette))
}

/// Quantize image with already known palette
pub fn quantize_rgb_known(img: RgbImage, palette_raw: &[u32]) -> Result<Vec<u8>, Error> {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let palette_size = palette_raw.len() as u8;
    if palette_size != 255 {
        return Err(Error::PaletteWrongSize(palette_raw.len()));
    }
    let palette = palette_raw
        .iter()
        .map(|c| rscolorq::color::Rgb {
            red: ((c & 0xFF) as f64) / 255.0,
            green: (((c >> 8) & 0xFF) as f64) / 255.0,
            blue: (((c >> 16) & 0xFF) as f64) / 255.0,
        })
        .collect();
    // Create the quantized palette index buffer
    let mut quantized_image = Matrix2d::new(width, height);

    // Build the quantization parameters, verify if accepting user input
    let mut conditions = Params::new();
    conditions.palette(palette);
    conditions.palette_size(palette_size);
    conditions.verify_parameters()?;

    // Convert the input image buffer from Rgb<u8> to Rgb<f64>
    let image = Matrix2d::from_vec(
        img.pixels()
            .map(|&c| Rgb {
                red: c[0] as f64 / 255.0,
                green: c[1] as f64 / 255.0,
                blue: c[2] as f64 / 255.0,
            })
            .collect(),
        width,
        height,
    );

    let mut palette = Vec::with_capacity(palette_size as usize);

    spatial_color_quant(&image, &mut quantized_image, &mut palette, &conditions)?;

    Ok(quantized_image.into_raw_vec())
}

use ::image::{error::ImageError, imageops::FilterType, io::Reader as ImageReader, ImageFormat};
use clap::{Parser, ValueEnum};
use image_blp::{convert::*, encode::error::Error as EncodeError, encode::save_blp, types::*};
use log::*;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to determine input format, please specify it via extension or by direct --input-format option")]
    MissingInputFormat,
    #[error("Failed to determine output format, please specify it via extension or by direct --output-format option")]
    MissingOutputFormat,
    #[error("Failed to load file {0} due: {1}")]
    LoadError(PathBuf, image_blp::parser::LoadError),
    #[error("Failed to convert file {0} due: {1}")]
    Convert(PathBuf, image_blp::convert::Error),
    #[error("Saving converted image from {0} to {1} failed due: {2}")]
    SaveError(PathBuf, PathBuf, ImageError),
    #[error("Saving converted image from {0} to {1} failed due: {2}")]
    BlpSaveError(PathBuf, PathBuf, EncodeError),
    #[error("Cannot find required level {1} of mipmaps for {0}")]
    WrongMipmap(PathBuf, usize),
    #[error("Invalid alpha bits value {2} for version {0:?} and format subtype {1:?}")]
    InvalidAlphaBits(OutputBlpVersion, OutputBlpFormat, u8),
    #[error("Version {0:?} doesn't support the format subtype {1:?}")]
    BlpOldSupport(OutputBlpVersion, OutputBlpFormat),
    #[error("Failed to open image {0} due {1}")]
    ImageOpenError(PathBuf, std::io::Error),
    #[error("Failed to decode image {0} due {1}")]
    ImageDecodeError(PathBuf, ImageError),
}

/// Input images that we can decode
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum InputFormat {
    Blp,
    Png,
    Jpeg,
    Gif,
    Bmp,
    Ico,
    Tiff,
    Webp,
    Pnm,
    Dds,
    Tga,
    OpenExr,
    Farbfeld,
}

fn guess_input_format(ext: &str) -> Option<InputFormat> {
    match ext.trim().to_lowercase().as_str() {
        "blp" => Some(InputFormat::Blp),
        "png" => Some(InputFormat::Png),
        "jpg" => Some(InputFormat::Jpeg),
        "jpeg" => Some(InputFormat::Jpeg),
        "gif" => Some(InputFormat::Gif),
        "bmp" => Some(InputFormat::Bmp),
        "ico" => Some(InputFormat::Ico),
        "tiff" => Some(InputFormat::Tiff),
        "webp" => Some(InputFormat::Webp),
        "pnm" => Some(InputFormat::Pnm),
        "dds" => Some(InputFormat::Dds),
        "tga" => Some(InputFormat::Tga),
        "exr" => Some(InputFormat::OpenExr),
        "ff" => Some(InputFormat::Farbfeld),
        _ => None,
    }
}

/// Output images that we can encode
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Blp,
    Png,
    Jpeg,
    Gif,
    Bmp,
    Ico,
    Tiff,
    Pnm,
    Tga,
    OpenExr,
    Farbfeld,
}

fn guess_output_format(ext: &str) -> Option<OutputFormat> {
    match ext.trim().to_lowercase().as_str() {
        "blp" => Some(OutputFormat::Blp),
        "png" => Some(OutputFormat::Png),
        "jpg" => Some(OutputFormat::Jpeg),
        "jpeg" => Some(OutputFormat::Jpeg),
        "gif" => Some(OutputFormat::Gif),
        "bmp" => Some(OutputFormat::Bmp),
        "ico" => Some(OutputFormat::Ico),
        "tiff" => Some(OutputFormat::Tiff),
        "pnm" => Some(OutputFormat::Pnm),
        "tga" => Some(OutputFormat::Tga),
        "exr" => Some(OutputFormat::OpenExr),
        "ff" => Some(OutputFormat::Farbfeld),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum OutputBlpVersion {
    Blp0,
    Blp1,
    Blp2,
}

impl From<OutputBlpVersion> for BlpVersion {
    fn from(value: OutputBlpVersion) -> Self {
        match value {
            OutputBlpVersion::Blp0 => BlpVersion::Blp0,
            OutputBlpVersion::Blp1 => BlpVersion::Blp1,
            OutputBlpVersion::Blp2 => BlpVersion::Blp2,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum OutputBlpFormat {
    Raw1,
    Raw3,
    Jpeg,
    Dxt1,
    Dxt3,
    Dxt5,
}

// That is safe-plug to make compiler happy. BLP format is processed adhoc.
#[derive(Debug)]
pub struct UnsupportedFormat(OutputFormat);

impl TryFrom<OutputFormat> for ImageFormat {
    type Error = UnsupportedFormat;

    fn try_from(val: OutputFormat) -> Result<ImageFormat, Self::Error> {
        match val {
            OutputFormat::Blp => Err(UnsupportedFormat(val)),
            OutputFormat::Png => Ok(ImageFormat::Png),
            OutputFormat::Jpeg => Ok(ImageFormat::Jpeg),
            OutputFormat::Gif => Ok(ImageFormat::Gif),
            OutputFormat::Bmp => Ok(ImageFormat::Bmp),
            OutputFormat::Ico => Ok(ImageFormat::Ico),
            OutputFormat::Tiff => Ok(ImageFormat::Tiff),
            OutputFormat::Pnm => Ok(ImageFormat::Pnm),
            OutputFormat::Tga => Ok(ImageFormat::Tga),
            OutputFormat::OpenExr => Ok(ImageFormat::OpenExr),
            OutputFormat::Farbfeld => Ok(ImageFormat::Farbfeld),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum MipmapFilter {
    /// Nearest Neighbor
    Nearest,
    /// Linear Filter
    Triangle,
    /// Cubic Filter
    CatmullRom,
    /// Gaussian Filter
    Gaussian,
    /// Lanczos with window 3
    Lanczos3,
}

impl From<MipmapFilter> for FilterType {
    fn from(value: MipmapFilter) -> FilterType {
        match value {
            MipmapFilter::Nearest => FilterType::Nearest,
            MipmapFilter::Triangle => FilterType::Triangle,
            MipmapFilter::CatmullRom => FilterType::CatmullRom,
            MipmapFilter::Gaussian => FilterType::Gaussian,
            MipmapFilter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

/// Conversion of Warcraft III BLP format
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file we convert from. Could be BLP if we convert from BLP
    /// or other image format when convert into BLP. Format is guessed
    /// from the extension or can be explicitly specified by --input-format
    #[clap(value_parser)]
    input_file: PathBuf,

    /// The file we place the result. Could be BLP if we convert to BLP
    /// or other image format when convert from BLP. Format is guessed
    /// from the extension or can be explicitly specified by --output-format
    #[clap(value_parser)]
    output_file: PathBuf,

    /// Format to use for input file. If not set, it is guessed from extension.
    #[clap(short, long, value_parser)]
    input_format: Option<InputFormat>,

    /// Format to use for output file. If not set, it is guessed from extension.
    #[clap(short, long, value_parser)]
    output_format: Option<OutputFormat>,

    /// Which version to use when encoding to BLP format
    #[clap(long, value_parser, default_value = "blp1")]
    blp_version: OutputBlpVersion,

    /// Which sub format to use when encoding to BLP. Note that BLP1, for instance,
    /// doesn't support dxtn compression.
    #[clap(long, value_parser, default_value = "jpeg")]
    blp_format: OutputBlpFormat,

    /// Which amount of alpha bits to use when encoding to BLP. Note that not all
    /// combination with formats are legit. Jpeg supports only 0 and 8 bits.
    /// Raw1 supports 0, 1, 4, 8 bits. Raw3 is always 8 bits.
    #[clap(long, default_value = "8")]
    alpha_bits: u8,

    /// Which level of mipmap to use from the BLP file. 0 is default root image.
    #[clap(long, default_value = "0")]
    mipmap_level: usize,

    /// Whether to skip generation of mipmaps for output BLP file
    #[clap(long)]
    no_mipmaps: bool,

    /// Which algorithm to use to scale mipmaps down.
    #[clap(long, value_parser, default_value = "gaussian")]
    mipmap_filter: MipmapFilter,
}

fn make_target_blp_format(args: &Args) -> Result<BlpTarget, Error> {
    match args.blp_version {
        OutputBlpVersion::Blp0 => match args.blp_format {
            OutputBlpFormat::Raw1 => {
                let alpha_bits = match args.alpha_bits {
                    0 => AlphaBits::NoAlpha,
                    1 => AlphaBits::Bit1,
                    4 => AlphaBits::Bit4,
                    8 => AlphaBits::Bit8,
                    _ => {
                        return Err(Error::InvalidAlphaBits(
                            args.blp_version,
                            args.blp_format,
                            args.alpha_bits,
                        ))
                    }
                };
                Ok(BlpTarget::Blp0(BlpOldFormat::Raw1 { alpha_bits }))
            }
            OutputBlpFormat::Jpeg => {
                let has_alpha = match args.alpha_bits {
                    0 => false,
                    8 => true,
                    _ => {
                        return Err(Error::InvalidAlphaBits(
                            args.blp_version,
                            args.blp_format,
                            args.alpha_bits,
                        ))
                    }
                };
                Ok(BlpTarget::Blp0(BlpOldFormat::Jpeg { has_alpha }))
            }
            _ => Err(Error::BlpOldSupport(args.blp_version, args.blp_format)),
        },
        OutputBlpVersion::Blp1 => match args.blp_format {
            OutputBlpFormat::Raw1 => {
                let alpha_bits = match args.alpha_bits {
                    0 => AlphaBits::NoAlpha,
                    1 => AlphaBits::Bit1,
                    4 => AlphaBits::Bit4,
                    8 => AlphaBits::Bit8,
                    _ => {
                        return Err(Error::InvalidAlphaBits(
                            args.blp_version,
                            args.blp_format,
                            args.alpha_bits,
                        ))
                    }
                };
                Ok(BlpTarget::Blp1(BlpOldFormat::Raw1 { alpha_bits }))
            }
            OutputBlpFormat::Jpeg => {
                let has_alpha = match args.alpha_bits {
                    0 => false,
                    8 => true,
                    _ => {
                        return Err(Error::InvalidAlphaBits(
                            args.blp_version,
                            args.blp_format,
                            args.alpha_bits,
                        ))
                    }
                };
                Ok(BlpTarget::Blp1(BlpOldFormat::Jpeg { has_alpha }))
            }
            _ => Err(Error::BlpOldSupport(args.blp_version, args.blp_format)),
        },
        OutputBlpVersion::Blp2 => match args.blp_format {
            OutputBlpFormat::Raw1 => {
                let alpha_bits = match args.alpha_bits {
                    0 => AlphaBits::NoAlpha,
                    1 => AlphaBits::Bit1,
                    4 => AlphaBits::Bit4,
                    8 => AlphaBits::Bit8,
                    _ => {
                        return Err(Error::InvalidAlphaBits(
                            args.blp_version,
                            args.blp_format,
                            args.alpha_bits,
                        ))
                    }
                };
                Ok(BlpTarget::Blp2(Blp2Format::Raw1 { alpha_bits }))
            }
            OutputBlpFormat::Raw3 => Ok(BlpTarget::Blp2(Blp2Format::Raw3)),
            OutputBlpFormat::Jpeg => {
                let has_alpha = match args.alpha_bits {
                    0 => false,
                    8 => true,
                    _ => {
                        return Err(Error::InvalidAlphaBits(
                            args.blp_version,
                            args.blp_format,
                            args.alpha_bits,
                        ))
                    }
                };
                Ok(BlpTarget::Blp2(Blp2Format::Jpeg { has_alpha }))
            }
            OutputBlpFormat::Dxt1 => Ok(BlpTarget::Blp2(Blp2Format::Dxt1)),
            OutputBlpFormat::Dxt3 => Ok(BlpTarget::Blp2(Blp2Format::Dxt3)),
            OutputBlpFormat::Dxt5 => Ok(BlpTarget::Blp2(Blp2Format::Dxt5)),
        },
    }
}

fn run_conv() -> Result<(), Error> {
    env_logger::init();
    let args = Args::parse();

    // Collect all info about input format
    let input_format_opt = if args.input_format.is_none() {
        args.input_file
            .extension()
            .and_then(|e| e.to_str())
            .and_then(guess_input_format)
    } else {
        args.input_format
    };

    // Collect all info about output format
    let output_format_opt = if args.output_format.is_none() {
        args.output_file
            .extension()
            .and_then(|e| e.to_str())
            .and_then(guess_output_format)
    } else {
        args.output_format
    };
    let output_format = match output_format_opt {
        None => return Err(Error::MissingOutputFormat),
        Some(fmt) => fmt,
    };
    let input_format = match input_format_opt {
        None => return Err(Error::MissingInputFormat),
        Some(fmt) => fmt,
    };

    trace!("Reading input image");
    let input_image = if input_format == InputFormat::Blp {
        let blp_image = image_blp::parser::load_blp(&args.input_file)
            .map_err(|e| Error::LoadError(args.input_file.clone(), e))?;

        image_blp::convert::blp_to_image(&blp_image, args.mipmap_level)
            .map_err(|e| Error::Convert(args.input_file.clone(), e))?
    } else {
        ImageReader::open(&args.input_file)
            .map_err(|e| Error::ImageOpenError(args.input_file.clone(), e))?
            .decode()
            .map_err(|e| Error::ImageDecodeError(args.input_file.clone(), e))?
    };

    match output_format {
        OutputFormat::Blp => {
            let target = make_target_blp_format(&args)?;
            let new_blp = image_to_blp(
                input_image,
                !args.no_mipmaps,
                target,
                args.mipmap_filter.into(),
            )
            .map_err(|e| Error::Convert(args.input_file.clone(), e))?;
            save_blp(&new_blp, &args.output_file).map_err(|e| {
                Error::BlpSaveError(args.input_file.clone(), args.output_file.clone(), e)
            })?;
        }
        _ => {
            let img_format = output_format
                .try_into()
                .expect("impossible execution branch");
            input_image
                .save_with_format(&args.output_file, img_format)
                .map_err(|e| {
                    Error::SaveError(args.input_file.clone(), args.output_file.clone(), e)
                })?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run_conv() {
        println!("{}", e);
        std::process::exit(1);
    }
}

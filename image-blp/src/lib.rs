//! This crate provides decoding/encoding for Blizzard BLP texture
//! format that is used across several games like Wacraft III and
//! World of Warcraft. You can load any BLP file into [DynamicImage]
//! from [image] crate and save any [DynamicImage] into BLP file.
//!
//! # Usage
//! This crate is on crates.io and can be used by adding image-blp to
//! your dependencies in your project's Cargo.toml.
//!
//! ``` toml
//! [dependencies]
//! image-blp = "1"
//! ```
//!
//! # Example: loading
//!
//! The crate separates loading of BLP file into parsing and converting.
//! That allows to process BLP files without loosing information due unessesary
//! conversion back and forth. Typically loading of BLP image into usable RGBA
//! image looks like:
//!
//! ```no_run
//! # use ::image::DynamicImage;
//! # use image_blp::{convert::blp_to_image, parser::load_blp};
//! #
//! # let blp_filename = "test.blp";
//! # let output_filename = "output.png";
//! let blp_file = load_blp(blp_filename).expect("loaded blp");
//! let mipmap_level = 0;
//! let image = blp_to_image(&blp_file, mipmap_level).expect("converted");
//! ```
//! See example `examples/load.rs` for full code.
//!
//! # Example: saving
//!
//! The crate provides simplified API for specifing which type of BLP do you want to use.
//! See [convert::BlpTarget] type for more info. Here the typical way to save image as
//! ```no_run
//! # use ::image::{io::Reader, DynamicImage};
//! # use image_blp::{
//! #     convert::{image_to_blp, AlphaBits, BlpOldFormat, BlpTarget, FilterType},
//! #     encode::save_blp,
//! # };
//! # let input_filename = "input.png";
//! # let output_filename = "output.blp";
//! let img_file: DynamicImage = Reader::open(input_filename)
//!     .expect("open")
//!     .decode()
//!     .expect("decode");
//! let make_mipmaps = true;
//! let blp = image_to_blp(
//!     img_file,
//!     make_mipmaps,
//!     BlpTarget::Blp1(BlpOldFormat::Raw1 {
//!         alpha_bits: AlphaBits::Bit1,
//!     }),
//!     FilterType::Nearest,
//! )
//! .expect("converted");
//! save_blp(&blp, output_filename).expect("saved");
//! ```  
//! See example `examples/save.rs` for full code.
//!
//! # CLI tool
//!
//! The library is used to build universal CLI tool [blp-conv] that allows
//! to convert from/into BLP format for wide range of image formats. You
//! can install it via:
//!
//! ```bash
//! cargo install blp-conv
//! ```
//!
//! # Features
//!
//! The crate supports all known BLP versions like:
//! * `BLP0` -- is used in old Wacraft III ROC Beta builds.
//! * `BLP1` -- is common for Wacraft III TFT.
//! * `BLP2` -- is used in World of Warcraft.
//!
//! The crate supports also all known encodings:
//! * `RAW1` -- paletted images with 256 colors. We use [color_quant]
//! package for compressing generic images to the format.
//! * `RAW3` -- like ordinary RGBA bitmaps.
//! * `JPEG` -- ordinary jpeg compressed image.
//! * `DXTn` -- [S3TC] compression algorithms for `BLP2` version. We use
//! [texpresso] for the compression/decompression.
//!
//! # Tests
//!
//! Tests of the library use original files of Blizzard games. So, they cannot
//! be distributed with the library due license issues. You should buy the
//! original games to extract the files for testing.
//!
//! [image]: https://crates.io/crates/image
//! [blp-conv]: https://crates.io/crates/blp-conv
//! [DynamicImage]: https://docs.rs/image/latest/image/enum.DynamicImage.html
//! [color_quant]: https://crates.io/crates/color_quant
//! [texpresso]: https://crates.io/crates/texpresso
//! [S3TC]: http://en.wikipedia.org/wiki/S3TC

/// Convertion utilities to/from [DynamicImage](https://docs.rs/image/latest/image/enum.DynamicImage.html)
pub mod convert;
/// Encoding BLP format into stream of bytes.
pub mod encode;
/// Decoding BLP format from raw bytes.
pub mod parser;
/// Utilities for mipmaps filename generation
pub mod path;
/// Defines structure of parsed BLP file
pub mod types;

pub use types::*;

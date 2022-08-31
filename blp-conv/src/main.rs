use clap::{Parser, ValueEnum};
use std::path::PathBuf;

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
   Avif,
   Pnm,
   Dds,
   Tga,
   OpenEXR,
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
      "avif" => Some(InputFormat::Avif),
      "pnm" => Some(InputFormat::Pnm),
      "dds" => Some(InputFormat::Dds),
      "tga" => Some(InputFormat::Tga),
      "exr" => Some(InputFormat::OpenEXR),
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
   Avif,
   Pnm,
   Tga,
   OpenEXR,
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
      "avif" => Some(OutputFormat::Avif),
      "pnm" => Some(OutputFormat::Pnm),
      "tga" => Some(OutputFormat::Tga),
      "exr" => Some(OutputFormat::OpenEXR),
      "ff" => Some(OutputFormat::Farbfeld),
      _ => None,
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
}

fn main() {
   env_logger::init();
   let args = Args::parse();

   // Collect all info about input format
   let input_format = if args.input_format.is_none() {
      args
         .input_file
         .extension()
         .and_then(|e| e.to_str())
         .and_then(|e| guess_input_format(e))
   } else {
      args.input_format
   };

   // Collect all info about output format
   let output_format_opt = if args.output_format.is_none() {
      args
         .output_file
         .extension()
         .and_then(|e| e.to_str())
         .and_then(|e| guess_output_format(e))
   } else {
      args.output_format
   };
   let output_format = match output_format_opt {
      None => panic!("Failed to determine output format, please specify it via extension or by direct --output-format option"),
      Some(fmt) => fmt,
   };

}

use nom::error::{ContextError, ErrorKind, ParseError};
use std::fmt;
use thiserror::Error;

/// Errors that appears when loading from filesystem
#[derive(Debug, Error)]
pub enum LoadError {
    #[error("{0}")]
    Parsing(String),
    #[error("File system error with file {0}, due: {1}")]
    FileSystem(std::path::PathBuf, std::io::Error),
    #[error("Input stream is incomplete, needed: {0:?}")]
    Incomplete(nom::Needed),
    #[error("Cannot derive mipmap name for {0}")]
    InvalidFilename(std::path::PathBuf),
}

/// Errors that BLP parser can produce
#[derive(Debug, Error)]
pub enum Error<I: fmt::Debug> {
    #[error("Unexpected magic value {0}. The file format is not BLP or not supported.")]
    WrongMagic(String),
    #[error("Failed to extract external mipmap number {0} with error {1}")]
    ExternalMipmap(usize, Box<dyn std::error::Error>),
    #[error("There is no body of image for BLP0 mipmap number {0}")]
    MissingImage(usize),
    #[error("Part of image exceeds bounds of file for mipmap number {0}")]
    OutOfBounds(usize),
    #[error("BLP2 doesn't support external mipmaps")]
    Blp2NoExternalMips,
    #[error("Library doesn't support compression tag: {0}")]
    Blp2UnknownCompression(u8),
    #[error("Library doesn't support alpha type: {0}")]
    Blp2UnknownAlphaType(u8),
    #[error("Impossible branch, JPEG compression but direct content type")]
    Blp2UnexpectedJpegCompression,
    #[error("Error {1:?} at: {0:?}")]
    Nom(I, ErrorKind),
    #[error("Context: {0}. Error: {1}")]
    Context(String, Box<Self>),
}

impl<'a> From<(&'a [u8], ErrorKind)> for Error<&'a [u8]> {
    fn from((input, kind): (&'a [u8], ErrorKind)) -> Self {
        Error::Nom(input, kind)
    }
}

impl<'a> ParseError<&'a [u8]> for Error<&'a [u8]> {
    fn from_error_kind(input: &'a [u8], kind: ErrorKind) -> Self {
        Error::Nom(input, kind)
    }

    fn append(_: &[u8], _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> ContextError<&'a [u8]> for Error<&'a [u8]> {
    fn add_context(_input: &'a [u8], ctx: &'static str, other: Self) -> Self {
        Error::Context(ctx.to_owned(), Box::new(other))
    }
}

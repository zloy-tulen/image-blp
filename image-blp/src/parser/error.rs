use nom::{error::{ParseError, ErrorKind}};
use thiserror::Error;
use std::fmt;

/// Errors that BLP parser can produce
#[derive(Debug, Error)]
pub enum Error<I: fmt::Debug> {
    #[error("Unexpected magic value ${0}. The file format is not BLP or not supported.")]
    WrongMagic(String),
    #[error("Failed to extract external mipmap number ${0} with error ${1}")]
    ExternalMipmap(u32, Box<dyn std::error::Error>),
    #[error("There is no body of image for BLP0 mipmap number ${0}")]
    MissingImage(u32),
    #[error("Part of image exceeds bounds of file for mipmap number ${0}")]
    OutOfBounds(u32),
    #[error("Error ${1:?} at: ${0:?}")]
    Nom(I, ErrorKind),
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

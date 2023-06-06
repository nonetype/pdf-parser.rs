use std::{
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
};

use nom::{error::ErrorKind, IResult};

pub type ParseResult<'a, T> = IResult<&'a [u8], T, ParseError>;

// TODO: Extend this error type to include more information about the error.
// Currently, most of the errors are just "ParseError::NomError(ErrorKind)".
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParseError {
    // #[error("IO error: {0:?}")]
    // IOError(#[from] std::io::Error),
    #[error("UTF-8 error")]
    UTF8Error(#[from] Utf8Error),
    #[error("ParseInt error: {0:?}")]
    ParseIntError(#[from] ParseIntError),
    #[error("ParseFloatError error: {0:?}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Invalid PDF file")]
    InvalidPDF,
    #[error("Invalid PDF version")]
    InvalidPDFVersion,
    #[error("Invalid PDF header")]
    InvalidPDFHeader,
    #[error("Invalid PDF body")]
    InvalidPDFBody,
    #[error("Invalid PDF trailer")]
    InvalidPDFTrailer,
    #[error("Invalid PDF cross reference table")]
    InvalidPDFXrefTable,
    #[error("Invalid PDF cross reference stream")]
    InvalidPDFXrefStream,
    #[error("Invalid PDF cross reference entry: in use flag")]
    InvalidPDFXrefEntryInUseFlag,
    #[error("Invalid PDF object")]
    InvalidPDFObject,
    #[error("Invalid PDF object stream")]
    InvalidPDFObjectStream,
    #[error("Invalid PDF object stream dictionary")]
    InvalidPDFObjectStreamDictionary,
    #[error("Invalid PDF object stream data")]
    InvalidPDFObjectStreamData,
    #[error("Invalid PDF object stream data length")]
    InvalidPDFObjectStreamDataLength,
    #[error("Nom Parse error: {0:?}")]
    NomError(ErrorKind),
}

impl From<ParseError> for nom::Err<ParseError> {
    fn from(e: ParseError) -> Self {
        nom::Err::Error(e)
    }
}

// impl From<ParseError> for std::io::Error {
//     fn from(err: ParseError) -> Self {
//         Self::new(std::io::ErrorKind::Other, err)
//     }
// }

impl From<ErrorKind> for ParseError {
    fn from(e: ErrorKind) -> Self {
        ParseError::NomError(e)
    }
}

impl<I> nom::error::ParseError<I> for ParseError {
    fn from_error_kind(_input: I, kind: ErrorKind) -> Self {
        Self::NomError(kind)
    }
    fn append(_input: I, kind: ErrorKind, _other: Self) -> Self {
        Self::NomError(kind)
    }
}

// impl<'a, T> nom::error::FromExternalError<T, Utf8Error> for ParseError {
//     fn from_external_error(_: T, _: ErrorKind, e: Utf8Error) -> Self {
//         Self::UTF8Error(e)
//     }
// }

// impl<T> nom::error::FromExternalError<T, ParseIntError> for ParseError {
//     fn from_external_error(_: T, _: ErrorKind, e: ParseIntError) -> Self {
//         Self::ParseIntError(e)
//     }
// }

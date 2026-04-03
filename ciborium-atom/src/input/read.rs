// SPDX-License-Identifier: Apache-2.0

//! Input implementation for [`std::io::Read`] types.

#![cfg(feature = "std")]

extern crate std;

use std::io::{self, Read};
use std::string::String;
use std::vec;

use flex::Flex;

use super::Input;

/// Error type for the [`Reader`] input implementation.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(io::Error),

    /// Invalid UTF-8 in text string.
    Utf8(std::string::FromUtf8Error),
}

impl From<io::Error> for Error {
    #[inline]
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    #[inline]
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Utf8(e) => write!(f, "invalid UTF-8: {e}"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Utf8(e) => Some(e),
        }
    }
}

/// Wrapper that implements [`Input`] for any [`Read`] type.
pub struct Reader<R>(R);

impl<R> From<R> for Reader<R> {
    #[inline]
    fn from(inner: R) -> Self {
        Self(inner)
    }
}

impl<R: Read> Input<'static> for Reader<R> {
    type Error = Error;

    fn head(&mut self) -> Result<Option<u8>, Self::Error> {
        let mut buf = [0u8; 1];
        match self.0.read(&mut buf) {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(buf[0])),
            Err(e) => Err(e.into()),
        }
    }

    fn body<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        let mut buf = [0u8; N];
        self.0.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn tail(&mut self, len: usize) -> Result<Flex<'static, [u8]>, Self::Error> {
        let mut buf = vec![0u8; len];
        self.0.read_exact(&mut buf)?;
        Ok(Flex::Give(buf.into()))
    }

    fn text(&mut self, len: usize) -> Result<Flex<'static, str>, Self::Error> {
        let mut buf = vec![0u8; len];
        self.0.read_exact(&mut buf)?;
        let s = String::from_utf8(buf)?;
        Ok(Flex::Give(s.into()))
    }
}

// SPDX-License-Identifier: Apache-2.0

//! Input implementation for byte slices.

use flex::Flex;

use super::Input;

/// Error type for the `&[u8]` input implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Not enough bytes remaining.
    Underflow,

    /// Invalid UTF-8 in text string.
    Utf8(core::str::Utf8Error),
}

impl From<core::str::Utf8Error> for Error {
    #[inline]
    fn from(e: core::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Underflow => write!(f, "not enough bytes"),
            Self::Utf8(e) => write!(f, "invalid UTF-8: {e}"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Utf8(e) => Some(e),
            _ => None,
        }
    }
}

impl<'a> Input<'a> for &'a [u8] {
    type Error = Error;

    fn head(&mut self) -> Result<Option<u8>, Self::Error> {
        let Some((first, rest)) = self.split_first() else {
            return Ok(None);
        };

        let b = *first;
        *self = rest;
        Ok(Some(b))
    }

    fn body<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        if self.len() < N {
            return Err(Error::Underflow);
        }

        let (head, rest) = self.split_at(N);
        let mut buf = [0u8; N];
        buf.copy_from_slice(head);
        *self = rest;
        Ok(buf)
    }

    fn tail(&mut self, len: usize) -> Result<Flex<'a, [u8]>, Self::Error> {
        if self.len() < len {
            return Err(Error::Underflow);
        }

        let (data, rest) = self.split_at(len);
        *self = rest;
        Ok(Flex::Lend(data))
    }

    fn text(&mut self, len: usize) -> Result<Flex<'a, str>, Self::Error> {
        if self.len() < len {
            return Err(Error::Underflow);
        }

        let (data, rest) = self.split_at(len);
        let s = core::str::from_utf8(data)?;
        *self = rest;
        Ok(Flex::Lend(s))
    }
}

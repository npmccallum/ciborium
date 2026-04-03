// SPDX-License-Identifier: Apache-2.0

//! Input sources for CBOR decoding.

pub mod read;
pub mod slice;
pub mod span;

use flex::Flex;

/// A source of CBOR bytes for decoding.
pub trait Input<'a> {
    /// The error type for this input source.
    type Error;

    /// Read the first byte of an atom. Returns `None` at end of stream.
    fn head(&mut self) -> Result<Option<u8>, Self::Error>;

    /// Read exactly `N` argument bytes.
    fn body<const N: usize>(&mut self) -> Result<[u8; N], Self::Error>;

    /// Read `len` bytes as a byte string.
    fn tail(&mut self, len: usize) -> Result<Flex<'a, [u8]>, Self::Error>;

    /// Read `len` bytes as a UTF-8 text string.
    fn text(&mut self, len: usize) -> Result<Flex<'a, str>, Self::Error>;
}

impl<'a, T: Input<'a>> Input<'a> for &mut T {
    type Error = T::Error;

    #[inline]
    fn head(&mut self) -> Result<Option<u8>, Self::Error> {
        (**self).head()
    }

    #[inline]
    fn body<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        (**self).body()
    }

    #[inline]
    fn tail(&mut self, len: usize) -> Result<Flex<'a, [u8]>, Self::Error> {
        (**self).tail(len)
    }

    #[inline]
    fn text(&mut self, len: usize) -> Result<Flex<'a, str>, Self::Error> {
        (**self).text(len)
    }
}

// SPDX-License-Identifier: Apache-2.0

//! Position-tracking wrapper for any [`Input`].

use core::ops::Range;

use flex::Flex;

use super::Input;

/// Wrapper that tracks byte position over an inner [`Input`].
///
/// The `span()` method returns a range from the start of the current
/// atom (set by `head()`) to the current read position.
pub struct Span<T> {
    inner: T,
    item: usize,
    read: usize,
}

impl<T> From<T> for Span<T> {
    #[inline]
    fn from(inner: T) -> Self {
        Self {
            inner,
            item: 0,
            read: 0,
        }
    }
}

impl<T> Span<T> {
    /// Returns the byte range from the start of the current atom to
    /// the current read position.
    #[inline]
    pub fn span(&self) -> Range<usize> {
        self.item..self.read
    }
}

impl<'a, T: Input<'a>> Input<'a> for Span<T> {
    type Error = T::Error;

    fn head(&mut self) -> Result<Option<u8>, Self::Error> {
        Ok(self.inner.head()?.inspect(|_| {
            self.item = self.read;
            self.read = self.read.saturating_add(1);
        }))
    }

    fn body<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        let result = self.inner.body()?;
        self.read = self.read.saturating_add(N);
        Ok(result)
    }

    fn tail(&mut self, len: usize) -> Result<Flex<'a, [u8]>, Self::Error> {
        let result = self.inner.tail(len)?;
        self.read = self.read.saturating_add(len);
        Ok(result)
    }

    fn text(&mut self, len: usize) -> Result<Flex<'a, str>, Self::Error> {
        let result = self.inner.text(len)?;
        self.read = self.read.saturating_add(len);
        Ok(result)
    }
}

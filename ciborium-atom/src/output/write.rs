// SPDX-License-Identifier: Apache-2.0

//! Output implementation for [`std::io::Write`] types.

#![cfg(feature = "std")]

extern crate std;

use std::io;

use super::Output;

/// Wrapper that implements [`Output`] for any [`io::Write`] type.
pub struct Writer<W>(W);

impl<W> From<W> for Writer<W> {
    #[inline]
    fn from(inner: W) -> Self {
        Self(inner)
    }
}

impl<W: io::Write> Output for Writer<W> {
    type Error = io::Error;

    fn write(&mut self, head: u8, body: &[u8], tail: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all(&[head])?;
        self.0.write_all(body)?;
        self.0.write_all(tail)
    }
}

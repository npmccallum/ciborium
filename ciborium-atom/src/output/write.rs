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
        // Buffer the head + body (max 9 bytes) so the initial byte
        // and argument are written in a single write_all call.
        let mut buf = [0u8; 9];
        let (h, rest) = buf.split_first_mut().ok_or(io::ErrorKind::Other)?;
        *h = head;
        let dst = rest.get_mut(..body.len()).ok_or(io::ErrorKind::Other)?;
        dst.copy_from_slice(body);

        let hb = buf.get(..body.len().saturating_add(1)).ok_or(io::ErrorKind::Other)?;
        self.0.write_all(hb)?;
        self.0.write_all(tail)
    }
}

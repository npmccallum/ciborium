// SPDX-License-Identifier: Apache-2.0

//! Output implementation for [`std::io::Write`] types.

#![cfg(feature = "std")]

extern crate std;

use std::io::{self, IoSlice, Write};

use super::Output;

/// Wrapper that implements [`Output`] for any [`Write`] type.
pub struct Writer<W>(W);

impl<W> From<W> for Writer<W> {
    #[inline]
    fn from(inner: W) -> Self {
        Self(inner)
    }
}

impl<W: Write> Output for Writer<W> {
    type Error = io::Error;

    fn write(&mut self, head: u8, body: &[u8], tail: &[u8]) -> Result<(), Self::Error> {
        let mut hb = [0u8; 9];
        let (h, rest) = hb.split_first_mut().ok_or(io::ErrorKind::Other)?;
        *h = head;
        let dst = rest.get_mut(..body.len()).ok_or(io::ErrorKind::Other)?;
        dst.copy_from_slice(body);
        let hb = hb.get(..body.len().saturating_add(1)).ok_or(io::ErrorKind::Other)?;

        let mut bufs = [IoSlice::new(hb), IoSlice::new(tail)];
        let total = hb.len().saturating_add(tail.len());
        let mut written = 0usize;

        while written < total {
            let n = self.0.write_vectored(&bufs)?;
            if n == 0 {
                return Err(io::ErrorKind::WriteZero.into());
            }
            written = written.saturating_add(n);
            bufs = [
                IoSlice::new(hb.get(hb.len().min(written)..).unwrap_or(&[])),
                IoSlice::new(tail.get(written.saturating_sub(hb.len())..).unwrap_or(&[])),
            ];
        }

        Ok(())
    }
}

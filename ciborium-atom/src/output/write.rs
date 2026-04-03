// SPDX-License-Identifier: Apache-2.0

//! Output implementation for [`std::io::Write`] types.

#![cfg(feature = "std")]

extern crate std;

use super::Output;

/// Wrapper that implements [`Output`] for any [`std::io::Write`] type.
pub struct Writer<W>(W);

impl<W> From<W> for Writer<W> {
    #[inline]
    fn from(inner: W) -> Self {
        Self(inner)
    }
}

impl<W: std::io::Write> Output for Writer<W> {
    type Error = std::io::Error;

    #[cfg(feature = "nightly")]
    fn write(&mut self, head: u8, body: &[u8], tail: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all_vectored(&mut [
            std::io::IoSlice::new(&[head]),
            std::io::IoSlice::new(body),
            std::io::IoSlice::new(tail),
        ])
    }

    #[cfg(not(feature = "nightly"))]
    fn write(&mut self, head: u8, body: &[u8], tail: &[u8]) -> Result<(), Self::Error> {
        let mut bufs = [
            std::io::IoSlice::new(core::slice::from_ref(&head)),
            std::io::IoSlice::new(body),
            std::io::IoSlice::new(tail),
        ];

        let total = 1usize.saturating_add(body.len()).saturating_add(tail.len());
        let mut written = 0usize;

        while written < total {
            let n = self.0.write_vectored(&bufs)?;
            if n == 0 {
                return Err(std::io::ErrorKind::WriteZero.into());
            }
            written = written.saturating_add(n);

            let body_skip = written.saturating_sub(1);
            let tail_skip = body_skip.saturating_sub(body.len());
            bufs = [
                std::io::IoSlice::new(&[]),
                std::io::IoSlice::new(body.get(body.len().min(body_skip)..).unwrap_or(&[])),
                std::io::IoSlice::new(tail.get(tail.len().min(tail_skip)..).unwrap_or(&[])),
            ];
        }

        Ok(())
    }
}

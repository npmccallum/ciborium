// SPDX-License-Identifier: Apache-2.0

//! Output implementation for mutable byte slices.

use super::Output;

/// Error type for the `&mut [u8]` output implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "out of space")
    }
}

impl core::error::Error for Error {}

impl Output for &mut [u8] {
    type Error = Error;

    fn write(&mut self, head: u8, body: &[u8], tail: &[u8]) -> Result<(), Self::Error> {
        let total = 1usize
            .checked_add(body.len())
            .and_then(|n| n.checked_add(tail.len()))
            .ok_or(Error)?;

        if self.len() < total {
            return Err(Error);
        }

        let buf = core::mem::take(self);
        let (dst, buf) = buf.split_at_mut(total);

        let (h, bt) = dst.split_first_mut().ok_or(Error)?;
        *h = head;
        let (b, t) = bt.split_at_mut(body.len());
        b.copy_from_slice(body);
        t.copy_from_slice(tail);

        *self = buf;
        Ok(())
    }
}

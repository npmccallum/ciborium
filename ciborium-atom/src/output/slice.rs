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
        let buf = core::mem::take(self);

        // Split off space for head byte.
        let (dst, buf) = buf.split_first_mut().ok_or(Error)?;
        *dst = head;

        // Split off space for body.
        if buf.len() < body.len() {
            return Err(Error);
        }
        let (dst, buf) = buf.split_at_mut(body.len());
        dst.copy_from_slice(body);

        // Split off space for tail.
        if buf.len() < tail.len() {
            return Err(Error);
        }
        let (dst, buf) = buf.split_at_mut(tail.len());
        dst.copy_from_slice(tail);

        *self = buf;
        Ok(())
    }
}

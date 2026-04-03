// SPDX-License-Identifier: Apache-2.0

//! Output sinks for CBOR encoding.

pub mod slice;
mod vec;
pub mod write;

/// A sink for encoded CBOR bytes.
///
/// Each call to [`write`](Output::write) emits one CBOR atom as three
/// contiguous regions: the initial byte, the argument bytes, and the
/// payload.
///
/// - `head` — the first byte (major type + additional info)
/// - `body` — the argument bytes (0, 1, 2, 4, or 8 bytes)
/// - `tail` — the payload (non-empty only for `Bytes` and `Text` atoms)
pub trait Output {
    /// The error type for this output sink.
    type Error;

    /// Write a single CBOR atom.
    fn write(&mut self, head: u8, body: &[u8], tail: &[u8]) -> Result<(), Self::Error>;
}

impl<T: Output> Output for &mut T {
    type Error = T::Error;

    #[inline]
    fn write(&mut self, head: u8, body: &[u8], tail: &[u8]) -> Result<(), Self::Error> {
        (**self).write(head, body, tail)
    }
}

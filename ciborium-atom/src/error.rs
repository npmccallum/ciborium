// SPDX-License-Identifier: Apache-2.0

use core::fmt;

/// Errors that can occur during decoding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error<E> {
    /// The input source produced an error.
    Input(E),

    /// Invalid CBOR: reserved additional info value (28-30).
    Invalid,

    /// The length value does not fit in `usize`.
    Overflow,
}

impl<E: fmt::Display> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(e) => write!(f, "input error: {e}"),
            Self::Invalid => write!(f, "invalid additional info"),
            Self::Overflow => write!(f, "length overflow"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Input(e) => Some(e),
            _ => None,
        }
    }
}

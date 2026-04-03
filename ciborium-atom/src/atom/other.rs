// SPDX-License-Identifier: Apache-2.0

use crate::output::Output;

use super::{Float, Head, Simple};

/// CBOR major type 7: simple values and floats.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Other {
    /// Simple value
    Simple(Simple),

    /// Floating-point value
    Float(Float),
}

impl Other {
    /// Encode this value as a CBOR head (major type 7).
    #[inline]
    pub(crate) fn encode(self) -> Head {
        match self {
            Self::Simple(s) => s.encode(),
            Self::Float(f) => f.encode(),
        }
    }

    /// Encode this value to an output (major type 7).
    #[inline]
    pub(crate) fn encode_to<O: Output>(self, output: &mut O) -> Result<(), O::Error> {
        match self {
            Self::Simple(s) => s.encode_to(output),
            Self::Float(f) => f.encode_to(output),
        }
    }
}

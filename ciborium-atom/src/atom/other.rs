// SPDX-License-Identifier: Apache-2.0

use crate::output::Output;

use super::{Float, Simple};

/// CBOR major type 7: simple values and floats.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Other {
    /// Simple value
    Simple(Simple),

    /// Floating-point value
    Float(Float),
}

impl Other {
    /// Encode this value to an output (major type 7).
    #[inline]
    pub(crate) fn encode<O: Output>(self, output: &mut O) -> Result<(), O::Error> {
        match self {
            Self::Simple(s) => s.encode(output),
            Self::Float(f) => f.encode(output),
        }
    }
}

// SPDX-License-Identifier: Apache-2.0

use crate::output::Output;

use super::short::Short;

/// A CBOR simple value preserving its wire size.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Simple {
    /// Simple value encoded inline (0 argument bytes, value 0-23)
    S0(Short),

    /// Simple value encoded with 1 argument byte
    S1(u8),
}

impl Simple {
    /// The "false" simple value (major type 7, additional info 20).
    pub const FALSE: Self = Self::S0(Short(20));

    /// The "true" simple value (major type 7, additional info 21).
    pub const TRUE: Self = Self::S0(Short(21));

    /// The "null" simple value (major type 7, additional info 22).
    pub const NULL: Self = Self::S0(Short(22));

    /// The "undefined" simple value (major type 7, additional info 23).
    pub const UNDEFINED: Self = Self::S0(Short(23));

    /// Encode this simple value to an output (major type 7).
    #[inline]
    pub(crate) fn encode<O: Output>(self, mut output: O) -> Result<(), O::Error> {
        let mt = 7 << 5;
        match self {
            Self::S0(v) => output.write(mt | v.get(), &[], &[]),
            Self::S1(v) => output.write(mt | 24, &[v], &[]),
        }
    }
}

// SPDX-License-Identifier: Apache-2.0

use super::short::Short;
use super::Head;

/// A CBOR simple value preserving its wire size.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Simple {
    /// Simple value encoded inline (0 argument bytes, value 0-23)
    S0(Short),

    /// Simple value encoded with 1 argument byte
    S1(u8),
}

impl Simple {
    /// Encode this simple value as a CBOR head (major type 7).
    #[inline]
    pub(crate) fn encode(self) -> Head {
        let mt = 7 << 5;
        match self {
            Self::S0(v) => Head::new0(mt | v.get()),
            Self::S1(v) => Head::new1(mt | 24, [v]),
        }
    }
}

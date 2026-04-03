// SPDX-License-Identifier: Apache-2.0

/// A value that fits in the CBOR additional info field (0-23).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Short(pub(crate) u8);

impl Short {
    /// Create a `Short` if the value is in range (0-23).
    #[inline]
    pub fn new(v: u8) -> Option<Self> {
        (v < 24).then_some(Self(v))
    }

    /// Return the inner value.
    #[inline]
    pub fn get(self) -> u8 {
        self.0
    }
}

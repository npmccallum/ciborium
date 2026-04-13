// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Sentinel name for CBOR simple value encoding.
pub(crate) const SENTINEL: &str = "@@CBOR_SIMPLE@@";

/// Represents a CBOR simple value (0-255).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Simple(pub u8);

impl Simple {
    /// The "false" simple value (20).
    pub const FALSE: Self = Self(20);

    /// The "true" simple value (21).
    pub const TRUE: Self = Self(21);

    /// The "null" simple value (22).
    pub const NULL: Self = Self(22);

    /// The "undefined" simple value (23).
    pub const UNDEFINED: Self = Self(23);
}

impl Serialize for Simple {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct(SENTINEL, &self.0)
    }
}

impl<'de> Deserialize<'de> for Simple {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct(SENTINEL, SimpleVisitor)
    }
}

struct SimpleVisitor;

impl<'de> serde::de::Visitor<'de> for SimpleVisitor {
    type Value = Simple;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("CBOR simple value")
    }

    fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Simple, E> {
        Ok(Simple(v))
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Simple, D::Error> {
        Ok(Simple(u8::deserialize(deserializer)?))
    }
}

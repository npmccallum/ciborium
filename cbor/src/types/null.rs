// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Sentinel name for CBOR null encoding.
pub(crate) const SENTINEL: &str = "@@CBOR_NULL@@";

/// Represents the CBOR null simple value (22).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl Serialize for Null {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct(SENTINEL, &())
    }
}

impl<'de> Deserialize<'de> for Null {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct(SENTINEL, NullVisitor)
    }
}

struct NullVisitor;

impl<'de> serde::de::Visitor<'de> for NullVisitor {
    type Value = Null;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("CBOR null")
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Null, E> {
        Ok(Null)
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(self, deserializer: D) -> Result<Null, D::Error> {
        <()>::deserialize(deserializer)?;
        Ok(Null)
    }
}

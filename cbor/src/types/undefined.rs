// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Sentinel name for CBOR undefined encoding.
pub(crate) const SENTINEL: &str = "@@CBOR_UNDEFINED@@";

/// Represents the CBOR undefined simple value (23).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Undefined;

impl Serialize for Undefined {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct(SENTINEL, &())
    }
}

impl<'de> Deserialize<'de> for Undefined {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_newtype_struct(SENTINEL, UndefinedVisitor)
    }
}

struct UndefinedVisitor;

impl<'de> serde::de::Visitor<'de> for UndefinedVisitor {
    type Value = Undefined;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("CBOR undefined")
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Undefined, E> {
        Ok(Undefined)
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Undefined, D::Error> {
        <()>::deserialize(deserializer)?;
        Ok(Undefined)
    }
}

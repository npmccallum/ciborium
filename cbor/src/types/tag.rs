// SPDX-License-Identifier: Apache-2.0

//! CBOR tag types.
//!
//! The tag protocol uses `serialize_tuple_struct("@@CBOR_TAG@@", 2)`
//! where the first field is `Option<u64>` (the tag number) and the
//! second field is the inner value.

use serde::ser::SerializeTupleStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Sentinel name for CBOR tag encoding.
pub(crate) const SENTINEL: &str = "@@CBOR_TAG@@";

macro_rules! tag_deserialize_seq {
    ($seq:ident, $require:expr, $exact:expr, $tag_val:expr) => {{
        let tag: Option<u64> = $seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::custom("missing tag"))?;

        if $require && tag.is_none() {
            return Err(serde::de::Error::custom("required tag not found"));
        }

        if $exact {
            if let Some(t) = tag {
                if t != $tag_val {
                    return Err(serde::de::Error::custom("wrong tag"));
                }
            }
        }

        let value = $seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::custom("missing value"))?;

        (tag, value)
    }};
}

// ---------------------------------------------------------------------------
// AllowAny
// ---------------------------------------------------------------------------

/// Allow any tag, if present.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AllowAny<V>(pub Option<u64>, pub V);

impl<V: Serialize> Serialize for AllowAny<V> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_tuple_struct(SENTINEL, 2)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.end()
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for AllowAny<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis<V>(core::marker::PhantomData<V>);
        impl<'de, V: Deserialize<'de>> serde::de::Visitor<'de> for Vis<V> {
            type Value = AllowAny<V>;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a CBOR tag")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let (tag, value) = tag_deserialize_seq!(seq, false, false, 0u64);
                Ok(AllowAny(tag, value))
            }
        }
        deserializer.deserialize_tuple_struct(SENTINEL, 2, Vis(core::marker::PhantomData))
    }
}

// ---------------------------------------------------------------------------
// AllowExact
// ---------------------------------------------------------------------------

/// Allow a specific tag, if present.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AllowExact<V, const TAG: u64>(pub V);

impl<V: Serialize, const TAG: u64> Serialize for AllowExact<V, TAG> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_tuple_struct(SENTINEL, 2)?;
        s.serialize_field(&Some(TAG))?;
        s.serialize_field(&self.0)?;
        s.end()
    }
}

impl<'de, V: Deserialize<'de>, const TAG: u64> Deserialize<'de> for AllowExact<V, TAG> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis<V, const TAG: u64>(core::marker::PhantomData<V>);
        impl<'de, V: Deserialize<'de>, const TAG: u64> serde::de::Visitor<'de> for Vis<V, TAG> {
            type Value = AllowExact<V, TAG>;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "CBOR tag {TAG} or untagged")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let (_tag, value) = tag_deserialize_seq!(seq, false, true, TAG);
                Ok(AllowExact(value))
            }
        }
        deserializer.deserialize_tuple_struct(SENTINEL, 2, Vis::<V, TAG>(core::marker::PhantomData))
    }
}

// ---------------------------------------------------------------------------
// RequireAny
// ---------------------------------------------------------------------------

/// Require any tag to be present.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequireAny<V>(pub u64, pub V);

impl<V: Serialize> Serialize for RequireAny<V> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_tuple_struct(SENTINEL, 2)?;
        s.serialize_field(&Some(self.0))?;
        s.serialize_field(&self.1)?;
        s.end()
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for RequireAny<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis<V>(core::marker::PhantomData<V>);
        impl<'de, V: Deserialize<'de>> serde::de::Visitor<'de> for Vis<V> {
            type Value = RequireAny<V>;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a required CBOR tag")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let (tag, value) = tag_deserialize_seq!(seq, true, false, 0u64);
                Ok(RequireAny(tag.unwrap(), value))
            }
        }
        deserializer.deserialize_tuple_struct(SENTINEL, 2, Vis(core::marker::PhantomData))
    }
}

// ---------------------------------------------------------------------------
// RequireExact
// ---------------------------------------------------------------------------

/// Require a specific tag to be present.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequireExact<V, const TAG: u64>(pub V);

impl<V: Serialize, const TAG: u64> Serialize for RequireExact<V, TAG> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_tuple_struct(SENTINEL, 2)?;
        s.serialize_field(&Some(TAG))?;
        s.serialize_field(&self.0)?;
        s.end()
    }
}

impl<'de, V: Deserialize<'de>, const TAG: u64> Deserialize<'de> for RequireExact<V, TAG> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis<V, const TAG: u64>(core::marker::PhantomData<V>);
        impl<'de, V: Deserialize<'de>, const TAG: u64> serde::de::Visitor<'de> for Vis<V, TAG> {
            type Value = RequireExact<V, TAG>;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "CBOR tag {TAG}")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let (_tag, value) = tag_deserialize_seq!(seq, true, true, TAG);
                Ok(RequireExact(value))
            }
        }
        deserializer.deserialize_tuple_struct(SENTINEL, 2, Vis::<V, TAG>(core::marker::PhantomData))
    }
}

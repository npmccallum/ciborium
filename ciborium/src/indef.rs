// SPDX-License-Identifier: Apache-2.0

//! Support for CBOR indefinite-length encoding.
//!
//! CBOR supports two encodings for arrays and maps: definite-length (where the
//! count is prepended) and indefinite-length (where a break marker follows the
//! last element). By default, ciborium uses definite-length encoding for all
//! collections. The [`Indefinite`] wrapper overrides this for the outermost
//! collection of the wrapped value.
//!
//! # Serialization
//!
//! Wrapping a value in `Indefinite` causes its outermost collection to be
//! serialized with an indefinite-length header. The effect is **non-hierarchical**:
//! only the immediate collection is affected, not any nested collections.
//!
//! ```
//! use ciborium::Indefinite;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Example {
//!     items: Indefinite<Vec<u32>>,
//! }
//! ```
//!
//! # Deserialization
//!
//! `Indefinite<T>` deserializes identically to `T`. CBOR decoders handle both
//! definite and indefinite encodings transparently, so the wrapper is a
//! pass-through on the deserialization side.
//!
//! # Non-collection types
//!
//! Wrapping a non-collection type (e.g., `Indefinite<u32>`) is harmless: it
//! serializes identically to the unwrapped type.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Wrapper that causes the outermost collection of `T` to be serialized
/// using CBOR indefinite-length encoding.
///
/// The effect is non-hierarchical: nested collections within `T` are
/// unaffected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Indefinite<T>(pub T);

impl<T> Indefinite<T> {
    /// The sentinel name used to signal indefinite-length encoding to ciborium's
    /// serializer via `serialize_newtype_struct`.
    pub(crate) const SENTINEL: &str = "@@CBOR_INDEFINITE@@";
}

impl<T: Serialize> Serialize for Indefinite<T> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct(Self::SENTINEL, &self.0)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Indefinite<T> {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Indefinite)
    }
}

impl<T> From<T> for Indefinite<T> {
    #[inline]
    fn from(value: T) -> Self {
        Indefinite(value)
    }
}

impl<T> core::ops::Deref for Indefinite<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Indefinite<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

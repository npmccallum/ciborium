// SPDX-License-Identifier: Apache-2.0

use core::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

/// Wrapper that causes the outermost collection of `T` to be serialized
/// using CBOR indefinite-length encoding.
///
/// The effect is non-hierarchical: nested collections within `T` are
/// unaffected. Wrapping a non-collection type is a harmless no-op.
///
/// Deserialization is transparent.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename = "@@CBOR_INDEFINITE@@")]
#[repr(transparent)]
pub struct Indefinite<T>(pub T);

impl<T> From<T> for Indefinite<T> {
    #[inline]
    fn from(value: T) -> Self {
        Indefinite(value)
    }
}

impl<T> Deref for Indefinite<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Indefinite<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

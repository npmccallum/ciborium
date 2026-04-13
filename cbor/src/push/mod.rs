// SPDX-License-Identifier: Apache-2.0

//! The Push trait for consuming CBOR atoms.

mod encoder;

pub use encoder::Encoder;

use ciborium_atom::Atom;

/// A consumer of CBOR atoms.
///
/// The serializer writes atoms to an implementation of this trait.
/// The simplest implementation encodes atoms to bytes via [`Output`](ciborium_atom::output::Output),
/// but implementations can also inspect, transform, or collect atoms.
pub trait Push {
    /// The error type.
    type Error;

    /// Consume a single CBOR atom.
    fn push(&mut self, atom: Atom<'_>) -> Result<(), Self::Error>;
}

impl<T: Push> Push for &mut T {
    type Error = T::Error;

    #[inline]
    fn push(&mut self, atom: Atom<'_>) -> Result<(), Self::Error> {
        (**self).push(atom)
    }
}

// SPDX-License-Identifier: Apache-2.0

//! The Pull trait for producing CBOR atoms.

mod decoder;

pub use decoder::Decoder;

use ciborium_atom::Atom;

/// A producer of CBOR atoms.
///
/// The deserializer reads atoms from an implementation of this trait.
/// The simplest implementation decodes atoms from bytes via [`Input`](ciborium_atom::input::Input),
/// but implementations can also generate atoms from other sources.
pub trait Pull<'a> {
    /// The error type.
    type Error;

    /// Produce the next CBOR atom, or `None` at end of input.
    fn pull(&mut self) -> Result<Option<Atom<'a>>, Self::Error>;
}

impl<'a, T: Pull<'a>> Pull<'a> for &mut T {
    type Error = T::Error;

    #[inline]
    fn pull(&mut self) -> Result<Option<Atom<'a>>, Self::Error> {
        (**self).pull()
    }
}

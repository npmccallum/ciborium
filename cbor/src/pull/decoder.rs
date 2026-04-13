// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::input::Input;
use ciborium_atom::Atom;

use super::Pull;

/// A [`Pull`] that decodes atoms from an [`Input`].
pub struct Decoder<I>(pub I);

impl<'a, I: Input<'a>> Pull<'a> for Decoder<I> {
    type Error = ciborium_atom::Error<I::Error>;

    #[inline]
    fn pull(&mut self) -> Result<Option<Atom<'a>>, Self::Error> {
        Atom::decode(&mut self.0)
    }
}

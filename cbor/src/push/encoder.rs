// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::output::Output;
use ciborium_atom::Atom;

use super::Push;

/// A [`Push`] that encodes atoms to an [`Output`].
pub struct Encoder<O>(pub O);

impl<O: Output> Push for Encoder<O> {
    type Error = O::Error;

    #[inline]
    fn push(&mut self, atom: Atom<'_>) -> Result<(), Self::Error> {
        atom.encode(&mut self.0)
    }
}

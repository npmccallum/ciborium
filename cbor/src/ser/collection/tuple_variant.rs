// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::Atom;
use serde::ser;

use super::super::{Error, Serializer};
use crate::push::Push;

pub struct TupleVariant<P> {
    push: P,
    inner_ending: bool,
    outer_ending: bool,
}

impl<P> TupleVariant<P> {
    pub(crate) fn new(push: P, inner_ending: bool, outer_ending: bool) -> Self {
        Self { push, inner_ending, outer_ending }
    }
}

impl<P: Push> ser::SerializeTupleVariant for TupleVariant<P>
where
    P::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<P::Error>;

    fn serialize_field<U: ?Sized + ser::Serialize>(&mut self, value: &U) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(&mut self.push))
    }

    fn end(mut self) -> Result<(), Self::Error> {
        if self.inner_ending { self.push.push(Atom::Other(None))?; }
        if self.outer_ending { self.push.push(Atom::Other(None))?; }
        Ok(())
    }
}

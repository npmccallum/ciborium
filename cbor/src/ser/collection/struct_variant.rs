// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::Atom;
use serde::ser;
use serde::Serialize as _;

use super::super::{Error, Serializer};
use crate::push::Push;

pub struct StructVariant<P> {
    push: P,
    inner_ending: bool,
    outer_ending: bool,
}

impl<P> StructVariant<P> {
    pub(crate) fn new(push: P, inner_ending: bool, outer_ending: bool) -> Self {
        Self { push, inner_ending, outer_ending }
    }
}

impl<P: Push> ser::SerializeStructVariant for StructVariant<P>
where
    P::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<P::Error>;

    fn serialize_field<U: ?Sized + ser::Serialize>(&mut self, key: &'static str, value: &U) -> Result<(), Self::Error> {
        key.serialize(Serializer::new(&mut self.push))?;
        value.serialize(Serializer::new(&mut self.push))
    }

    fn end(mut self) -> Result<(), Self::Error> {
        if self.inner_ending { self.push.push(Atom::Other(None))?; }
        if self.outer_ending { self.push.push(Atom::Other(None))?; }
        Ok(())
    }
}

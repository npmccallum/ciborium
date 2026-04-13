// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::Atom;
use serde::ser;

use super::super::{Error, Serializer};
use crate::push::Push;

pub struct Map<P> {
    push: P,
    ending: bool,
}

impl<P> Map<P> {
    pub(crate) fn new(push: P, ending: bool) -> Self {
        Self { push, ending }
    }
}

impl<P: Push> ser::SerializeMap for Map<P>
where
    P::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<P::Error>;

    fn serialize_key<U: ?Sized + ser::Serialize>(&mut self, key: &U) -> Result<(), Self::Error> {
        key.serialize(Serializer::new(&mut self.push))
    }

    fn serialize_value<U: ?Sized + ser::Serialize>(&mut self, value: &U) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(&mut self.push))
    }

    fn end(mut self) -> Result<(), Self::Error> {
        if self.ending { self.push.push(Atom::Other(None))?; }
        Ok(())
    }
}

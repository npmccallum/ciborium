// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::Atom;
use serde::ser;
use serde::Serialize as _;

use super::super::{Error, Serializer};
use crate::push::Push;

pub struct Struct<P> {
    push: P,
    ending: bool,
}

impl<P> Struct<P> {
    pub(crate) fn new(push: P, ending: bool) -> Self {
        Self { push, ending }
    }
}

impl<P: Push> ser::SerializeStruct for Struct<P>
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
        if self.ending { self.push.push(Atom::Other(None))?; }
        Ok(())
    }
}

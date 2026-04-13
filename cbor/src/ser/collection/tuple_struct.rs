// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::{Atom, Unsigned};
use serde::ser;

use super::super::{Error, Serializer, TagExtractor};
use crate::push::Push;

pub struct TupleStruct<P> {
    push: P,
    ending: bool,
    tag: bool,
}

impl<P> TupleStruct<P> {
    pub(crate) fn new(push: P, ending: bool) -> Self {
        Self {
            push,
            ending,
            tag: false,
        }
    }

    pub(crate) fn tag(push: P) -> Self {
        Self {
            push,
            ending: false,
            tag: true,
        }
    }
}

impl<P: Push> ser::SerializeTupleStruct for TupleStruct<P>
where
    P::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<P::Error>;

    fn serialize_field<U: ?Sized + ser::Serialize>(
        &mut self,
        value: &U,
    ) -> Result<(), Self::Error> {
        if self.tag {
            self.tag = false;
            match value.serialize(TagExtractor) {
                Ok(Some(t)) => {
                    self.push.push(Atom::Tag(Unsigned::from(t).shrink()))?;
                    Ok(())
                }
                Ok(None) => Ok(()),
                Err(_) => Err(ser::Error::custom("expected tag value")),
            }
        } else {
            value.serialize(Serializer::new(&mut self.push))
        }
    }

    fn end(mut self) -> Result<(), Self::Error> {
        if self.ending {
            self.push.push(Atom::Other(None))?;
        }
        Ok(())
    }
}

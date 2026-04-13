// SPDX-License-Identifier: Apache-2.0

//! Serde serialization support for CBOR.

pub(crate) mod collection;
mod error;
mod mode;

pub use error::Error;

use ciborium_atom::output::Output;
use ciborium_atom::{Atom, Float, Other, Simple, Unsigned};
use core::fmt::Debug;
use serde::Serialize as _;
use serde::ser;

use crate::push::{Encoder, Push};
use crate::{BIGNEG, BIGPOS};

use self::mode::Mode;

/// A CBOR serializer that writes atoms to a [`Push`] sink.
pub struct Serializer<P> {
    push: P,
    mode: Mode,
}

impl<P: Push> Serializer<P> {
    /// Create a new serializer writing to the given push sink.
    pub fn new(push: P) -> Self {
        Self {
            push,
            mode: Mode::default(),
        }
    }

    /// Consume the serializer and return the inner push sink.
    pub fn into_inner(self) -> P {
        self.push
    }
}

impl<P: Push> ser::Serializer for Serializer<P>
where
    P::Error: core::fmt::Debug,
{
    type Ok = ();
    type Error = Error<P::Error>;

    type SerializeSeq = collection::Seq<P>;
    type SerializeTuple = collection::Tuple<P>;
    type SerializeTupleStruct = collection::TupleStruct<P>;
    type SerializeTupleVariant = collection::TupleVariant<P>;
    type SerializeMap = collection::Map<P>;
    type SerializeStruct = collection::Struct<P>;
    type SerializeStructVariant = collection::StructVariant<P>;

    #[inline]
    fn serialize_bool(mut self, v: bool) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i8(mut self, v: i8) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_i16(mut self, v: i16) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_i32(mut self, v: i32) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_i64(mut self, v: i64) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_i128(mut self, v: i128) -> Result<(), Self::Error> {
        if let Ok(x) = i64::try_from(v) {
            return self.serialize_i64(x);
        }

        let (tag, raw) = if v.is_negative() {
            (BIGNEG, v as u128 ^ !0)
        } else {
            (BIGPOS, v as u128)
        };

        let bytes = raw.to_be_bytes();
        let start = raw.leading_zeros() as usize / 8;
        let slice = &bytes[start..];

        self.push.push(Atom::Tag(Unsigned::from(tag).shrink()))?;
        self.push.push(Atom::Bytes(Some(slice.into())))?;
        Ok(())
    }

    #[inline]
    fn serialize_u8(mut self, v: u8) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_u16(mut self, v: u16) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_u32(mut self, v: u32) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_u64(mut self, v: u64) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_u128(mut self, v: u128) -> Result<(), Self::Error> {
        if let Ok(x) = u64::try_from(v) {
            return self.serialize_u64(x);
        }

        let bytes = v.to_be_bytes();
        let start = v.leading_zeros() as usize / 8;
        let slice = &bytes[start..];

        self.push.push(Atom::Tag(Unsigned::from(BIGPOS).shrink()))?;
        self.push.push(Atom::Bytes(Some(slice.into())))?;
        Ok(())
    }

    #[inline]
    fn serialize_f32(mut self, v: f32) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_f64(mut self, v: f64) -> Result<(), Self::Error> {
        self.push.push(Atom::from(v).shrink())?;
        Ok(())
    }

    #[inline]
    fn serialize_char(mut self, v: char) -> Result<(), Self::Error> {
        let mut buf = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(mut self, v: &str) -> Result<(), Self::Error> {
        self.push.push(Atom::Text(Some(v.into())))?;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(mut self, v: &[u8]) -> Result<(), Self::Error> {
        self.push.push(Atom::Bytes(Some(v.into())))?;
        Ok(())
    }

    #[inline]
    fn serialize_none(mut self) -> Result<(), Self::Error> {
        self.push
            .push(Atom::Other(Some(Other::Simple(Simple::NULL))))?;
        Ok(())
    }

    #[inline]
    fn serialize_some<U: ?Sized + ser::Serialize>(self, value: &U) -> Result<(), Self::Error> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<(), Self::Error> {
        self.serialize_none()
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<U: ?Sized + ser::Serialize>(
        self,
        name: &'static str,
        value: &U,
    ) -> Result<(), Self::Error> {
        value.serialize(Serializer {
            push: self.push,
            mode: Mode::from_sentinel(name),
        })
    }

    #[inline]
    fn serialize_newtype_variant<U: ?Sized + ser::Serialize>(
        mut self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &U,
    ) -> Result<(), Self::Error> {
        let indefinite = self.mode == Mode::Indefinite;
        let outer = if indefinite {
            None
        } else {
            Some(Unsigned::from(1u64).shrink())
        };
        self.push.push(Atom::Map(outer))?;
        self.push.push(Atom::Text(Some(variant.into())))?;
        value.serialize(Serializer::new(&mut self.push))?;
        if indefinite {
            self.push.push(Atom::Other(None))?;
        }
        Ok(())
    }

    #[inline]
    fn serialize_seq(mut self, length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let length = match (self.mode, length) {
            (Mode::Default, Some(len)) => Some(Unsigned::from(len as u64).shrink()),
            _ => None,
        };

        self.push.push(Atom::Array(length))?;
        Ok(collection::Seq::new(self.push, length.is_none()))
    }

    #[inline]
    fn serialize_tuple(mut self, length: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let length = match self.mode {
            Mode::Default => Some(Unsigned::from(length as u64).shrink()),
            _ => None,
        };

        self.push.push(Atom::Array(length))?;
        Ok(collection::Tuple::new(self.push, length.is_none()))
    }

    #[inline]
    fn serialize_tuple_struct(
        mut self,
        name: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        if name == crate::types::tag::SENTINEL {
            return Ok(collection::TupleStruct::tag(self.push));
        }

        let length = match self.mode {
            Mode::Default => Some(Unsigned::from(length as u64).shrink()),
            _ => None,
        };

        self.push.push(Atom::Array(length))?;
        Ok(collection::TupleStruct::new(self.push, length.is_none()))
    }

    #[inline]
    fn serialize_tuple_variant(
        mut self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let (outer, inner) = match self.mode {
            Mode::Indefinite => (None, None),
            Mode::Default => (
                Some(Unsigned::from(1u64).shrink()),
                Some(Unsigned::from(length as u64).shrink()),
            ),
        };

        self.push.push(Atom::Map(outer))?;
        self.push.push(Atom::Text(Some(variant.into())))?;
        self.push.push(Atom::Array(inner))?;
        Ok(collection::TupleVariant::new(
            self.push,
            inner.is_none(),
            outer.is_none(),
        ))
    }

    #[inline]
    fn serialize_map(mut self, length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let length = match (self.mode, length) {
            (Mode::Default, Some(len)) => Some(Unsigned::from(len as u64).shrink()),
            _ => None,
        };

        self.push.push(Atom::Map(length))?;
        Ok(collection::Map::new(self.push, length.is_none()))
    }

    #[inline]
    fn serialize_struct(
        mut self,
        _name: &'static str,
        length: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let length = match self.mode {
            Mode::Default => Some(Unsigned::from(length as u64).shrink()),
            _ => None,
        };

        self.push.push(Atom::Map(length))?;
        Ok(collection::Struct::new(self.push, length.is_none()))
    }

    #[inline]
    fn serialize_struct_variant(
        mut self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let indefinite = self.mode == Mode::Indefinite;
        let outer = if indefinite {
            None
        } else {
            Some(Unsigned::from(1u64).shrink())
        };
        let inner = if indefinite {
            None
        } else {
            Some(Unsigned::from(length as u64).shrink())
        };

        self.push.push(Atom::Map(outer))?;
        self.push.push(Atom::Text(Some(variant.into())))?;
        self.push.push(Atom::Map(inner))?;
        Ok(collection::StructVariant::new(
            self.push,
            inner.is_none(),
            outer.is_none(),
        ))
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// TagExtractor: extracts Option<u64> from a serde Serialize value
// ---------------------------------------------------------------------------

pub(crate) struct TagExtractor;

#[derive(Debug)]
pub(crate) struct TagError;

impl core::fmt::Display for TagError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("tag extraction error")
    }
}

impl ser::StdError for TagError {}

impl ser::Error for TagError {
    fn custom<U: core::fmt::Display>(_msg: U) -> Self {
        TagError
    }
}

impl ser::Serializer for TagExtractor {
    type Ok = Option<u64>;
    type Error = TagError;

    type SerializeSeq = ser::Impossible<Option<u64>, TagError>;
    type SerializeTuple = ser::Impossible<Option<u64>, TagError>;
    type SerializeTupleStruct = ser::Impossible<Option<u64>, TagError>;
    type SerializeTupleVariant = ser::Impossible<Option<u64>, TagError>;
    type SerializeMap = ser::Impossible<Option<u64>, TagError>;
    type SerializeStruct = ser::Impossible<Option<u64>, TagError>;
    type SerializeStructVariant = ser::Impossible<Option<u64>, TagError>;

    fn serialize_none(mut self) -> Result<Option<u64>, TagError> {
        Ok(None)
    }

    fn serialize_some<U: ?Sized + ser::Serialize>(
        mut self,
        value: &U,
    ) -> Result<Option<u64>, TagError> {
        value.serialize(self).map(|v| v.or(Some(0)))
    }

    fn serialize_u8(mut self, v: u8) -> Result<Option<u64>, TagError> {
        Ok(Some(v.into()))
    }

    fn serialize_u16(mut self, v: u16) -> Result<Option<u64>, TagError> {
        Ok(Some(v.into()))
    }

    fn serialize_u32(mut self, v: u32) -> Result<Option<u64>, TagError> {
        Ok(Some(v.into()))
    }

    fn serialize_u64(mut self, v: u64) -> Result<Option<u64>, TagError> {
        Ok(Some(v))
    }

    fn serialize_bool(mut self, _: bool) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_i8(mut self, _: i8) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_i16(mut self, _: i16) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_i32(mut self, _: i32) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_i64(mut self, _: i64) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_i128(mut self, _: i128) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_u128(mut self, _: u128) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_f32(mut self, _: f32) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_f64(mut self, _: f64) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_char(mut self, _: char) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_str(mut self, _: &str) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_bytes(mut self, _: &[u8]) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_unit(mut self) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_unit_struct(mut self, _: &'static str) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_unit_variant(
        mut self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_newtype_struct<U: ?Sized + ser::Serialize>(
        mut self,
        _: &'static str,
        _: &U,
    ) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_newtype_variant<U: ?Sized + ser::Serialize>(
        mut self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &U,
    ) -> Result<Option<u64>, TagError> {
        Err(TagError)
    }
    fn serialize_seq(mut self, _: Option<usize>) -> Result<Self::SerializeSeq, TagError> {
        Err(TagError)
    }
    fn serialize_tuple(mut self, _: usize) -> Result<Self::SerializeTuple, TagError> {
        Err(TagError)
    }
    fn serialize_tuple_struct(
        mut self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, TagError> {
        Err(TagError)
    }
    fn serialize_tuple_variant(
        mut self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, TagError> {
        Err(TagError)
    }
    fn serialize_map(mut self, _: Option<usize>) -> Result<Self::SerializeMap, TagError> {
        Err(TagError)
    }
    fn serialize_struct(
        mut self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, TagError> {
        Err(TagError)
    }
    fn serialize_struct_variant(
        mut self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, TagError> {
        Err(TagError)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Convenience functions
// ---------------------------------------------------------------------------

/// Serializes a value as CBOR into an [`Output`].
#[inline]
pub fn into_writer<T: ?Sized + ser::Serialize, O: Output>(
    value: &T,
    output: O,
) -> Result<(), Error<O::Error>>
where
    O::Error: core::fmt::Debug,
{
    value.serialize(Serializer::new(Encoder(output)))
}

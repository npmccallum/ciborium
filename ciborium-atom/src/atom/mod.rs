// SPDX-License-Identifier: Apache-2.0

mod float;
mod other;
mod short;
mod simple;
mod unsigned;

pub use float::Float;
pub use other::Other;
pub use short::Short;
pub use simple::Simple;
pub use unsigned::Unsigned;

use flex::Flex;

use crate::error::Error;
use crate::input::Input;
use crate::output::Output;

/// A single CBOR item as it appears on the wire.
///
/// Each variant corresponds to a CBOR major type. `Option` fields use
/// `None` to represent indefinite-length encoding or the break stop code.
#[derive(Clone, Debug, PartialEq)]
pub enum Atom<'a> {
    /// Positive integer (major type 0)
    Positive(Unsigned),

    /// Negative integer (major type 1): the value represents -1 - n
    Negative(Unsigned),

    /// Byte string (major type 2): `None` starts an indefinite-length sequence
    Bytes(Option<Flex<'a, [u8]>>),

    /// Text string (major type 3): `None` starts an indefinite-length sequence
    Text(Option<Flex<'a, str>>),

    /// Array (major type 4): `None` starts an indefinite-length array
    Array(Option<Unsigned>),

    /// Map (major type 5): `None` starts an indefinite-length map
    Map(Option<Unsigned>),

    /// Tag (major type 6)
    Tag(Unsigned),

    /// Simple/float/break (major type 7): `None` is the break stop code
    Other(Option<Other>),
}

impl Atom<'_> {
    /// Shrink all numeric values to the smallest lossless wire representation.
    #[inline]
    pub fn shrink(self) -> Self {
        match self {
            Self::Positive(u) => Self::Positive(u.shrink()),
            Self::Negative(u) => Self::Negative(u.shrink()),
            Self::Other(Some(Other::Float(f))) => Self::Other(Some(Other::Float(f.shrink()))),
            other => other,
        }
    }

    /// Expand all numeric values to the largest wire representation.
    #[inline]
    pub fn expand(self) -> Self {
        match self {
            Self::Positive(u) => Self::Positive(u.expand()),
            Self::Negative(u) => Self::Negative(u.expand()),
            Self::Other(Some(Other::Float(f))) => Self::Other(Some(Other::Float(f.expand()))),
            other => other,
        }
    }

    /// Encode this atom to an output.
    ///
    /// Note: for `Bytes` and `Text` variants, the length is derived from
    /// the payload data and always encoded in the smallest form. This is
    /// the only case where the wire encoding cannot be controlled
    /// directly; all other variants preserve their exact wire size.
    pub fn encode<O: Output>(&self, mut output: O) -> Result<(), O::Error> {
        match self {
            Self::Positive(u) => u.encode(output, 0, &[]),
            Self::Negative(u) => u.encode(output, 1, &[]),

            Self::Bytes(None) => output.write(2 << 5 | 31, &[], &[]),
            Self::Bytes(Some(b)) => Unsigned::from(b.len() as u64).shrink().encode(output, 2, b),

            Self::Text(None) => output.write(3 << 5 | 31, &[], &[]),
            Self::Text(Some(s)) => {
                Unsigned::from(s.len() as u64).shrink().encode(output, 3, s.as_bytes())
            }

            Self::Array(None) => output.write(4 << 5 | 31, &[], &[]),
            Self::Array(Some(u)) => u.encode(output, 4, &[]),

            Self::Map(None) => output.write(5 << 5 | 31, &[], &[]),
            Self::Map(Some(u)) => u.encode(output, 5, &[]),

            Self::Tag(u) => u.encode(output, 6, &[]),

            Self::Other(None) => output.write(7 << 5 | 31, &[], &[]),
            Self::Other(Some(o)) => o.encode(output),
        }
    }
}

impl<'a> Atom<'a> {
    /// Decode a single CBOR atom from the input.
    ///
    /// Returns `Ok(None)` at end of stream.
    pub fn decode<I: Input<'a>>(mut input: I) -> Result<Option<Self>, Error<I::Error>> {
        let first = match input.head().map_err(Error::Input)? {
            Some(b) => b,
            None => return Ok(None),
        };

        let major = first >> 5;
        let info = first & 0x1f;

        let atom = match major {
            0 => Self::Positive(Unsigned::decode(&mut input, info)?),
            1 => Self::Negative(Unsigned::decode(&mut input, info)?),

            2 if info == 31 => Self::Bytes(None),
            2 => {
                let u = Unsigned::decode(&mut input, info)?;
                let len = usize::try_from(u).map_err(|_| Error::Overflow)?;
                let data = input.tail(len).map_err(Error::Input)?;
                Self::Bytes(Some(data))
            }

            3 if info == 31 => Self::Text(None),
            3 => {
                let u = Unsigned::decode(&mut input, info)?;
                let len = usize::try_from(u).map_err(|_| Error::Overflow)?;
                let data = input.text(len).map_err(Error::Input)?;
                Self::Text(Some(data))
            }

            4 if info == 31 => Self::Array(None),
            4 => Self::Array(Some(Unsigned::decode(&mut input, info)?)),

            5 if info == 31 => Self::Map(None),
            5 => Self::Map(Some(Unsigned::decode(&mut input, info)?)),

            6 => Self::Tag(Unsigned::decode(&mut input, info)?),

            7 => Self::Other(Other::decode(&mut input, info)?),

            _ => return Err(Error::Invalid), // major is 3 bits; can't happen
        };

        Ok(Some(atom))
    }
}

// ---------------------------------------------------------------------------
// From impls: convert Rust primitives to Atoms (shrinking by default)
// ---------------------------------------------------------------------------

impl From<u8> for Atom<'_> {
    #[inline]
    fn from(v: u8) -> Self {
        Self::Positive(Unsigned::from(v))
    }
}

impl From<u16> for Atom<'_> {
    #[inline]
    fn from(v: u16) -> Self {
        Self::Positive(Unsigned::from(v))
    }
}

impl From<u32> for Atom<'_> {
    #[inline]
    fn from(v: u32) -> Self {
        Self::Positive(Unsigned::from(v))
    }
}

impl From<u64> for Atom<'_> {
    #[inline]
    fn from(v: u64) -> Self {
        Self::Positive(Unsigned::from(v))
    }
}

impl From<i8> for Atom<'_> {
    #[inline]
    fn from(v: i8) -> Self {
        if v.is_negative() {
            Self::Negative(Unsigned::from(v as u8 ^ !0))
        } else {
            Self::Positive(Unsigned::from(v as u8))
        }
    }
}

impl From<i16> for Atom<'_> {
    #[inline]
    fn from(v: i16) -> Self {
        if v.is_negative() {
            Self::Negative(Unsigned::from(v as u16 ^ !0))
        } else {
            Self::Positive(Unsigned::from(v as u16))
        }
    }
}

impl From<i32> for Atom<'_> {
    #[inline]
    fn from(v: i32) -> Self {
        if v.is_negative() {
            Self::Negative(Unsigned::from(v as u32 ^ !0))
        } else {
            Self::Positive(Unsigned::from(v as u32))
        }
    }
}

impl From<i64> for Atom<'_> {
    #[inline]
    fn from(v: i64) -> Self {
        if v.is_negative() {
            Self::Negative(Unsigned::from(v as u64 ^ !0))
        } else {
            Self::Positive(Unsigned::from(v as u64))
        }
    }
}

impl From<f32> for Atom<'_> {
    #[inline]
    fn from(v: f32) -> Self {
        Self::Other(Some(Other::Float(Float::from(v))))
    }
}

impl From<f64> for Atom<'_> {
    #[inline]
    fn from(v: f64) -> Self {
        Self::Other(Some(Other::Float(Float::from(v))))
    }
}

impl From<bool> for Atom<'_> {
    #[inline]
    fn from(v: bool) -> Self {
        Self::Other(Some(Other::Simple(if v {
            Simple::TRUE
        } else {
            Simple::FALSE
        })))
    }
}

// SPDX-License-Identifier: Apache-2.0

mod float;
mod head;
mod other;
pub(crate) mod short;
mod simple;
mod unsigned;

pub use float::Float;
pub use head::Head;
pub use other::Other;
pub use short::Short;
pub use simple::Simple;
pub use unsigned::Unsigned;

use flex::Flex;
use floats::f16;

use crate::error::Error;
use crate::input::Input;

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
    /// Encode this atom into a head and a tail.
    ///
    /// The head contains the CBOR major type and argument bytes. The tail
    /// contains the payload data for `Bytes` and `Text` variants, and is
    /// empty for all other variants.
    ///
    /// Note: for `Bytes` and `Text` variants, the length is derived from
    /// the payload data and always encoded in the smallest form. This is
    /// the only case where the wire encoding cannot be controlled
    /// directly; all other variants preserve their exact wire size.
    pub fn encode(&self) -> (Head, &[u8]) {
        match self {
            Self::Positive(u) => (u.encode(0), &[]),
            Self::Negative(u) => (u.encode(1), &[]),

            Self::Bytes(None) => (Head::new0(2 << 5 | 31), &[]),
            Self::Bytes(Some(b)) => (Unsigned::from(b.len() as u64).encode(2), b),

            Self::Text(None) => (Head::new0(3 << 5 | 31), &[]),
            Self::Text(Some(s)) => (Unsigned::from(s.len() as u64).encode(3), s.as_bytes()),

            Self::Array(None) => (Head::new0(4 << 5 | 31), &[]),
            Self::Array(Some(u)) => (u.encode(4), &[]),

            Self::Map(None) => (Head::new0(5 << 5 | 31), &[]),
            Self::Map(Some(u)) => (u.encode(5), &[]),

            Self::Tag(u) => (u.encode(6), &[]),

            Self::Other(None) => (Head::new0(7 << 5 | 31), &[]),
            Self::Other(Some(o)) => (o.encode(), &[]),
        }
    }
}

impl<'a> Atom<'a> {
    /// Decode a single CBOR atom from the input.
    ///
    /// Returns `Ok(None)` at end of stream.
    pub fn decode<I: Input<'a>>(input: &mut I) -> Result<Option<Self>, Error<I::Error>> {
        let first = match input.head().map_err(Error::Input)? {
            Some(b) => b,
            None => return Ok(None),
        };

        let major = first >> 5;
        let info = first & 0x1f;

        let atom = match major {
            0 => Self::Positive(Unsigned::decode(input, info)?),
            1 => Self::Negative(Unsigned::decode(input, info)?),

            2 if info == 31 => Self::Bytes(None),
            2 => {
                let u = Unsigned::decode(input, info)?;
                let len = usize::try_from(u).map_err(|_| Error::Overflow)?;
                let data = input.tail(len).map_err(Error::Input)?;
                Self::Bytes(Some(data))
            }

            3 if info == 31 => Self::Text(None),
            3 => {
                let u = Unsigned::decode(input, info)?;
                let len = usize::try_from(u).map_err(|_| Error::Overflow)?;
                let data = input.text(len).map_err(Error::Input)?;
                Self::Text(Some(data))
            }

            4 if info == 31 => Self::Array(None),
            4 => Self::Array(Some(Unsigned::decode(input, info)?)),

            5 if info == 31 => Self::Map(None),
            5 => Self::Map(Some(Unsigned::decode(input, info)?)),

            6 => Self::Tag(Unsigned::decode(input, info)?),

            7 if info == 31 => Self::Other(None),
            7 if info == 25 => {
                let bytes = input.body::<2>().map_err(Error::Input)?;
                Self::Other(Some(Other::Float(Float::F2(f16::from_be_bytes(bytes)))))
            }
            7 if info == 26 => {
                let bytes = input.body::<4>().map_err(Error::Input)?;
                Self::Other(Some(Other::Float(Float::F4(f32::from_be_bytes(bytes)))))
            }
            7 if info == 27 => {
                let bytes = input.body::<8>().map_err(Error::Input)?;
                Self::Other(Some(Other::Float(Float::F8(f64::from_be_bytes(bytes)))))
            }
            7 if info < 24 => {
                Self::Other(Some(Other::Simple(Simple::S0(Short(info)))))
            }
            7 if info == 24 => {
                let v = input.body::<1>().map_err(Error::Input)?[0];
                Self::Other(Some(Other::Simple(Simple::S1(v))))
            }
            7 => return Err(Error::Invalid), // info 28-30

            _ => return Err(Error::Invalid), // major is 3 bits; can't happen
        };

        Ok(Some(atom))
    }
}

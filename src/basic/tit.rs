// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::io::{Read, Write};

use core::convert::{TryFrom, TryInto};

/// The `Prefix`, `Affix` sections of a CBOR item
///
/// For numeric types, including simple constants such as booleans, no
/// additional `Suffix` data is required. For complex types, the `Title`
/// indicates the length of additional CBOR items or bytes (depending on
/// context) to read from the `Suffix` section. For this reason, `Title`
/// implements conversions to and from all the major Rust numeric types.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Title(pub Major, pub Minor);

impl Title {
    /// A simple constant representing `false`
    pub const FALSE: Self = Self(Major::Other, Minor::Immediate(Immediate(20)));

    /// A simple constant representing `true`
    pub const TRUE: Self = Self(Major::Other, Minor::Immediate(Immediate(21)));

    /// A simple constant representing `null`
    pub const NULL: Self = Self(Major::Other, Minor::Immediate(Immediate(22)));

    /// A simple constant representing `undefined`
    pub const UNDEFINED: Self = Self(Major::Other, Minor::Immediate(Immediate(23)));

    /// The break indicator
    pub const BREAK: Self = Self(Major::Other, Minor::Indeterminate);

    /// A tag indicating the next item is a positive arbitrary sized integer
    pub const TAG_BIGPOS: Self = Self(Major::Tag, Minor::Immediate(Immediate(2)));

    /// A tag indicating the next item is a negative arbitrary sized integer
    pub const TAG_BIGNEG: Self = Self(Major::Tag, Minor::Immediate(Immediate(3)));
}

impl TryFrom<Title> for f32 {
    type Error = InvalidError;

    #[inline]
    #[allow(clippy::float_cmp)]
    fn try_from(value: Title) -> Result<Self, Self::Error> {
        match value.0 {
            Major::Other => f32::try_from(value.1),
            _ => Err(InvalidError(())),
        }
    }
}

impl TryFrom<Title> for f64 {
    type Error = InvalidError;

    #[inline]
    fn try_from(value: Title) -> Result<Self, Self::Error> {
        match value.0 {
            Major::Other => f64::try_from(value.1),
            _ => Err(InvalidError(())),
        }
    }
}

impl TryFrom<Title> for i128 {
    type Error = InvalidError;

    #[inline]
    fn try_from(value: Title) -> Result<Self, Self::Error> {
        let x = Option::<u64>::from(value.1).ok_or(InvalidError(()))? as i128;

        match value.0 {
            Major::Positive => Ok(x),
            Major::Negative => Ok(x ^ !0),
            _ => Err(InvalidError(())),
        }
    }
}

impl TryFrom<Title> for Option<usize> {
    type Error = InvalidError;

    #[inline]
    fn try_from(value: Title) -> Result<Self, Self::Error> {
        match value.0 {
            Major::Bytes | Major::Text | Major::Array | Major::Map => Option::<u64>::from(value.1)
                .map(usize::try_from)
                .transpose()
                .or(Err(InvalidError(()))),

            _ => Err(InvalidError(())),
        }
    }
}

impl From<bool> for Title {
    #[inline]
    fn from(value: bool) -> Self {
        if value {
            Title::TRUE
        } else {
            Title::FALSE
        }
    }
}

impl From<u8> for Title {
    #[inline]
    fn from(value: u8) -> Self {
        Self(Major::Positive, Minor::from(u64::from(value)))
    }
}

impl From<u16> for Title {
    #[inline]
    fn from(value: u16) -> Self {
        Self(Major::Positive, Minor::from(u64::from(value)))
    }
}

impl From<u32> for Title {
    #[inline]
    fn from(value: u32) -> Self {
        Self(Major::Positive, Minor::from(u64::from(value)))
    }
}

impl From<u64> for Title {
    #[inline]
    fn from(value: u64) -> Self {
        Self(Major::Positive, Minor::from(value))
    }
}

impl TryFrom<u128> for Title {
    type Error = InvalidError;

    #[inline]
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let x = u64::try_from(value).map_err(|_| InvalidError(()))?;
        Ok(Self(Major::Positive, Minor::from(x)))
    }
}

impl From<i8> for Title {
    #[inline]
    fn from(value: i8) -> Self {
        let (major, value) = match value.is_negative() {
            false => (Major::Positive, value as u64),
            true => (Major::Negative, value as u64 ^ !0),
        };

        Self(major, Minor::from(value))
    }
}

impl From<i16> for Title {
    #[inline]
    fn from(value: i16) -> Self {
        let (major, value) = match value.is_negative() {
            false => (Major::Positive, value as u64),
            true => (Major::Negative, value as u64 ^ !0),
        };

        Self(major, Minor::from(value))
    }
}

impl From<i32> for Title {
    #[inline]
    fn from(value: i32) -> Self {
        let (major, value) = match value.is_negative() {
            false => (Major::Positive, value as u64),
            true => (Major::Negative, value as u64 ^ !0),
        };

        Self(major, Minor::from(value))
    }
}

impl From<i64> for Title {
    #[inline]
    fn from(value: i64) -> Self {
        let (major, value) = match value.is_negative() {
            false => (Major::Positive, value as u64),
            true => (Major::Negative, value as u64 ^ !0),
        };

        Self(major, Minor::from(value))
    }
}

impl TryFrom<i128> for Title {
    type Error = InvalidError;

    #[inline]
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let (major, value) = match value.is_negative() {
            false => (Major::Positive, u64::try_from(value)),
            true => (Major::Negative, u64::try_from(value ^ !0)),
        };

        Ok(Self(major, value.map_err(|_| InvalidError(()))?.into()))
    }
}

impl From<f32> for Title {
    #[inline]
    fn from(value: f32) -> Self {
        Self(Major::Other, Minor::from(value as f64))
    }
}

impl From<f64> for Title {
    #[inline]
    fn from(value: f64) -> Self {
        Self(Major::Other, Minor::from(value))
    }
}

impl Title {
    /// Creates a title from a `Major` and a length
    #[inline]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64",))]
    pub fn from_length(major: Major, length: impl Into<Option<usize>>) -> Self {
        Self(major, length.into().map(|x| x as u64).into())
    }

    /// Returns the number of bytes used for this title on the wire
    #[inline]
    pub fn len(&self) -> usize {
        self.1.as_ref().len() + 1
    }

    /// Decodes a title from the supplied reader
    #[inline]
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError<R::Error>> {
        let mut prefix = 0u8;
        reader.read_exact(core::slice::from_mut(&mut prefix))?;

        let major = match prefix >> 5 {
            0 => Major::Positive,
            1 => Major::Negative,
            2 => Major::Bytes,
            3 => Major::Text,
            4 => Major::Array,
            5 => Major::Map,
            6 => Major::Tag,
            7 => Major::Other,
            _ => unreachable!(),
        };

        let mut minor = match prefix & 0b00011111 {
            24 => Minor::Subsequent1([0u8; 1]),
            25 => Minor::Subsequent2([0u8; 2]),
            26 => Minor::Subsequent4([0u8; 4]),
            27 => Minor::Subsequent8([0u8; 8]),
            31 => Minor::Indeterminate,
            x => Minor::Immediate(x.try_into().or(Err(DecodeError::Invalid))?),
        };

        reader.read_exact(minor.as_mut())?;
        Ok(Self(major, minor))
    }

    /// Encodes a title to the supplied writer
    #[inline]
    pub fn encode<W: Write>(&self, writer: &mut W) -> Result<(), W::Error> {
        let major: u8 = match self.0 {
            Major::Positive => 0,
            Major::Negative => 1,
            Major::Bytes => 2,
            Major::Text => 3,
            Major::Array => 4,
            Major::Map => 5,
            Major::Tag => 6,
            Major::Other => 7,
        };

        let minor: u8 = match self.1 {
            Minor::Immediate(x) => x.into(),
            Minor::Subsequent1(..) => 24,
            Minor::Subsequent2(..) => 25,
            Minor::Subsequent4(..) => 26,
            Minor::Subsequent8(..) => 27,
            Minor::Indeterminate => 31,
        };

        writer.write_all(&[(major << 5) | minor])?;
        writer.write_all(self.1.as_ref())?;
        Ok(())
    }
}

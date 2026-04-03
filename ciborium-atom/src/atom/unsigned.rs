// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::input::Input;
use crate::output::Output;

use super::short::Short;

/// All sizes a CBOR unsigned integer can take on the wire.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Unsigned {
    /// Value encoded inline in the additional info (0 argument bytes)
    U0(Short),

    /// Value encoded with 1 argument byte
    U1(u8),

    /// Value encoded with 2 argument bytes
    U2(u16),

    /// Value encoded with 4 argument bytes
    U4(u32),

    /// Value encoded with 8 argument bytes
    U8(u64),
}

impl Unsigned {
    /// Decode an unsigned argument from the additional info value.
    pub(crate) fn decode<'a, I: Input<'a>>(
        input: &mut I,
        info: u8,
    ) -> Result<Self, Error<I::Error>> {
        match info {
            v @ 0..24 => Ok(Self::U0(Short(v))),
            24 => Ok(Self::U1(input.body::<1>().map_err(Error::Input)?[0])),
            25 => Ok(Self::U2(u16::from_be_bytes(
                input.body().map_err(Error::Input)?,
            ))),
            26 => Ok(Self::U4(u32::from_be_bytes(
                input.body().map_err(Error::Input)?,
            ))),
            27 => Ok(Self::U8(u64::from_be_bytes(
                input.body().map_err(Error::Input)?,
            ))),
            _ => Err(Error::Invalid),
        }
    }

    /// Encode this unsigned value to an output with the given major type.
    #[inline]
    pub(crate) fn encode<O: Output>(
        self,
        major: u8,
        output: &mut O,
    ) -> Result<(), O::Error> {
        self.write(major, output, &[])
    }

    /// Encode this unsigned value to an output with the given major type
    /// and a tail payload.
    #[inline]
    pub(crate) fn write<O: Output>(
        self,
        major: u8,
        output: &mut O,
        tail: &[u8],
    ) -> Result<(), O::Error> {
        let mt = major << 5;
        match self {
            Self::U0(v) => output.write(mt | v.get(), &[], tail),
            Self::U1(v) => output.write(mt | 24, &[v], tail),
            Self::U2(v) => output.write(mt | 25, &v.to_be_bytes(), tail),
            Self::U4(v) => output.write(mt | 26, &v.to_be_bytes(), tail),
            Self::U8(v) => output.write(mt | 27, &v.to_be_bytes(), tail),
        }
    }
}

impl From<u8> for Unsigned {
    #[inline]
    fn from(v: u8) -> Self {
        match Short::new(v) {
            Some(s) => Self::U0(s),
            None => Self::U1(v),
        }
    }
}

impl From<u16> for Unsigned {
    #[inline]
    fn from(v: u16) -> Self {
        match u8::try_from(v) {
            Ok(v) => v.into(),
            Err(_) => Self::U2(v),
        }
    }
}

impl From<u32> for Unsigned {
    #[inline]
    fn from(v: u32) -> Self {
        match u16::try_from(v) {
            Ok(v) => v.into(),
            Err(_) => Self::U4(v),
        }
    }
}

impl From<u64> for Unsigned {
    #[inline]
    fn from(v: u64) -> Self {
        match u32::try_from(v) {
            Ok(v) => v.into(),
            Err(_) => Self::U8(v),
        }
    }
}

impl From<Unsigned> for u64 {
    #[inline]
    fn from(v: Unsigned) -> Self {
        match v {
            Unsigned::U0(v) => v.get().into(),
            Unsigned::U1(v) => v.into(),
            Unsigned::U2(v) => v.into(),
            Unsigned::U4(v) => v.into(),
            Unsigned::U8(v) => v,
        }
    }
}

impl TryFrom<Unsigned> for usize {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: Unsigned) -> Result<Self, Self::Error> {
        u64::from(v).try_into()
    }
}

impl TryFrom<Unsigned> for u32 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: Unsigned) -> Result<Self, Self::Error> {
        u64::from(v).try_into()
    }
}

impl TryFrom<Unsigned> for u16 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: Unsigned) -> Result<Self, Self::Error> {
        u64::from(v).try_into()
    }
}

impl TryFrom<Unsigned> for u8 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: Unsigned) -> Result<Self, Self::Error> {
        u64::from(v).try_into()
    }
}

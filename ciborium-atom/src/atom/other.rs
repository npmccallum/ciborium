// SPDX-License-Identifier: Apache-2.0

use floats::f16;

use crate::error::Error;
use crate::input::Input;
use crate::output::Output;

use super::short::Short;
use super::{Float, Simple};

/// CBOR major type 7: simple values and floats.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Other {
    /// Simple value
    Simple(Simple),

    /// Floating-point value
    Float(Float),
}

impl Other {
    /// Decode major type 7 from the additional info value.
    pub(crate) fn decode<'a, I: Input<'a>>(
        mut input: I,
        info: u8,
    ) -> Result<Option<Self>, Error<I::Error>> {
        Ok(Some(match info {
            v @ 0..24 => Self::Simple(Simple::S0(Short(v))),
            24 => Self::Simple(Simple::S1(input.body::<1>().map_err(Error::Input)?[0])),
            25 => Self::Float(Float::F2(f16::from_be_bytes(
                input.body().map_err(Error::Input)?,
            ))),
            26 => Self::Float(Float::F4(f32::from_be_bytes(
                input.body().map_err(Error::Input)?,
            ))),
            27 => Self::Float(Float::F8(f64::from_be_bytes(
                input.body().map_err(Error::Input)?,
            ))),
            31 => return Ok(None),
            _ => return Err(Error::Invalid),
        }))
    }

    /// Encode this value to an output (major type 7).
    #[inline]
    pub(crate) fn encode<O: Output>(self, output: O) -> Result<(), O::Error> {
        match self {
            Self::Simple(s) => s.encode(output),
            Self::Float(f) => f.encode(output),
        }
    }
}

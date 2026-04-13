// SPDX-License-Identifier: Apache-2.0

use floats::casting::CastInto;
use floats::f16;

use crate::output::Output;

/// A CBOR floating-point value preserving its wire size.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Float {
    /// 16-bit half-precision (2 argument bytes)
    F2(f16),

    /// 32-bit single-precision (4 argument bytes)
    F4(f32),

    /// 64-bit double-precision (8 argument bytes)
    F8(f64),
}

impl Float {
    /// Encode this float to an output (major type 7).
    #[inline]
    pub(crate) fn encode<O: Output>(self, mut output: O) -> Result<(), O::Error> {
        let mt = 7 << 5;
        match self {
            Self::F2(v) => output.write(mt | 25, &v.to_be_bytes(), &[]),
            Self::F4(v) => output.write(mt | 26, &v.to_be_bytes(), &[]),
            Self::F8(v) => output.write(mt | 27, &v.to_be_bytes(), &[]),
        }
    }
}

impl Float {
    /// Shrink to the smallest lossless wire representation.
    ///
    /// NaN values are canonicalized to f16, preserving the sign bit.
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub fn shrink(self) -> Self {
        match self {
            Self::F8(v) => {
                if v.is_nan() {
                    let nan: f16 = v.cast_into();
                    return Self::F2(nan);
                }

                let single = v as f32;
                if single as f64 == v {
                    return Self::F4(single).shrink();
                }

                Self::F8(v)
            }
            Self::F4(v) => {
                if v.is_nan() {
                    let nan: f16 = v.cast_into();
                    return Self::F2(nan);
                }

                let half: f16 = v.cast_into();
                let back: f32 = half.cast_into();
                if back == v {
                    return Self::F2(half);
                }

                Self::F4(v)
            }
            Self::F2(v) => Self::F2(v),
        }
    }

    /// Expand to the largest wire representation (F8).
    #[inline]
    pub fn expand(self) -> Self {
        Self::F8(f64::from(self))
    }
}

impl From<f16> for Float {
    #[inline]
    fn from(v: f16) -> Self {
        Self::F2(v)
    }
}

impl From<f32> for Float {
    #[inline]
    fn from(v: f32) -> Self {
        Self::F4(v)
    }
}

impl From<f64> for Float {
    #[inline]
    fn from(v: f64) -> Self {
        Self::F8(v)
    }
}

impl TryFrom<Float> for f16 {
    type Error = Float;

    #[inline]
    fn try_from(v: Float) -> Result<Self, Self::Error> {
        match v {
            Float::F2(v) => Ok(v),
            _ => Err(v),
        }
    }
}

impl TryFrom<Float> for f32 {
    type Error = Float;

    #[inline]
    fn try_from(v: Float) -> Result<Self, Self::Error> {
        match v {
            Float::F2(v) => Ok(v.cast_into()),
            Float::F4(v) => Ok(v),
            _ => Err(v),
        }
    }
}

impl From<Float> for f64 {
    #[inline]
    fn from(v: Float) -> Self {
        match v {
            Float::F2(v) => v.cast_into(),
            Float::F4(v) => v.into(),
            Float::F8(v) => v,
        }
    }
}

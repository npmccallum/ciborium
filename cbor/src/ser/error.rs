// SPDX-License-Identifier: Apache-2.0

use core::fmt::{Debug, Display, Formatter};

use serde::ser;

/// An error that occurred during serialization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error<T> {
    /// The underlying transport produced an error.
    Io(T),

    /// A custom error from a `Serialize` implementation.
    #[cfg(not(feature = "alloc"))]
    Custom,

    /// A custom error from a `Serialize` implementation.
    #[cfg(feature = "alloc")]
    Custom(alloc::string::String),
}

impl<T> From<T> for Error<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::Io(value)
    }
}

impl<T: Debug> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e:?}"),

            #[cfg(not(feature = "alloc"))]
            Self::Custom => f.write_str("custom serialization error"),

            #[cfg(feature = "alloc")]
            Self::Custom(msg) => f.write_str(msg),
        }
    }
}

impl<T: Debug> ser::StdError for Error<T> {}

impl<T: Debug> ser::Error for Error<T> {
    #[cfg(not(feature = "alloc"))]
    fn custom<U: Display>(_: U) -> Self {
        Self::Custom
    }

    #[cfg(feature = "alloc")]
    fn custom<U: Display>(msg: U) -> Self {
        use alloc::string::ToString;
        Self::Custom(msg.to_string())
    }
}

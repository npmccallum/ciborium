// SPDX-License-Identifier: Apache-2.0

//! Deserialization error types.

use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};

use ciborium_atom::Atom;
use serde::de;
use serde::ser;

/// What the deserializer expected to receive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Class {
    /// Any non-break atom.
    Atom,

    /// The break stop code.
    Break,
}

impl Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom => f.write_str("CBOR atom"),
            Self::Break => f.write_str("break"),
        }
    }
}

/// The deserializer expected one thing but received another.
#[derive(Clone, Debug)]
pub struct Expected {
    /// What was expected.
    pub expected: Class,

    /// What was received, or `None` for end of input.
    pub received: Option<Atom<'static>>,
}

impl Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.received {
            None => write!(f, "expected {}, got end of input", self.expected),
            Some(atom) => write!(f, "expected {}, got {:?}", self.expected, atom),
        }
    }
}

/// An error that occurred during deserialization.
#[derive(Clone, Debug)]
pub enum Error<T> {
    /// The underlying transport produced an error.
    Io(T),

    /// The deserializer expected one thing but received another.
    Expected(Expected),

    /// The recursion limit was exceeded.
    RecursionLimitExceeded,

    /// A custom error from serde.
    Custom(String),
}

impl<T: Debug> Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e:?}"),
            Self::Expected(e) => Display::fmt(e, f),
            Self::RecursionLimitExceeded => f.write_str("recursion limit exceeded"),
            Self::Custom(msg) => f.write_str(msg),
        }
    }
}

impl<T: Debug> ser::StdError for Error<T> {}

impl<T: Debug> de::Error for Error<T> {
    fn custom<U: Display>(msg: U) -> Self {
        Self::Custom(msg.to_string())
    }
}

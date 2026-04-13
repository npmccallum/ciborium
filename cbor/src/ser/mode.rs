// SPDX-License-Identifier: Apache-2.0

/// Sentinel flags set by newtype wrappers, consumed by serialize methods.
///
/// Only one flag is active at a time. The flag is set by a sentinel
/// wrapper and consumed by the next serialize method via `take()`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    /// Default behavior: shrink numerics, definite-length collections.
    #[default]
    Default,

    /// Use indefinite-length encoding for the next collection.
    Indefinite,
}

impl Mode {
    /// Create a flag from a sentinel name.
    pub fn from_sentinel(name: &str) -> Self {
        match name {
            "@@CBOR_INDEFINITE@@" => Self::Indefinite,
            _ => Self::Default,
        }
    }
}

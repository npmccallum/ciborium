// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![forbid(clippy::panic)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::unreachable)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::cast_possible_truncation)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod atom;
mod error;

pub mod input;
pub mod output;

pub use atom::{Atom, Float, Other, Short, Simple, Unsigned};
pub use error::Error;
pub use output::Output;

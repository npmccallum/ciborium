// SPDX-License-Identifier: Apache-2.0

//! A serde implementation of CBOR built on ciborium-atom.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

pub use ciborium_atom as atom;

pub mod de;
pub mod pull;
pub mod push;
pub mod ser;
pub mod types;

/// Big integer tag constants.
pub(crate) const BIGPOS: u64 = 2;
pub(crate) const BIGNEG: u64 = 3;

// SPDX-License-Identifier: Apache-2.0

//! CBOR-specific types for use with serde.

pub(crate) mod indefinite;
pub(crate) mod null;
pub(crate) mod simple;
pub(crate) mod tag;
pub(crate) mod undefined;

pub use indefinite::Indefinite;
pub use null::Null;
pub use simple::Simple;
pub use tag::{AllowAny, AllowExact, RequireAny, RequireExact};
pub use undefined::Undefined;

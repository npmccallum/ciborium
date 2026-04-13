// SPDX-License-Identifier: Apache-2.0

//! Collection serializer types, one per serde trait.

mod map;
mod seq;
mod r#struct;
mod struct_variant;
mod tuple;
mod tuple_struct;
mod tuple_variant;

pub use map::Map;
pub use r#struct::Struct;
pub use seq::Seq;
pub use struct_variant::StructVariant;
pub use tuple::Tuple;
pub use tuple_struct::TupleStruct;
pub use tuple_variant::TupleVariant;

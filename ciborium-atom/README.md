# ciborium-atom

Low-level CBOR wire types for `no_std` environments.

This crate models individual CBOR data items (RFC 8949) as Rust types. It covers
**syntax, not semantics** — higher-level interpretation belongs in the consumer.
Every variant preserves the exact wire encoding size, enabling lossless
roundtrips.

## Types

Each CBOR major type maps to an [`Atom`] variant:

| Major | Variant    | Argument             |
| ----: | ---------- | -------------------- |
|     0 | `Positive` | `Unsigned`           |
|     1 | `Negative` | `Unsigned`           |
|     2 | `Bytes`    | `Option<Flex<[u8]>>` |
|     3 | `Text`     | `Option<Flex<str>>`  |
|     4 | `Array`    | `Option<Unsigned>`   |
|     5 | `Map`      | `Option<Unsigned>`   |
|     6 | `Tag`      | `Unsigned`           |
|     7 | `Other`    | `Option<Other>`      |

`None` represents indefinite-length encoding (or break for `Other`).

## Wire size preservation

`Unsigned`, `Float`, and `Simple` carry their wire encoding size in the variant
name: `U0`/`U1`/`U2`/`U4`/`U8`, `F2`/`F4`/`F8`, `S0`/`S1`. The number is the
count of argument bytes after the initial byte.

`From` conversions minimize to the smallest representation. To force a specific
wire size, construct the variant directly.

The one exception is `Bytes` and `Text`: their length is derived from the
payload data and always encoded minimally. The length is not stored separately,
so non-minimal length encodings cannot survive a roundtrip.

## Encoding

```rust
use ciborium_atom::{Atom, Unsigned};

let atom = Atom::Positive(Unsigned::from(42u64));
let mut buf = Vec::new();
atom.encode(&mut buf).unwrap();
assert_eq!(buf, [0x18, 0x2a]);
```

The [`Output`](output::Output) trait abstracts the byte sink. Implementations
are provided for `Vec<u8>`, `&mut [u8]`, and (with the `std` feature)
`std::io::Write` via [`Writer`](output::write::Writer).

## Decoding

```rust
use ciborium_atom::{Atom, Unsigned};

let cbor = [0x18, 0x2a]; // positive integer 42
let atom = Atom::decode(cbor.as_slice()).unwrap();
assert_eq!(atom, Some(Atom::Positive(Unsigned::U1(42))));
```

The [`Input`](input::Input) trait abstracts the byte source. Implementations are
provided for `&[u8]` and (with the `std` feature) `std::io::Read` via
[`Reader`](input::read::Reader). The [`Span`](input::span::Span) wrapper adds
position tracking.

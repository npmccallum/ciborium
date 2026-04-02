// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use ciborium::{de::from_reader, ser::into_writer, Indefinite};
use serde::{Deserialize, Serialize};

/// Serialize a value and return the hex string of the encoded bytes.
fn encode_hex<T: Serialize>(value: &T) -> String {
    let mut buf = Vec::new();
    into_writer(value, &mut buf).unwrap();
    hex::encode(buf)
}

// ----------------------------------------------------------------
// Basic indefinite-length encoding
// ----------------------------------------------------------------

#[test]
fn indefinite_vec() {
    // Definite:   83 01 02 03          = array(3), 1, 2, 3
    // Indefinite: 9f 01 02 03 ff       = indef-array, 1, 2, 3, break
    let definite = encode_hex(&vec![1u8, 2, 3]);
    let indefinite = encode_hex(&Indefinite(vec![1u8, 2, 3]));

    assert_eq!(definite, "83010203");
    assert_eq!(indefinite, "9f010203ff");
}

#[test]
fn indefinite_map() {
    // BTreeMap with one entry "a" => 1
    // Definite:   a1 61 61 01          = map(1), "a", 1
    // Indefinite: bf 61 61 01 ff       = indef-map, "a", 1, break
    let mut m = BTreeMap::new();
    m.insert("a", 1u32);

    let definite = encode_hex(&m);
    let indefinite = encode_hex(&Indefinite(m));

    assert_eq!(definite, "a1616101");
    assert_eq!(indefinite, "bf616101ff");
}

#[test]
fn indefinite_struct() {
    #[derive(Serialize)]
    struct S {
        a: u32,
        b: u32,
    }

    // Struct serializes as map with string keys:
    // Definite:   a2 61 61 01 61 62 02      = map(2), "a", 1, "b", 2
    // Indefinite: bf 61 61 01 61 62 02 ff   = indef-map, "a", 1, "b", 2, break
    let definite = encode_hex(&S { a: 1, b: 2 });
    let indefinite = encode_hex(&Indefinite(S { a: 1, b: 2 }));

    assert_eq!(definite, "a2616101616202");
    assert_eq!(indefinite, "bf616101616202ff");
}

#[test]
fn indefinite_tuple() {
    // Tuple (1u8, 2u8, 3u8) serializes via serialize_tuple → serialize_seq
    // Definite:   83 01 02 03
    // Indefinite: 9f 01 02 03 ff
    let definite = encode_hex(&(1u8, 2u8, 3u8));
    let indefinite = encode_hex(&Indefinite((1u8, 2u8, 3u8)));

    assert_eq!(definite, "83010203");
    assert_eq!(indefinite, "9f010203ff");
}

#[test]
fn indefinite_empty_vec() {
    // Definite:   80               = array(0)
    // Indefinite: 9f ff            = indef-array, break
    let definite = encode_hex(&Vec::<u32>::new());
    let indefinite = encode_hex(&Indefinite(Vec::<u32>::new()));

    assert_eq!(definite, "80");
    assert_eq!(indefinite, "9fff");
}

// ----------------------------------------------------------------
// Non-propagation: flag must NOT leak to nested collections
// ----------------------------------------------------------------

#[test]
fn no_propagation_nested_vec() {
    // Indefinite<Vec<Vec<u8>>>: outer array is indefinite, inner arrays are definite.
    //
    // Inner [1, 2] = 82 01 02 (definite array(2))
    // Inner [3]    = 81 03    (definite array(1))
    // Outer        = 9f ... ff (indefinite)
    //
    // Expected: 9f 82 01 02 81 03 ff
    let val = Indefinite(vec![vec![1u8, 2], vec![3]]);
    assert_eq!(encode_hex(&val), "9f8201028103ff");
}

#[test]
fn no_propagation_struct_fields() {
    // A struct with one Indefinite<Vec> field and one normal Vec field.
    // The struct itself is definite (map(2)). The first field is indefinite,
    // the second field must remain definite.
    #[derive(Serialize)]
    struct Mixed {
        x: Indefinite<Vec<u32>>,
        y: Vec<u32>,
    }

    let val = Mixed {
        x: Indefinite(vec![1, 2]),
        y: vec![3, 4],
    };
    let bytes = encode_hex(&val);

    // a2                    map(2) - the struct, definite
    // 61 78                 text "x"
    // 9f 01 02 ff           indefinite array [1, 2]
    // 61 79                 text "y"
    // 82 03 04              definite array(2) [3, 4]
    assert_eq!(bytes, "a261789f0102ff6179820304");
}

#[test]
fn no_propagation_map_values() {
    // Indefinite<BTreeMap<&str, Vec<u32>>>: the map is indefinite, but the
    // Vec values inside it must be definite.
    let mut m = BTreeMap::new();
    m.insert("k", vec![1u32, 2]);

    let bytes = encode_hex(&Indefinite(m));

    // bf                    indef-map
    // 61 6b                 text "k"
    // 82 01 02              definite array(2) [1, 2]
    // ff                    break
    assert_eq!(bytes, "bf616b820102ff");
}

// ----------------------------------------------------------------
// Non-collection inner type: harmless no-op
// ----------------------------------------------------------------

#[test]
fn indefinite_primitive_is_noop() {
    // Indefinite<u32> should produce the same bytes as u32 alone.
    assert_eq!(encode_hex(&42u32), encode_hex(&Indefinite(42u32)));
    assert_eq!(encode_hex(&true), encode_hex(&Indefinite(true)));
    assert_eq!(encode_hex(&"hello"), encode_hex(&Indefinite("hello")));
}

// ----------------------------------------------------------------
// Deserialization: transparent pass-through
// ----------------------------------------------------------------

#[test]
fn roundtrip_indefinite_vec() {
    let original = vec![10u32, 20, 30];

    // Serialize with Indefinite wrapper
    let mut buf = Vec::new();
    into_writer(&Indefinite(original.clone()), &mut buf).unwrap();

    // Deserialize into plain Vec<u32> (no Indefinite wrapper needed)
    let decoded: Vec<u32> = from_reader(&buf[..]).unwrap();
    assert_eq!(decoded, original);

    // Also works when deserializing into Indefinite<Vec<u32>>
    let decoded2: Indefinite<Vec<u32>> = from_reader(&buf[..]).unwrap();
    assert_eq!(decoded2.0, original);
}

#[test]
fn roundtrip_indefinite_struct() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let original = Point { x: -5, y: 42 };

    let mut buf = Vec::new();
    into_writer(&Indefinite(&original), &mut buf).unwrap();

    // Deserialize without wrapper
    let decoded: Point = from_reader(&buf[..]).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn roundtrip_indefinite_map() {
    let mut original = BTreeMap::new();
    original.insert("alpha".to_string(), 1u64);
    original.insert("beta".to_string(), 2u64);

    let mut buf = Vec::new();
    into_writer(&Indefinite(&original), &mut buf).unwrap();

    let decoded: BTreeMap<String, u64> = from_reader(&buf[..]).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn deserialize_definite_into_indefinite_wrapper() {
    // Serialize normally (definite), then deserialize into Indefinite<Vec<u32>>.
    let original = vec![1u32, 2, 3];

    let mut buf = Vec::new();
    into_writer(&original, &mut buf).unwrap();

    let decoded: Indefinite<Vec<u32>> = from_reader(&buf[..]).unwrap();
    assert_eq!(decoded.0, original);
}

// ----------------------------------------------------------------
// Interaction with CBOR tags
// ----------------------------------------------------------------

#[test]
fn indefinite_with_tag() {
    use ciborium::tag::RequireExact;

    // Tag 42 wrapping an indefinite-length array [1, 2, 3]
    // Expected: d8 2a 9f 01 02 03 ff
    //           tag(42) indef-array 1 2 3 break
    let val = RequireExact::<Indefinite<Vec<u8>>, 42>(Indefinite(vec![1, 2, 3]));
    assert_eq!(encode_hex(&val), "d82a9f010203ff");

    // Deserialize back
    let mut buf = Vec::new();
    into_writer(&val, &mut buf).unwrap();
    let decoded: RequireExact<Vec<u8>, 42> = from_reader(&buf[..]).unwrap();
    assert_eq!(decoded.0, vec![1, 2, 3]);
}

#[test]
fn tag_inside_indefinite() {
    use ciborium::tag::RequireExact;

    // Indefinite wrapper around a tagged value — same wire format
    let val = Indefinite(RequireExact::<Vec<u8>, 42>(vec![1, 2, 3]));
    assert_eq!(encode_hex(&val), "d82a9f010203ff");
}

// ----------------------------------------------------------------
// Existing behavior preserved: definite encoding without wrapper
// ----------------------------------------------------------------

#[test]
fn baseline_definite_encoding() {
    // Ensure unwrapped types still produce definite-length encoding.
    assert_eq!(encode_hex(&vec![1u8, 2, 3]), "83010203");

    let mut m = BTreeMap::new();
    m.insert("a", 1u32);
    assert_eq!(encode_hex(&m), "a1616101");
}

// ----------------------------------------------------------------
// Struct field-level control
// ----------------------------------------------------------------

#[test]
fn struct_with_indefinite_field_roundtrip() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Config {
        name: String,
        values: Indefinite<Vec<u32>>,
    }

    let original = Config {
        name: "test".into(),
        values: Indefinite(vec![100, 200, 300]),
    };

    let mut buf = Vec::new();
    into_writer(&original, &mut buf).unwrap();

    let decoded: Config = from_reader(&buf[..]).unwrap();
    assert_eq!(decoded, original);
}

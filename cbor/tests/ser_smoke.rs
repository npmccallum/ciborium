// SPDX-License-Identifier: Apache-2.0

use cbor::ser::into_writer;

#[test]
fn serialize_u8() {
    let mut buf = Vec::new();
    into_writer(&7u8, &mut buf).unwrap();
    assert_eq!(buf, [0x07]);
}

#[test]
fn serialize_negative() {
    let mut buf = Vec::new();
    into_writer(&-1i8, &mut buf).unwrap();
    assert_eq!(buf, [0x20]);
}

#[test]
fn serialize_bool() {
    let mut buf = Vec::new();
    into_writer(&false, &mut buf).unwrap();
    assert_eq!(buf, [0xf4]);

    buf.clear();
    into_writer(&true, &mut buf).unwrap();
    assert_eq!(buf, [0xf5]);
}

#[test]
fn serialize_null() {
    let mut buf = Vec::new();
    into_writer(&Option::<u8>::None, &mut buf).unwrap();
    assert_eq!(buf, [0xf6]);
}

#[test]
fn serialize_string() {
    let mut buf = Vec::new();
    into_writer(&"IETF", &mut buf).unwrap();
    assert_eq!(buf, hex::decode("6449455446").unwrap());
}

#[test]
fn serialize_array() {
    let mut buf = Vec::new();
    into_writer(&vec![1u8, 2, 3], &mut buf).unwrap();
    assert_eq!(buf, hex::decode("83010203").unwrap());
}

#[test]
fn serialize_float_f16() {
    let mut buf = Vec::new();
    into_writer(&0.0f64, &mut buf).unwrap();
    assert_eq!(buf, hex::decode("f90000").unwrap());
}

#[test]
fn serialize_float_nan() {
    let mut buf = Vec::new();
    into_writer(&f64::NAN, &mut buf).unwrap();
    assert_eq!(buf, hex::decode("f97e00").unwrap());
}

#[test]
fn serialize_neg_nan() {
    let mut buf = Vec::new();
    into_writer(&(-f64::NAN), &mut buf).unwrap();
    assert_eq!(buf, hex::decode("f9fe00").unwrap());
}

#[test]
fn serialize_slice_output() {
    let mut buffer = [0u8; 1];
    into_writer(&3u8, &mut buffer[..]).unwrap();
    assert_eq!(buffer[0], 3);
}

#[test]
fn serialize_slice_oos() {
    into_writer(&3u8, &mut [][..]).unwrap_err();
}

#[test]
fn serialize_big_u128() {
    let mut buf = Vec::new();
    into_writer(&18446744073709551616u128, &mut buf).unwrap();
    assert_eq!(buf, hex::decode("c249010000000000000000").unwrap());
}

#[test]
fn serialize_map() {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    map.insert(1u8, 2u8);
    map.insert(3u8, 4u8);
    let mut buf = Vec::new();
    into_writer(&map, &mut buf).unwrap();
    assert_eq!(buf, hex::decode("a201020304").unwrap());
}

#[test]
fn serialize_i128_small_shrinks() {
    let mut buf = Vec::new();
    into_writer(&42i128, &mut buf).unwrap();
    assert_eq!(buf, [0x18, 42]);
}

#[test]
fn serialize_u128_small_shrinks() {
    let mut buf = Vec::new();
    into_writer(&42u128, &mut buf).unwrap();
    assert_eq!(buf, [0x18, 42]);
}

// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::input::slice::Error as SliceError;
use ciborium_atom::*;
use flex::Flex;
use floats::f16;

/// Decode a hex string into an Atom, assert it matches expected, then re-encode
/// and verify the output matches the original hex.
fn roundtrip(hex_str: &str, expected: Atom) {
    let bytes = hex::decode(hex_str).unwrap();

    // Decode
    let mut input = bytes.as_slice();
    let decoded = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(decoded, expected, "decode mismatch for {hex_str}");
    assert!(input.is_empty(), "trailing bytes for {hex_str}");

    // Encode
    let mut encoded = Vec::new();
    decoded.encode(&mut encoded).unwrap();
    assert_eq!(
        hex::encode(&encoded),
        hex_str,
        "encode mismatch for {hex_str}"
    );
}

/// Decode hex and expect a specific error.
fn decode_err(hex_str: &str, expected: Error<SliceError>) {
    let bytes = hex::decode(hex_str).unwrap();
    let mut input = bytes.as_slice();
    let result = Atom::decode(&mut input);
    assert_eq!(result, Err(expected), "error mismatch for {hex_str}");
}

// ----------------------------------------------------------------
// Positive integers (major type 0)
// ----------------------------------------------------------------

#[test]
fn positive_inline() {
    roundtrip("00", Atom::Positive(Unsigned::U0(Short::new(0).unwrap())));
    roundtrip("01", Atom::Positive(Unsigned::U0(Short::new(1).unwrap())));
    roundtrip("0a", Atom::Positive(Unsigned::U0(Short::new(10).unwrap())));
    roundtrip("17", Atom::Positive(Unsigned::U0(Short::new(23).unwrap())));
}

#[test]
fn positive_u1() {
    roundtrip("1818", Atom::Positive(Unsigned::U1(24)));
    roundtrip("1819", Atom::Positive(Unsigned::U1(25)));
    roundtrip("1864", Atom::Positive(Unsigned::U1(100)));
    roundtrip("18ff", Atom::Positive(Unsigned::U1(255)));
}

#[test]
fn positive_u2() {
    roundtrip("190100", Atom::Positive(Unsigned::U2(256)));
    roundtrip("1903e8", Atom::Positive(Unsigned::U2(1000)));
    roundtrip("19ffff", Atom::Positive(Unsigned::U2(65535)));
}

#[test]
fn positive_u4() {
    roundtrip("1a00010000", Atom::Positive(Unsigned::U4(65536)));
    roundtrip("1a000f4240", Atom::Positive(Unsigned::U4(1_000_000)));
    roundtrip("1affffffff", Atom::Positive(Unsigned::U4(u32::MAX)));
}

#[test]
fn positive_u8() {
    roundtrip(
        "1b0000000100000000",
        Atom::Positive(Unsigned::U8(0x1_0000_0000)),
    );
    roundtrip(
        "1b000000e8d4a51000",
        Atom::Positive(Unsigned::U8(1_000_000_000_000)),
    );
    roundtrip("1bffffffffffffffff", Atom::Positive(Unsigned::U8(u64::MAX)));
}

// ----------------------------------------------------------------
// Non-minimal positive integers (wire size preserved)
// ----------------------------------------------------------------

#[test]
fn positive_non_minimal() {
    // Value 0 encoded with 1-byte argument (non-minimal but valid CBOR)
    roundtrip("1800", Atom::Positive(Unsigned::U1(0)));
    // Value 0 encoded with 2-byte argument
    roundtrip("190000", Atom::Positive(Unsigned::U2(0)));
    // Value 0 encoded with 4-byte argument
    roundtrip("1a00000000", Atom::Positive(Unsigned::U4(0)));
    // Value 0 encoded with 8-byte argument
    roundtrip("1b0000000000000000", Atom::Positive(Unsigned::U8(0)));
}

// ----------------------------------------------------------------
// Negative integers (major type 1)
// ----------------------------------------------------------------

#[test]
fn negative_inline() {
    roundtrip("20", Atom::Negative(Unsigned::U0(Short::new(0).unwrap())));
    roundtrip("29", Atom::Negative(Unsigned::U0(Short::new(9).unwrap())));
}

#[test]
fn negative_u1() {
    roundtrip("3863", Atom::Negative(Unsigned::U1(99)));
}

#[test]
fn negative_u2() {
    roundtrip("3903e7", Atom::Negative(Unsigned::U2(999)));
}

#[test]
fn negative_u8() {
    roundtrip("3bffffffffffffffff", Atom::Negative(Unsigned::U8(u64::MAX)));
}

// ----------------------------------------------------------------
// Byte strings (major type 2)
// ----------------------------------------------------------------

#[test]
fn bytes_empty() {
    roundtrip("40", Atom::Bytes(Some(Flex::Lend(&[]))));
}

#[test]
fn bytes_short() {
    roundtrip(
        "4401020304",
        Atom::Bytes(Some(Flex::Lend(&[0x01, 0x02, 0x03, 0x04]))),
    );
}

#[test]
fn bytes_indefinite() {
    roundtrip("5f", Atom::Bytes(None));
}

// ----------------------------------------------------------------
// Text strings (major type 3)
// ----------------------------------------------------------------

#[test]
fn text_empty() {
    roundtrip("60", Atom::Text(Some(Flex::Lend(""))));
}

#[test]
fn text_ascii() {
    roundtrip("6161", Atom::Text(Some(Flex::Lend("a"))));
    roundtrip("6449455446", Atom::Text(Some(Flex::Lend("IETF"))));
}

#[test]
fn text_utf8_multibyte() {
    roundtrip("62c3bc", Atom::Text(Some(Flex::Lend("\u{00fc}"))));
    roundtrip("63e6b0b4", Atom::Text(Some(Flex::Lend("\u{6c34}"))));
    roundtrip("64f0908591", Atom::Text(Some(Flex::Lend("\u{10151}"))));
}

#[test]
fn text_indefinite() {
    roundtrip("7f", Atom::Text(None));
}

#[test]
fn text_invalid_utf8() {
    let bytes = hex::decode("62c328").unwrap();
    let mut input = bytes.as_slice();
    match Atom::decode(&mut input) {
        Err(Error::Input(SliceError::Utf8(_))) => {}
        other => panic!("expected Utf8 error, got {other:?}"),
    }
}

// ----------------------------------------------------------------
// Arrays (major type 4)
// ----------------------------------------------------------------

#[test]
fn array_empty() {
    roundtrip(
        "80",
        Atom::Array(Some(Unsigned::U0(Short::new(0).unwrap()))),
    );
}

#[test]
fn array_definite() {
    roundtrip(
        "83",
        Atom::Array(Some(Unsigned::U0(Short::new(3).unwrap()))),
    );
}

#[test]
fn array_large() {
    roundtrip("9819", Atom::Array(Some(Unsigned::U1(25))));
}

#[test]
fn array_indefinite() {
    roundtrip("9f", Atom::Array(None));
}

// ----------------------------------------------------------------
// Maps (major type 5)
// ----------------------------------------------------------------

#[test]
fn map_empty() {
    roundtrip("a0", Atom::Map(Some(Unsigned::U0(Short::new(0).unwrap()))));
}

#[test]
fn map_definite() {
    roundtrip("a2", Atom::Map(Some(Unsigned::U0(Short::new(2).unwrap()))));
}

#[test]
fn map_indefinite() {
    roundtrip("bf", Atom::Map(None));
}

// ----------------------------------------------------------------
// Tags (major type 6)
// ----------------------------------------------------------------

#[test]
fn tag_small() {
    roundtrip("c0", Atom::Tag(Unsigned::U0(Short::new(0).unwrap())));
    roundtrip("c1", Atom::Tag(Unsigned::U0(Short::new(1).unwrap())));
    roundtrip("c6", Atom::Tag(Unsigned::U0(Short::new(6).unwrap())));
}

#[test]
fn tag_u1() {
    roundtrip("d818", Atom::Tag(Unsigned::U1(24)));
    roundtrip("d820", Atom::Tag(Unsigned::U1(32)));
}

#[test]
fn tag_bignum() {
    roundtrip("c2", Atom::Tag(Unsigned::U0(Short::new(2).unwrap())));
    roundtrip("c3", Atom::Tag(Unsigned::U0(Short::new(3).unwrap())));
}

// ----------------------------------------------------------------
// Simple values (major type 7)
// ----------------------------------------------------------------

#[test]
fn simple_false() {
    roundtrip(
        "f4",
        Atom::Other(Some(Other::Simple(Simple::S0(Short::new(20).unwrap())))),
    );
}

#[test]
fn simple_true() {
    roundtrip(
        "f5",
        Atom::Other(Some(Other::Simple(Simple::S0(Short::new(21).unwrap())))),
    );
}

#[test]
fn simple_null() {
    roundtrip(
        "f6",
        Atom::Other(Some(Other::Simple(Simple::S0(Short::new(22).unwrap())))),
    );
}

#[test]
fn simple_undefined() {
    roundtrip(
        "f7",
        Atom::Other(Some(Other::Simple(Simple::S0(Short::new(23).unwrap())))),
    );
}

#[test]
fn simple_one_byte() {
    roundtrip("f818", Atom::Other(Some(Other::Simple(Simple::S1(24)))));
    roundtrip("f8ff", Atom::Other(Some(Other::Simple(Simple::S1(255)))));
}

#[test]
fn simple_non_minimal() {
    // Simple value 0 encoded with 1-byte argument (non-minimal)
    roundtrip("f800", Atom::Other(Some(Other::Simple(Simple::S1(0)))));
}

// ----------------------------------------------------------------
// Floats (major type 7)
// ----------------------------------------------------------------

#[test]
fn float_f2_zero() {
    roundtrip(
        "f90000",
        Atom::Other(Some(Other::Float(Float::F2(f16::from_bits(0x0000))))),
    );
}

#[test]
fn float_f2_neg_zero() {
    roundtrip(
        "f98000",
        Atom::Other(Some(Other::Float(Float::F2(f16::from_bits(0x8000))))),
    );
}

#[test]
fn float_f2_one() {
    roundtrip(
        "f93c00",
        Atom::Other(Some(Other::Float(Float::F2(f16::from_bits(0x3c00))))),
    );
}

#[test]
fn float_f2_one_point_five() {
    roundtrip(
        "f93e00",
        Atom::Other(Some(Other::Float(Float::F2(f16::from_bits(0x3e00))))),
    );
}

#[test]
fn float_f2_max() {
    roundtrip(
        "f97bff",
        Atom::Other(Some(Other::Float(Float::F2(f16::from_bits(0x7bff))))),
    );
}

#[test]
fn float_f2_infinity() {
    roundtrip(
        "f97c00",
        Atom::Other(Some(Other::Float(Float::F2(f16::from_bits(0x7c00))))),
    );
}

#[test]
fn float_f2_neg_infinity() {
    roundtrip(
        "f9fc00",
        Atom::Other(Some(Other::Float(Float::F2(f16::from_bits(0xfc00))))),
    );
}

#[test]
fn float_f2_nan() {
    let bytes = hex::decode("f97e00").unwrap();

    let mut input = bytes.as_slice();
    let decoded = Atom::decode(&mut input).unwrap().unwrap();
    assert!(input.is_empty());
    match decoded {
        Atom::Other(Some(Other::Float(Float::F2(v)))) => {
            assert_eq!(v.to_bits(), 0x7e00, "NaN bits mismatch");
        }
        other => panic!("expected f16 NaN, got {other:?}"),
    }

    let mut encoded = Vec::new();
    decoded.encode(&mut encoded).unwrap();
    assert_eq!(hex::encode(&encoded), "f97e00");
}

#[test]
fn float_f4() {
    roundtrip(
        "fa47c35000",
        Atom::Other(Some(Other::Float(Float::F4(100000.0)))),
    );
    roundtrip(
        "fa7f7fffff",
        Atom::Other(Some(Other::Float(Float::F4(f32::MAX)))),
    );
}

#[test]
fn float_f4_infinity() {
    roundtrip(
        "fa7f800000",
        Atom::Other(Some(Other::Float(Float::F4(f32::INFINITY)))),
    );
}

#[test]
fn float_f4_neg_infinity() {
    roundtrip(
        "faff800000",
        Atom::Other(Some(Other::Float(Float::F4(f32::NEG_INFINITY)))),
    );
}

#[test]
fn float_f8() {
    roundtrip(
        "fb3ff199999999999a",
        Atom::Other(Some(Other::Float(Float::F8(1.1)))),
    );
    roundtrip(
        "fb7e37e43c8800759c",
        Atom::Other(Some(Other::Float(Float::F8(1.0e+300)))),
    );
}

#[test]
fn float_f8_neg() {
    roundtrip(
        "fbc010666666666666",
        Atom::Other(Some(Other::Float(Float::F8(-4.1)))),
    );
}

// ----------------------------------------------------------------
// Break (major type 7, info 31)
// ----------------------------------------------------------------

#[test]
fn break_code() {
    roundtrip("ff", Atom::Other(None));
}

// ----------------------------------------------------------------
// End of stream
// ----------------------------------------------------------------

#[test]
fn empty_input() {
    let bytes: &[u8] = &[];
    let mut input = bytes;
    let result = Atom::decode(&mut input).unwrap();
    assert_eq!(result, None);
}

// ----------------------------------------------------------------
// Error cases
// ----------------------------------------------------------------

#[test]
fn invalid_additional_info() {
    decode_err("1c", Error::Invalid);
    decode_err("1d", Error::Invalid);
    decode_err("1e", Error::Invalid);
    decode_err("3c", Error::Invalid);
    decode_err("5c", Error::Invalid);
    decode_err("7c", Error::Invalid);
    decode_err("9c", Error::Invalid);
    decode_err("bc", Error::Invalid);
    decode_err("dc", Error::Invalid);
    decode_err("fc", Error::Invalid);
}

#[test]
fn truncated_u1_argument() {
    decode_err("18", Error::Input(SliceError::Underflow));
}

#[test]
fn truncated_u2_argument() {
    decode_err("1900", Error::Input(SliceError::Underflow));
}

#[test]
fn truncated_u4_argument() {
    decode_err("1a0000", Error::Input(SliceError::Underflow));
}

#[test]
fn truncated_u8_argument() {
    decode_err("1b00000000", Error::Input(SliceError::Underflow));
}

#[test]
fn truncated_bytes_payload() {
    decode_err("440102", Error::Input(SliceError::Underflow));
}

#[test]
fn truncated_text_payload() {
    decode_err("644142", Error::Input(SliceError::Underflow));
}

#[test]
fn truncated_f2() {
    decode_err("f900", Error::Input(SliceError::Underflow));
}

#[test]
fn truncated_f4() {
    decode_err("fa000000", Error::Input(SliceError::Underflow));
}

#[test]
fn truncated_f8() {
    decode_err("fb00000000000000", Error::Input(SliceError::Underflow));
}

// ----------------------------------------------------------------
// Sequential decoding
// ----------------------------------------------------------------

#[test]
fn decode_multiple_atoms() {
    let bytes = hex::decode("0102").unwrap();
    let mut input = bytes.as_slice();

    let first = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(first, Atom::Positive(Unsigned::U0(Short::new(1).unwrap())));

    let second = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(second, Atom::Positive(Unsigned::U0(Short::new(2).unwrap())));

    let end = Atom::decode(&mut input).unwrap();
    assert_eq!(end, None);
}

#[test]
fn decode_array_then_elements() {
    let bytes = hex::decode("83010203").unwrap();
    let mut input = bytes.as_slice();

    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Array(Some(Unsigned::U0(Short::new(3).unwrap())))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(1).unwrap()))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(2).unwrap()))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(3).unwrap()))
    );
    assert_eq!(Atom::decode(&mut input).unwrap(), None);
}

#[test]
fn decode_indefinite_array() {
    let bytes = hex::decode("9f010203ff").unwrap();
    let mut input = bytes.as_slice();

    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Array(None)
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(1).unwrap()))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(2).unwrap()))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(3).unwrap()))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Other(None)
    );
    assert_eq!(Atom::decode(&mut input).unwrap(), None);
}

#[test]
fn decode_map_with_string_keys() {
    let bytes = hex::decode("a26161016162820203").unwrap();
    let mut input = bytes.as_slice();

    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Map(Some(Unsigned::U0(Short::new(2).unwrap())))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Text(Some(Flex::Lend("a")))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(1).unwrap()))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Text(Some(Flex::Lend("b")))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Array(Some(Unsigned::U0(Short::new(2).unwrap())))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(2).unwrap()))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U0(Short::new(3).unwrap()))
    );
    assert_eq!(Atom::decode(&mut input).unwrap(), None);
}

#[test]
fn decode_tagged_value() {
    let bytes = hex::decode("c11a514b67b0").unwrap();
    let mut input = bytes.as_slice();

    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Tag(Unsigned::U0(Short::new(1).unwrap()))
    );
    assert_eq!(
        Atom::decode(&mut input).unwrap().unwrap(),
        Atom::Positive(Unsigned::U4(1363896240))
    );
    assert_eq!(Atom::decode(&mut input).unwrap(), None);
}

// ----------------------------------------------------------------
// Unsigned conversion tests
// ----------------------------------------------------------------

#[test]
fn unsigned_from_minimizes() {
    assert_eq!(Unsigned::from(0u8), Unsigned::U0(Short::new(0).unwrap()));
    assert_eq!(Unsigned::from(23u8), Unsigned::U0(Short::new(23).unwrap()));
    assert_eq!(Unsigned::from(24u8), Unsigned::U1(24));
    assert_eq!(Unsigned::from(255u64), Unsigned::U1(255));
    assert_eq!(Unsigned::from(256u64), Unsigned::U2(256));
    assert_eq!(Unsigned::from(65535u64), Unsigned::U2(65535));
    assert_eq!(Unsigned::from(65536u64), Unsigned::U4(65536));
    assert_eq!(Unsigned::from(u32::MAX as u64), Unsigned::U4(u32::MAX));
    assert_eq!(
        Unsigned::from(u32::MAX as u64 + 1),
        Unsigned::U8(u32::MAX as u64 + 1)
    );
    assert_eq!(Unsigned::from(u64::MAX), Unsigned::U8(u64::MAX));
}

#[test]
fn unsigned_to_u64() {
    assert_eq!(u64::from(Unsigned::U0(Short::new(10).unwrap())), 10u64);
    assert_eq!(u64::from(Unsigned::U1(42)), 42u64);
    assert_eq!(u64::from(Unsigned::U2(1000)), 1000u64);
    assert_eq!(u64::from(Unsigned::U4(100000)), 100000u64);
    assert_eq!(u64::from(Unsigned::U8(u64::MAX)), u64::MAX);
}

#[test]
fn unsigned_try_from() {
    assert_eq!(
        u8::try_from(Unsigned::U0(Short::new(10).unwrap())),
        Ok(10u8)
    );
    assert_eq!(u8::try_from(Unsigned::U1(42)), Ok(42u8));
    assert!(u8::try_from(Unsigned::U2(256)).is_err());

    assert_eq!(u16::try_from(Unsigned::U2(1000)), Ok(1000u16));
    assert!(u16::try_from(Unsigned::U4(70000)).is_err());

    assert_eq!(u32::try_from(Unsigned::U4(100000)), Ok(100000u32));
    assert!(u32::try_from(Unsigned::U8(u64::MAX)).is_err());
}

// ----------------------------------------------------------------
// Short tests
// ----------------------------------------------------------------

#[test]
fn short_valid() {
    for i in 0..24 {
        let s = Short::new(i).unwrap();
        assert_eq!(s.get(), i);
    }
}

#[test]
fn short_invalid() {
    for i in 24..=255 {
        assert_eq!(Short::new(i), None);
    }
}

// ----------------------------------------------------------------
// Float conversion tests
// ----------------------------------------------------------------

#[test]
fn float_from_minimizes() {
    assert!(matches!(Float::from(0.0f64), Float::F2(_)));
    assert!(matches!(Float::from(1.0f64), Float::F2(_)));
    assert!(matches!(Float::from(-1.0f64), Float::F2(_)));
    assert!(matches!(Float::from(f64::INFINITY), Float::F2(_)));
    assert!(matches!(Float::from(f64::NEG_INFINITY), Float::F2(_)));
    assert!(matches!(Float::from(f64::NAN), Float::F2(_)));

    assert!(matches!(Float::from(100000.0f64), Float::F4(_)));

    assert!(matches!(Float::from(1.0e+300f64), Float::F8(_)));
    assert!(matches!(Float::from(1.1f64), Float::F8(_)));
}

#[test]
fn float_from_f32_minimizes() {
    assert!(matches!(Float::from(0.0f32), Float::F2(_)));
    assert!(matches!(Float::from(1.0f32), Float::F2(_)));
    assert!(matches!(Float::from(f32::NAN), Float::F2(_)));

    assert!(matches!(Float::from(100000.0f32), Float::F4(_)));
}

#[test]
fn float_to_f64() {
    let half = Float::F2(f16::from_bits(0x3c00));
    assert_eq!(f64::from(half), 1.0f64);

    let single = Float::F4(100000.0);
    assert_eq!(f64::from(single), 100000.0f64);

    let double = Float::F8(1.1);
    assert_eq!(f64::from(double), 1.1f64);
}

#[test]
fn float_try_from() {
    assert!(f16::try_from(Float::F2(f16::from_bits(0x3c00))).is_ok());
    let err = f16::try_from(Float::F4(1.0));
    assert_eq!(err, Err(Float::F4(1.0)));

    assert!(f32::try_from(Float::F2(f16::from_bits(0x3c00))).is_ok());
    assert!(f32::try_from(Float::F4(1.0)).is_ok());
    let err = f32::try_from(Float::F8(1.0));
    assert_eq!(err, Err(Float::F8(1.0)));
}

// ----------------------------------------------------------------
// Wire size preservation
// ----------------------------------------------------------------

#[test]
fn wire_size_preserved_unsigned() {
    let mut buf = Vec::new();
    Atom::Positive(Unsigned::U2(0)).encode(&mut buf).unwrap();
    assert_eq!(buf, hex::decode("190000").unwrap());
}

#[test]
fn wire_size_preserved_float() {
    let mut buf = Vec::new();
    Atom::Other(Some(Other::Float(Float::F8(1.0)))).encode(&mut buf).unwrap();
    assert_eq!(buf, hex::decode("fb3ff0000000000000").unwrap());
}

// ----------------------------------------------------------------
// Encode-only edge cases
// ----------------------------------------------------------------

#[test]
fn encode_bytes_length() {
    let mut buf = Vec::new();
    Atom::Bytes(Some(Flex::Lend(&[]))).encode(&mut buf).unwrap();
    assert_eq!(buf, &[0x40]);

    buf.clear();
    Atom::Bytes(Some(Flex::Lend(&[1, 2, 3, 4]))).encode(&mut buf).unwrap();
    assert_eq!(buf, &[0x44, 1, 2, 3, 4]);
}

#[test]
fn encode_text_length() {
    let mut buf = Vec::new();
    Atom::Text(Some(Flex::Lend("IETF"))).encode(&mut buf).unwrap();
    assert_eq!(buf, b"\x64IETF");
}

#[test]
fn encode_all_indefinite_markers() {
    let cases = [
        ("5f", Atom::Bytes(None)),
        ("7f", Atom::Text(None)),
        ("9f", Atom::Array(None)),
        ("bf", Atom::Map(None)),
        ("ff", Atom::Other(None)),
    ];
    for (hex_str, atom) in &cases {
        let mut buf = Vec::new();
        atom.encode(&mut buf).unwrap();
        assert_eq!(hex::encode(&buf), *hex_str);
    }
}

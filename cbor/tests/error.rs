// SPDX-License-Identifier: Apache-2.0

use cbor::{
    de::{from_reader, Error},
    ser::into_writer,
    value::Value,
};
use rstest::rstest;

fn io() -> Error<std::io::Error> {
    Error::Io(std::io::ErrorKind::UnexpectedEof.into())
}

fn expected() -> Error<std::io::Error> {
    Error::Expected(cbor::de::Expected {
        expected: cbor::de::Class::Atom,
        received: None,
    })
}

fn custom(msg: &str) -> Error<std::io::Error> {
    Error::Custom(msg.into())
}

#[rstest(bytes, error,
    // Invalid value
    case("1e", io()),

    // Indeterminate integers are invalid
    case("1f", io()),

    // Indeterminate integer in an array
    case("83011f03", io()),

    // Integer in a string continuation
    case("7F616101FF", io()),

    // Bytes in a string continuation
    case("7F61614101FF", io()),

    // Invalid UTF-8
    case("62C328", io()),

    // Invalid UTF-8 in a string continuation
    case("7F62C328FF", io()),

    // End of input in a head
    case("18", io()),
    case("19", io()),
    case("1a", io()),
    case("1b", io()),
    case("1901", io()),
    case("1a0102", io()),
    case("1b01020304050607", io()),
    case("38", io()),
    case("58", io()),
    case("78", io()),
    case("98", io()),
    case("9a01ff00", io()),
    case("b8", io()),
    case("d8", io()),
    case("f8", io()),
    case("f900", io()),
    case("fa0000", io()),
    case("fb000000", io()),

    // Definite-length strings with short data:
    case("41", io()),
    case("61", io()),
    case("5affffffff00", io()),
    case("5bffffffffffffffff010203", io()),
    case("7affffffff00", io()),
    case("7b7fffffffffffffff010203", io()),

    // Definite-length maps and arrays not closed with enough items:
    case("81", expected()),
    case("818181818181818181", expected()),
    case("8200", expected()),
    case("a1", expected()),
    case("a20102", expected()),
    case("a100", expected()),
    case("a2000000", expected()),

    // Tag number not followed by tag content:
    case("c0", expected()),

    // Indefinite-length strings not closed by a "break" stop code:
    case("5f4100", expected()),
    case("7f6100", expected()),

    // Indefinite-length maps and arrays not closed by a "break" stop code:
    case("9f", expected()),
    case("9f0102", expected()),
    case("bf", expected()),
    case("bf01020102", expected()),
    case("819f", expected()),
    case("9f8000", expected()),
    case("9f9f9f9f9fffffffff", expected()),
    case("9f819f819f9fffffff", expected()),

    // Reserved additional information values:
    case("1c", io()),
    case("1d", io()),
    case("1e", io()),
    case("3c", io()),
    case("3d", io()),
    case("3e", io()),
    case("5c", io()),
    case("5d", io()),
    case("5e", io()),
    case("7c", io()),
    case("7d", io()),
    case("7e", io()),
    case("9c", io()),
    case("9d", io()),
    case("9e", io()),
    case("bc", io()),
    case("bd", io()),
    case("be", io()),
    case("dc", io()),
    case("dd", io()),
    case("de", io()),
    case("fc", io()),
    case("fd", io()),
    case("fe", io()),

    // Reserved two-byte encodings of simple values:
    case("f800", custom("invalid type: simple, expected known simple value")),
    case("f801", custom("invalid type: simple, expected known simple value")),
    case("f818", custom("invalid type: simple, expected known simple value")),
    case("f81f", custom("invalid type: simple, expected known simple value")),

    // Indefinite-length string chunks not of the correct type:
    case("5f00ff", io()),
    case("5f21ff", io()),
    case("5f6100ff", io()),
    case("5f80ff", io()),
    case("5fa0ff", io()),
    case("5fc000ff", io()),
    case("5fe0ff", io()),
    case("7f4100ff", io()),

    // Indefinite-length string chunks not definite length:
    case("5f5f4100ffff", io()),
    case("7f7f6100ffff", io()),

    // Break occurring on its own outside of an indefinite-length item:
    case("ff", custom("invalid type: break, expected non-break")),

    // Break occurring in a definite-length array or map or a tag:
    case("81ff", custom("invalid type: break, expected non-break")),
    case("8200ff", custom("invalid type: break, expected non-break")),
    case("a1ff", custom("invalid type: break, expected non-break")),
    case("a1ff00", custom("invalid type: break, expected non-break")),
    case("a100ff", custom("invalid type: break, expected non-break")),
    case("a20000ff", custom("invalid type: break, expected non-break")),
    case("9f81ff", custom("invalid type: break, expected non-break")),
    case("9f829f819f9fffffffff", custom("invalid type: break, expected non-break")),

    // Break in an indefinite-length map that would lead to an odd number of items (break in a value position):
    case("bf00ff", custom("invalid type: break, expected non-break")),
    case("bf000000ff", custom("invalid type: break, expected non-break")),

    // Major type 0, 1, 6 with additional information 31:
    case("1f", io()),
    case("3f", io()),
    case("df", io()),
)]
fn test(bytes: &str, error: Error<std::io::Error>) {
    let bytes = hex::decode(bytes).unwrap();

    let correct = match error {
        Error::Io(..) => "io",
        Error::Expected(..) => "expected",
        Error::Custom(ref s) => if s.is_empty() { "custom" } else { "custom" },
        Error::RecursionLimitExceeded => panic!(),
    };
    // For Custom errors, keep the message for comparison
    let correct_msg = match &error {
        Error::Custom(s) => Some(s.clone()),
        _ => None,
    };

    let result: Result<Value, _> = from_reader(dbg!(&bytes[..]));
    let err = dbg!(result.unwrap_err());
    let actual = match &err {
        Error::Io(..) => "io",
        Error::Expected(..) => "expected",
        Error::Custom(..) => "custom",
        Error::RecursionLimitExceeded => panic!(),
    };
    let actual_msg = match &err {
        Error::Custom(s) => Some(s.clone()),
        _ => None,
    };

    assert_eq!(correct, actual, "error kind mismatch");
    if correct_msg.is_some() {
        assert_eq!(correct_msg, actual_msg, "error message mismatch");
    }
}

#[test]
fn test_long_utf8_deserialization() {
    let s = (0..2000).map(|_| 'ボ').collect::<String>();
    let mut v = Vec::new();
    into_writer(&s, &mut v).unwrap();
    let _: String = from_reader(&*v).unwrap();
}

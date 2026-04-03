// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "std")]

use std::io::Cursor;

use ciborium_atom::input::read::{Error as ReadError, Reader};
use ciborium_atom::*;

// ----------------------------------------------------------------
// Reader basic decoding
// ----------------------------------------------------------------

#[test]
fn reader_decode_positive() {
    let data = hex::decode("0a").unwrap();
    let mut input = Reader::from(Cursor::new(data));

    let atom = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(atom, Atom::Positive(Unsigned::U0(Short::new(10).unwrap())));
}

#[test]
fn reader_decode_text() {
    let data = hex::decode("6449455446").unwrap();
    let mut input = Reader::from(Cursor::new(data));

    let atom = Atom::decode(&mut input).unwrap().unwrap();
    match atom {
        Atom::Text(Some(flex)) => assert_eq!(&*flex, "IETF"),
        other => panic!("expected text, got {other:?}"),
    }
}

#[test]
fn reader_decode_bytes() {
    let data = hex::decode("4401020304").unwrap();
    let mut input = Reader::from(Cursor::new(data));

    let atom = Atom::decode(&mut input).unwrap().unwrap();
    match atom {
        Atom::Bytes(Some(flex)) => assert_eq!(&*flex, &[1, 2, 3, 4]),
        other => panic!("expected bytes, got {other:?}"),
    }
}

#[test]
fn reader_end_of_stream() {
    let mut input = Reader::from(Cursor::new(Vec::new()));

    let result = Atom::decode(&mut input).unwrap();
    assert_eq!(result, None);
}

#[test]
fn reader_sequential() {
    let data = hex::decode("0102").unwrap();
    let mut input = Reader::from(Cursor::new(data));

    let first = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(first, Atom::Positive(Unsigned::U0(Short::new(1).unwrap())));

    let second = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(second, Atom::Positive(Unsigned::U0(Short::new(2).unwrap())));

    assert_eq!(Atom::decode(&mut input).unwrap(), None);
}

// ----------------------------------------------------------------
// Reader error cases
// ----------------------------------------------------------------

#[test]
fn reader_truncated_body() {
    // u16 argument but only 1 byte follows — body() fails
    let data = hex::decode("1900").unwrap();
    let mut input = Reader::from(Cursor::new(data));

    match Atom::decode(&mut input) {
        Err(Error::Input(ReadError::Io(_))) => {}
        other => panic!("expected Io error, got {other:?}"),
    }
}

#[test]
fn reader_head_io_error() {
    use ciborium_atom::input::Input;

    // A reader that always errors
    struct FailReader;
    impl std::io::Read for FailReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "fail"))
        }
    }

    let mut input = Reader::from(FailReader);
    match input.head() {
        Err(ReadError::Io(e)) => assert_eq!(e.kind(), std::io::ErrorKind::BrokenPipe),
        other => panic!("expected Io error, got {other:?}"),
    }
}

#[test]
fn reader_invalid_utf8() {
    // text of length 2 with invalid utf8 bytes
    let data = hex::decode("62c328").unwrap();
    let mut input = Reader::from(Cursor::new(data));

    match Atom::decode(&mut input) {
        Err(Error::Input(ReadError::Utf8(_))) => {}
        other => panic!("expected Utf8 error, got {other:?}"),
    }
}

// ----------------------------------------------------------------
// ReadError Display and Error trait
// ----------------------------------------------------------------

#[test]
fn read_error_display() {
    let io_err = ReadError::Io(std::io::Error::new(
        std::io::ErrorKind::UnexpectedEof,
        "eof",
    ));
    assert!(io_err.to_string().starts_with("I/O error:"));

    let utf8_err = String::from_utf8(vec![0xc3, 0x28]).unwrap_err();
    let err = ReadError::Utf8(utf8_err);
    assert!(err.to_string().starts_with("invalid UTF-8:"));
}

#[test]
fn read_error_source() {
    use std::error::Error;

    let io_err = ReadError::Io(std::io::Error::new(
        std::io::ErrorKind::UnexpectedEof,
        "eof",
    ));
    assert!(io_err.source().is_some());

    let utf8_err = String::from_utf8(vec![0xc3, 0x28]).unwrap_err();
    let err = ReadError::Utf8(utf8_err);
    assert!(err.source().is_some());
}

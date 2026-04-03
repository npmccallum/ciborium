// SPDX-License-Identifier: Apache-2.0

use ciborium_atom::input::slice::Error as SliceError;
use ciborium_atom::input::span::Span;
use ciborium_atom::*;
use flex::Flex;

// ----------------------------------------------------------------
// Span tracking
// ----------------------------------------------------------------

#[test]
fn span_tracks_simple_atom() {
    let bytes = hex::decode("0a").unwrap();
    let mut input = Span::from(bytes.as_slice());

    let atom = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(atom, Atom::Positive(Unsigned::U0(Short::new(10).unwrap())));
    assert_eq!(input.span(), 0..1);
}

#[test]
fn span_tracks_multibyte_head() {
    let bytes = hex::decode("1903e8").unwrap();
    let mut input = Span::from(bytes.as_slice());

    let atom = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(atom, Atom::Positive(Unsigned::U2(1000)));
    assert_eq!(input.span(), 0..3);
}

#[test]
fn span_tracks_text_with_payload() {
    let bytes = hex::decode("6449455446").unwrap();
    let mut input = Span::from(bytes.as_slice());

    let atom = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(atom, Atom::Text(Some(Flex::Lend("IETF"))));
    assert_eq!(input.span(), 0..5);
}

#[test]
fn span_tracks_sequential_atoms() {
    let bytes = hex::decode("0102").unwrap();
    let mut input = Span::from(bytes.as_slice());

    Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(input.span(), 0..1);

    Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(input.span(), 1..2);
}

#[test]
fn span_end_of_stream() {
    let bytes: &[u8] = &[];
    let mut input = Span::from(bytes);

    let result = Atom::decode(&mut input).unwrap();
    assert_eq!(result, None);
    assert_eq!(input.span(), 0..0);
}

#[test]
fn span_tracks_bytes_payload() {
    let bytes = hex::decode("4401020304").unwrap();
    let mut input = Span::from(bytes.as_slice());

    Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(input.span(), 0..5);
}

#[test]
fn span_tracks_utf8_error() {
    // 0x01 = positive(1), then 0x62 0xc3 0x28 = text(2) with invalid UTF-8
    let bytes = hex::decode("0162c328").unwrap();
    let mut input = Span::from(bytes.as_slice());

    // First atom succeeds
    Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(input.span(), 0..1);

    // Second atom fails on UTF-8 — span starts at the text atom's head byte.
    // The read position only reflects successfully tracked bytes (head byte
    // was read, but text() failed before Span could update).
    let err = Atom::decode(&mut input);
    assert!(err.is_err());
    assert_eq!(input.span(), 1..2);
}

// ----------------------------------------------------------------
// &mut T blanket impl
// ----------------------------------------------------------------

#[test]
fn blanket_mut_ref_forwards() {
    let bytes = hex::decode("83010203").unwrap();
    let mut input = bytes.as_slice();

    let atom = Atom::decode(&mut &mut input).unwrap().unwrap();
    assert_eq!(
        atom,
        Atom::Array(Some(Unsigned::U0(Short::new(3).unwrap())))
    );

    let atom = Atom::decode(&mut input).unwrap().unwrap();
    assert_eq!(atom, Atom::Positive(Unsigned::U0(Short::new(1).unwrap())));
}

#[test]
fn blanket_mut_ref_tail_and_text() {
    let bytes = hex::decode("42010262616200").unwrap();
    let mut input = bytes.as_slice();
    let mut ref_input = &mut input;

    let atom = Atom::decode(&mut ref_input).unwrap().unwrap();
    assert_eq!(atom, Atom::Bytes(Some(Flex::Lend(&[0x01, 0x02]))));

    let atom = Atom::decode(&mut ref_input).unwrap().unwrap();
    assert_eq!(atom, Atom::Text(Some(Flex::Lend("ab"))));
}

// ----------------------------------------------------------------
// Error Display and Error trait
// ----------------------------------------------------------------

#[test]
#[allow(invalid_from_utf8)]
fn slice_error_display() {
    assert_eq!(SliceError::Underflow.to_string(), "not enough bytes");

    let bad = core::str::from_utf8(&[0xc3, 0x28]).unwrap_err();
    let err = SliceError::Utf8(bad);
    assert!(err.to_string().starts_with("invalid UTF-8:"));
}

#[test]
#[allow(invalid_from_utf8)]
fn slice_error_source() {
    use core::error::Error as _;

    assert!(SliceError::Underflow.source().is_none());

    let bad = core::str::from_utf8(&[0xc3, 0x28]).unwrap_err();
    let err = SliceError::Utf8(bad);
    assert!(err.source().is_some());
}

#[test]
fn decode_error_display() {
    let input_err: Error<SliceError> = Error::Input(SliceError::Underflow);
    assert_eq!(input_err.to_string(), "input error: not enough bytes");

    let invalid: Error<SliceError> = Error::Invalid;
    assert_eq!(invalid.to_string(), "invalid additional info");

    let overflow: Error<SliceError> = Error::Overflow;
    assert_eq!(overflow.to_string(), "length overflow");
}

#[test]
fn decode_error_source() {
    use core::error::Error as _;

    let input_err: ciborium_atom::Error<SliceError> =
        ciborium_atom::Error::Input(SliceError::Underflow);
    assert!(input_err.source().is_some());

    let invalid: ciborium_atom::Error<SliceError> = ciborium_atom::Error::Invalid;
    assert!(invalid.source().is_none());

    let overflow: ciborium_atom::Error<SliceError> = ciborium_atom::Error::Overflow;
    assert!(overflow.source().is_none());
}

// SPDX-License-Identifier: Apache-2.0

//! Fuzz target: decode atoms from arbitrary bytes, then re-encode and verify
//! that encoding produces the same bytes that were consumed.
//!
//! Note: Bytes/Text lengths are always re-encoded minimally, so we skip
//! the byte-exact comparison for those variants and verify only that the
//! payload is preserved.

#![no_main]

use ciborium_atom::Atom;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut input = data;

    loop {
        let before = input.len();
        match Atom::decode(&mut input) {
            Ok(Some(atom)) => {
                let consumed = before - input.len();
                let original = &data[data.len() - before..data.len() - before + consumed];

                let (head, tail) = atom.encode();
                let mut encoded = Vec::new();
                encoded.extend_from_slice(&head);
                encoded.extend_from_slice(tail);

                match &atom {
                    // Bytes/Text lengths are derived from payload and always
                    // minimized, so a byte-exact roundtrip is not guaranteed.
                    // Verify the payload content matches instead.
                    Atom::Bytes(Some(_)) | Atom::Text(Some(_)) => {
                        assert_eq!(tail, &original[original.len() - tail.len()..]);
                    }

                    _ => {
                        assert_eq!(
                            &encoded[..], original,
                            "roundtrip mismatch for atom {atom:?}"
                        );
                    }
                }
            }
            Ok(None) => break,
            Err(_) => break,
        }
    }
});

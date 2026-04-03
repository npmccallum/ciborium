// SPDX-License-Identifier: Apache-2.0

//! Fuzz target: feed arbitrary bytes to Atom::decode and ensure it never panics.

#![no_main]

use ciborium_atom::Atom;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut input = data;

    // Decode as many atoms as possible from the input.
    // We only care that it doesn't panic — Ok and Err are both fine.
    loop {
        match Atom::decode(&mut input) {
            Ok(Some(_)) => continue,
            Ok(None) => break,
            Err(_) => break,
        }
    }
});

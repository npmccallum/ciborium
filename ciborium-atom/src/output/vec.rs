// SPDX-License-Identifier: Apache-2.0

//! Output implementation for `Vec<u8>`.

#![cfg(feature = "alloc")]

extern crate alloc;

use alloc::vec::Vec;

use super::Output;

impl Output for Vec<u8> {
    type Error = core::convert::Infallible;

    fn write(&mut self, head: u8, body: &[u8], tail: &[u8]) -> Result<(), Self::Error> {
        self.push(head);
        self.extend_from_slice(body);
        self.extend_from_slice(tail);
        Ok(())
    }
}

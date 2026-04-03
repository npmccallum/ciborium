// SPDX-License-Identifier: Apache-2.0

/// A small stack buffer holding the encoded CBOR head (major type + argument).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Head {
    buf: [u8; 9],
    len: usize,
}

impl Head {
    #[inline]
    pub(crate) fn new0(first: u8) -> Self {
        Self {
            buf: [first, 0, 0, 0, 0, 0, 0, 0, 0],
            len: 1,
        }
    }

    #[inline]
    pub(crate) fn new1(first: u8, arg: [u8; 1]) -> Self {
        Self {
            buf: [first, arg[0], 0, 0, 0, 0, 0, 0, 0],
            len: 2,
        }
    }

    #[inline]
    pub(crate) fn new2(first: u8, arg: [u8; 2]) -> Self {
        Self {
            buf: [first, arg[0], arg[1], 0, 0, 0, 0, 0, 0],
            len: 3,
        }
    }

    #[inline]
    pub(crate) fn new4(first: u8, arg: [u8; 4]) -> Self {
        Self {
            buf: [first, arg[0], arg[1], arg[2], arg[3], 0, 0, 0, 0],
            len: 5,
        }
    }

    #[inline]
    pub(crate) fn new8(first: u8, arg: [u8; 8]) -> Self {
        Self {
            buf: [
                first, arg[0], arg[1], arg[2], arg[3], arg[4], arg[5], arg[6], arg[7],
            ],
            len: 9,
        }
    }
}

impl core::ops::Deref for Head {
    type Target = [u8];

    #[inline]
    #[allow(clippy::indexing_slicing)] // len is always 1-9, buf is [u8; 9]
    fn deref(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

#![allow(dead_code)]

use crate::empty_cipher::EmptyKem;

pub type KemType = EmptyKem;
pub const PUBLIC_KEY: &[u8] = &[];
pub const SECRET_KEY: &[u8] = &[];
pub const ENCRYPTED_MESSAGE: &[u8] = &[
    81, 222, 100, 112, 141, 90, 105, 47, 243, 178, 97, 115, 174, 231, 96, 8, 47, 108, 198, 194,
    229, 59, 28, 115, 26, 102, 134, 159, 12,
];

#![allow(dead_code)]

pub type KemType = hpke::kem::X25519HkdfSha256;
pub const SECRET_KEY: &[u8] = &[
    24, 94, 40, 181, 62, 205, 140, 17, 139, 148, 248, 157, 17, 253, 6, 219, 176, 241, 97, 245, 18,
    22, 208, 122, 79, 251, 26, 21, 199, 63, 115, 95,
];
pub const PUBLIC_KEY: &[u8] = &[
    94, 63, 124, 29, 39, 11, 38, 85, 140, 250, 65, 38, 63, 131, 166, 248, 119, 5, 44, 210, 22, 11,
    220, 213, 47, 94, 29, 216, 123, 208, 155, 4,
];
pub const ENCRYPTED_MESSAGE: &[u8] = &[
    39, 81, 204, 216, 195, 255, 146, 83, 107, 57, 25, 137, 143, 178, 155, 120, 128, 14, 45, 208,
    68, 110, 97, 17, 152, 141, 164, 151, 209, 207, 74, 75, 247, 111, 152, 232, 54, 120, 112, 105,
    188, 194, 83, 215, 160, 109, 18, 226, 39, 76, 122, 253, 148, 37, 6, 112, 45, 9, 97, 84, 196,
];

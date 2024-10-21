#![allow(dead_code)]

pub type KemType = hpke::kem::xyber768_oqs::X25519Kyber768;
pub const PUBLIC_KEY: &[u8] = &[
    120, 40, 31, 187, 118, 18, 76, 239, 28, 181, 204, 204, 195, 240, 237, 239, 219, 14, 88, 232,
    26, 208, 61, 104, 230, 194, 44, 89, 122, 155, 53, 36, 197, 106, 5, 186, 150, 54, 108, 193, 9,
    10, 245, 153, 187, 65, 39, 136, 16, 199, 24, 210, 87, 126, 150, 147, 196, 96, 143, 176, 82, 80,
    214, 119, 115, 88, 180, 96, 72, 202, 203, 52, 44, 118, 35, 88, 31, 51, 51, 187, 233, 180, 36,
    45, 91, 63, 36, 132, 80, 160, 19, 200, 115, 149, 4, 216, 55, 82, 26, 103, 43, 245, 82, 189, 91,
    6, 87, 180, 170, 105, 68, 24, 163, 23, 162, 82, 255, 55, 187, 192, 163, 61, 125, 102, 96, 78,
    108, 189, 191, 144, 196, 108, 0, 20, 70, 198, 112, 25, 240, 156, 93, 252, 80, 71, 23, 181, 201,
    145, 72, 95, 32, 199, 41, 35, 32, 54, 180, 85, 197, 196, 148, 183, 136, 111, 70, 67, 0, 91,
    210, 174, 34, 145, 63, 210, 99, 192, 49, 171, 206, 219, 18, 204, 226, 133, 169, 210, 186, 184,
    78, 155, 17, 44, 202, 92, 175, 148, 124, 134, 5, 12, 2, 134, 8, 87, 52, 95, 148, 6, 62, 225,
    201, 37, 235, 101, 181, 220, 27, 91, 19, 199, 55, 86, 80, 176, 29, 33, 20, 171, 69, 12, 233,
    161, 28, 16, 153, 65, 24, 241, 92, 98, 145, 96, 144, 177, 14, 136, 69, 42, 19, 82, 165, 129,
    119, 115, 222, 6, 67, 5, 114, 26, 57, 10, 38, 82, 99, 84, 29, 84, 188, 54, 235, 112, 230, 115,
    38, 233, 152, 154, 188, 212, 130, 8, 106, 137, 139, 112, 125, 224, 103, 92, 227, 156, 55, 197,
    229, 182, 237, 229, 125, 150, 90, 180, 203, 84, 156, 31, 241, 18, 242, 170, 201, 215, 17, 70,
    76, 240, 89, 95, 10, 166, 164, 204, 103, 251, 235, 24, 41, 224, 71, 227, 209, 181, 83, 233,
    190, 253, 152, 156, 204, 75, 200, 96, 70, 201, 1, 162, 26, 149, 108, 189, 6, 51, 139, 233, 152,
    147, 16, 70, 95, 238, 220, 165, 117, 21, 166, 117, 24, 30, 238, 88, 100, 67, 250, 112, 164, 66,
    160, 224, 179, 92, 160, 184, 55, 182, 68, 155, 21, 226, 134, 130, 49, 146, 84, 60, 117, 46,
    162, 98, 126, 121, 60, 240, 199, 85, 136, 104, 148, 97, 6, 137, 14, 130, 25, 194, 124, 131,
    157, 38, 7, 228, 36, 158, 240, 132, 197, 61, 200, 101, 179, 138, 194, 109, 25, 142, 204, 6,
    172, 0, 11, 161, 203, 195, 168, 176, 97, 121, 210, 166, 39, 8, 168, 67, 7, 76, 180, 254, 187,
    39, 191, 202, 205, 62, 167, 149, 225, 179, 193, 101, 114, 54, 253, 150, 34, 195, 123, 138, 91,
    59, 94, 63, 52, 174, 82, 96, 137, 125, 74, 31, 165, 25, 20, 217, 97, 42, 25, 10, 150, 183, 16,
    162, 57, 152, 169, 4, 69, 65, 62, 107, 153, 70, 116, 75, 16, 240, 17, 106, 121, 181, 228, 156,
    81, 172, 154, 26, 245, 96, 94, 137, 160, 75, 240, 144, 89, 160, 73, 205, 50, 122, 42, 250, 194,
    189, 0, 202, 187, 110, 53, 166, 177, 114, 86, 86, 96, 28, 71, 65, 194, 70, 19, 47, 250, 71,
    137, 251, 148, 64, 165, 58, 18, 134, 24, 203, 187, 90, 59, 116, 165, 33, 81, 65, 36, 80, 185,
    141, 69, 84, 170, 33, 71, 132, 250, 185, 167, 90, 65, 71, 20, 48, 185, 182, 132, 74, 204, 49,
    157, 147, 102, 139, 72, 104, 64, 246, 106, 123, 117, 2, 122, 58, 97, 51, 35, 87, 35, 214, 26,
    150, 178, 19, 79, 180, 43, 171, 210, 68, 158, 94, 241, 191, 53, 165, 186, 77, 160, 98, 39, 176,
    76, 252, 182, 20, 69, 116, 141, 212, 51, 54, 162, 107, 96, 123, 195, 1, 34, 112, 77, 112, 120,
    157, 29, 119, 14, 218, 198, 94, 84, 194, 5, 237, 246, 197, 19, 71, 75, 127, 68, 66, 218, 114,
    122, 247, 17, 206, 154, 7, 57, 235, 194, 78, 196, 11, 81, 135, 53, 44, 105, 74, 130, 54, 167,
    34, 183, 81, 1, 253, 64, 59, 36, 199, 152, 10, 120, 12, 60, 84, 140, 233, 11, 99, 196, 148,
    115, 253, 163, 72, 209, 194, 186, 104, 2, 130, 151, 10, 198, 143, 80, 169, 102, 140, 53, 226,
    37, 17, 122, 114, 122, 239, 58, 100, 12, 80, 107, 125, 137, 159, 93, 21, 175, 172, 242, 204,
    191, 5, 40, 56, 216, 60, 182, 148, 28, 72, 19, 128, 13, 156, 137, 75, 133, 90, 187, 136, 62, 6,
    5, 147, 108, 58, 88, 1, 242, 49, 207, 177, 123, 226, 204, 206, 186, 155, 15, 23, 102, 99, 239,
    184, 104, 97, 91, 166, 223, 215, 0, 182, 86, 58, 16, 75, 112, 65, 118, 84, 135, 101, 110, 13,
    52, 110, 150, 64, 121, 214, 44, 149, 208, 188, 44, 134, 180, 50, 54, 235, 120, 43, 97, 43, 157,
    181, 10, 42, 188, 193, 66, 64, 98, 106, 85, 69, 173, 81, 96, 241, 64, 53, 197, 183, 192, 142,
    107, 40, 179, 168, 20, 227, 115, 9, 118, 19, 169, 150, 12, 15, 232, 119, 200, 215, 146, 183,
    51, 172, 42, 47, 166, 17, 99, 6, 194, 224, 92, 38, 8, 105, 3, 220, 131, 147, 21, 147, 41, 171,
    60, 43, 39, 50, 45, 232, 80, 141, 134, 67, 202, 234, 11, 125, 99, 80, 204, 22, 202, 142, 231,
    75, 116, 123, 129, 145, 50, 69, 47, 68, 156, 71, 242, 55, 180, 86, 20, 69, 158, 51, 110, 226,
    149, 167, 252, 151, 3, 175, 41, 117, 94, 101, 8, 116, 171, 183, 152, 18, 73, 199, 243, 191, 38,
    180, 68, 113, 22, 92, 81, 80, 104, 85, 53, 120, 58, 200, 74, 124, 137, 103, 166, 105, 182, 74,
    22, 199, 132, 129, 191, 125, 39, 21, 142, 202, 134, 5, 42, 203, 49, 25, 193, 55, 96, 195, 159,
    1, 205, 165, 164, 143, 250, 168, 118, 180, 156, 7, 150, 154, 134, 244, 177, 66, 111, 75, 125,
    208, 114, 101, 223, 214, 102, 63, 70, 162, 14, 172, 89, 105, 153, 169, 244, 249, 78, 102, 249,
    207, 179, 217, 122, 37, 129, 145, 223, 231, 85, 23, 176, 140, 111, 21, 97, 207, 183, 152, 196,
    182, 204, 112, 252, 197, 101, 100, 173, 71, 152, 65, 191, 181, 89, 25, 248, 167, 110, 42, 111,
    44, 154, 91, 93, 48, 27, 66, 74, 190, 238, 135, 139, 159, 69, 115, 177, 176, 121, 6, 89, 66,
    137, 185, 114, 198, 101, 124, 229, 10, 140, 250, 176, 66, 3, 115, 151, 46, 68, 188, 168, 123,
    89, 185, 137, 44, 81, 201, 81, 89, 92, 102, 185, 118, 38, 6, 26, 182, 58, 172, 0, 27, 103, 138,
    121, 195, 122, 96, 115, 96, 17, 214, 25, 230, 101, 4, 157, 233, 48, 97, 153, 127, 189, 133,
    212, 179, 146, 217, 103, 173, 29, 227, 85, 144, 227, 19, 198, 186, 148, 59, 24, 52, 240, 240,
    165, 71, 144, 211, 145, 94, 108, 154, 55, 54,
];
pub const SECRET_KEY: &[u8] = &[
    235, 3, 235, 134, 131, 130, 254, 71, 234, 130, 100, 154, 150, 120, 99, 195, 9, 52, 49, 108,
    150, 169, 232, 220, 29, 145, 31, 175, 144, 109, 44, 222, 197, 85, 196, 127, 216, 163, 192, 37,
    180, 151, 252, 141, 236, 155, 158, 219, 58, 30, 247, 203, 141, 152, 107, 94, 54, 136, 147, 254,
    11, 10, 24, 84, 52, 129, 49, 27, 81, 184, 72, 90, 107, 72, 227, 42, 154, 47, 25, 21, 79, 68,
    181, 39, 70, 125, 137, 81, 2, 164, 244, 21, 18, 194, 64, 227, 146, 95, 9, 34, 154, 41, 101, 4,
    71, 68, 0, 98, 43, 10, 238, 209, 2, 179, 140, 79, 140, 97, 93, 25, 88, 196, 42, 36, 37, 210,
    112, 104, 208, 131, 94, 241, 25, 105, 131, 131, 78, 18, 118, 107, 77, 50, 171, 161, 153, 13,
    127, 218, 39, 71, 145, 203, 80, 197, 121, 219, 194, 109, 45, 19, 182, 3, 74, 116, 51, 195, 120,
    187, 179, 170, 93, 180, 42, 177, 68, 66, 148, 8, 145, 223, 216, 191, 148, 20, 84, 106, 98, 171,
    82, 217, 0, 159, 73, 161, 176, 227, 65, 250, 50, 68, 76, 23, 87, 144, 6, 190, 158, 145, 36,
    134, 252, 182, 129, 16, 85, 10, 64, 94, 163, 42, 134, 148, 219, 19, 74, 42, 28, 57, 132, 95,
    135, 150, 135, 186, 213, 108, 105, 6, 26, 115, 7, 105, 30, 139, 48, 168, 80, 198, 127, 162,
    196, 115, 65, 30, 1, 24, 78, 146, 140, 87, 25, 130, 86, 1, 1, 126, 202, 69, 68, 192, 248, 139,
    185, 80, 162, 119, 232, 48, 82, 70, 141, 29, 150, 105, 55, 192, 86, 196, 60, 3, 113, 136, 34,
    231, 224, 37, 95, 233, 180, 54, 139, 71, 107, 40, 105, 107, 147, 91, 245, 34, 153, 76, 43, 99,
    183, 76, 116, 125, 50, 207, 88, 179, 92, 156, 89, 39, 237, 161, 8, 93, 195, 76, 174, 118, 173,
    208, 24, 117, 199, 124, 203, 179, 198, 21, 30, 165, 104, 209, 183, 78, 92, 80, 108, 170, 114,
    21, 188, 183, 0, 47, 123, 82, 0, 6, 174, 206, 117, 204, 87, 32, 44, 94, 60, 66, 155, 40, 26,
    255, 228, 204, 223, 34, 122, 87, 124, 160, 21, 218, 193, 159, 172, 90, 30, 243, 84, 174, 9,
    180, 53, 2, 167, 130, 194, 3, 219, 182, 16, 124, 172, 116, 172, 36, 193, 92, 234, 25, 198, 210,
    77, 110, 220, 162, 247, 252, 117, 2, 114, 139, 129, 41, 205, 225, 57, 190, 165, 88, 70, 161,
    133, 156, 246, 32, 84, 53, 148, 199, 20, 171, 102, 100, 68, 182, 231, 169, 49, 189, 21, 133,
    52, 4, 13, 104, 25, 58, 194, 235, 192, 19, 230, 68, 90, 243, 17, 167, 33, 11, 86, 248, 169,
    222, 68, 176, 250, 163, 184, 114, 22, 38, 211, 102, 66, 43, 25, 148, 4, 176, 0, 205, 117, 106,
    19, 133, 20, 123, 151, 10, 40, 231, 30, 93, 107, 114, 56, 50, 62, 83, 42, 91, 196, 220, 133,
    208, 64, 154, 95, 80, 82, 126, 118, 93, 118, 118, 196, 221, 236, 39, 95, 153, 92, 121, 150, 88,
    251, 244, 131, 136, 86, 139, 123, 52, 146, 86, 60, 91, 30, 182, 131, 92, 137, 8, 72, 212, 3,
    211, 200, 202, 250, 150, 68, 248, 215, 52, 137, 57, 95, 224, 215, 135, 237, 248, 115, 59, 24,
    176, 175, 153, 141, 126, 245, 44, 139, 197, 81, 132, 131, 107, 206, 243, 19, 4, 137, 112, 114,
    252, 1, 255, 116, 159, 211, 82, 117, 171, 113, 13, 123, 106, 23, 214, 73, 25, 255, 149, 50,
    240, 67, 17, 124, 145, 56, 62, 60, 152, 105, 102, 194, 191, 0, 54, 231, 91, 89, 108, 131, 117,
    48, 114, 175, 189, 52, 105, 47, 101, 187, 90, 241, 145, 210, 210, 132, 85, 234, 177, 103, 39,
    79, 9, 156, 83, 42, 113, 70, 205, 129, 0, 251, 9, 199, 94, 39, 138, 11, 133, 160, 93, 44, 62,
    93, 145, 163, 168, 240, 145, 241, 87, 108, 162, 0, 198, 104, 179, 44, 238, 82, 37, 182, 162,
    84, 242, 101, 194, 57, 224, 127, 174, 23, 37, 109, 86, 85, 136, 181, 25, 48, 136, 56, 67, 87,
    132, 4, 107, 175, 246, 152, 153, 188, 106, 0, 248, 215, 34, 166, 243, 136, 132, 65, 83, 88,
    164, 28, 44, 179, 23, 162, 82, 164, 214, 184, 184, 210, 188, 167, 21, 196, 64, 41, 184, 46, 70,
    35, 116, 36, 149, 140, 45, 91, 29, 57, 71, 199, 100, 150, 101, 217, 165, 64, 66, 20, 150, 23,
    140, 169, 102, 85, 74, 164, 119, 134, 44, 229, 96, 71, 72, 4, 76, 108, 121, 143, 35, 150, 45,
    151, 148, 223, 11, 101, 190, 152, 35, 39, 100, 107, 88, 11, 145, 27, 60, 16, 127, 179, 170,
    242, 245, 163, 139, 115, 140, 13, 154, 166, 47, 129, 49, 82, 149, 2, 186, 166, 3, 173, 179,
    168, 164, 23, 28, 168, 55, 138, 88, 181, 42, 95, 34, 91, 193, 38, 205, 121, 233, 75, 167, 167,
    119, 181, 203, 135, 194, 100, 191, 111, 135, 127, 12, 176, 119, 43, 180, 101, 78, 35, 167, 194,
    100, 173, 50, 248, 83, 125, 197, 24, 158, 215, 57, 139, 145, 1, 161, 118, 121, 209, 209, 187,
    129, 219, 1, 171, 168, 5, 176, 16, 159, 127, 139, 130, 218, 122, 180, 67, 236, 30, 164, 112,
    178, 139, 181, 174, 225, 76, 55, 1, 155, 176, 244, 164, 45, 38, 32, 155, 4, 52, 63, 187, 211,
    156, 234, 203, 33, 134, 179, 136, 240, 227, 155, 111, 119, 182, 210, 7, 35, 54, 92, 85, 5, 80,
    48, 196, 192, 206, 119, 105, 39, 3, 22, 121, 8, 184, 151, 201, 80, 120, 34, 169, 99, 76, 140,
    74, 90, 185, 145, 239, 97, 5, 126, 16, 9, 99, 28, 48, 151, 103, 4, 146, 198, 100, 107, 37, 206,
    111, 26, 52, 64, 243, 191, 224, 117, 149, 73, 193, 9, 86, 249, 110, 96, 119, 178, 240, 120, 40,
    51, 209, 152, 133, 149, 48, 245, 131, 20, 61, 187, 52, 190, 32, 132, 1, 101, 74, 223, 220, 52,
    61, 148, 70, 224, 187, 37, 231, 226, 136, 186, 117, 57, 254, 68, 128, 32, 39, 105, 106, 106,
    191, 42, 48, 170, 201, 39, 126, 93, 231, 37, 154, 17, 21, 0, 118, 32, 196, 7, 86, 186, 84, 148,
    4, 219, 207, 59, 75, 139, 211, 128, 111, 124, 252, 172, 33, 234, 92, 192, 230, 185, 118, 48,
    99, 181, 201, 187, 18, 117, 80, 117, 105, 30, 76, 21, 58, 126, 153, 12, 190, 118, 97, 118, 136,
    66, 248, 112, 96, 135, 146, 39, 126, 72, 43, 204, 171, 54, 99, 76, 41, 74, 154, 129, 140, 20,
    74, 222, 180, 30, 23, 2, 115, 87, 84, 4, 45, 108, 139, 61, 70, 6, 100, 68, 56, 134, 81, 88,
    197, 106, 5, 186, 150, 54, 108, 193, 9, 10, 245, 153, 187, 65, 39, 136, 16, 199, 24, 210, 87,
    126, 150, 147, 196, 96, 143, 176, 82, 80, 214, 119, 115, 88, 180, 96, 72, 202, 203, 52, 44,
    118, 35, 88, 31, 51, 51, 187, 233, 180, 36, 45, 91, 63, 36, 132, 80, 160, 19, 200, 115, 149, 4,
    216, 55, 82, 26, 103, 43, 245, 82, 189, 91, 6, 87, 180, 170, 105, 68, 24, 163, 23, 162, 82,
    255, 55, 187, 192, 163, 61, 125, 102, 96, 78, 108, 189, 191, 144, 196, 108, 0, 20, 70, 198,
    112, 25, 240, 156, 93, 252, 80, 71, 23, 181, 201, 145, 72, 95, 32, 199, 41, 35, 32, 54, 180,
    85, 197, 196, 148, 183, 136, 111, 70, 67, 0, 91, 210, 174, 34, 145, 63, 210, 99, 192, 49, 171,
    206, 219, 18, 204, 226, 133, 169, 210, 186, 184, 78, 155, 17, 44, 202, 92, 175, 148, 124, 134,
    5, 12, 2, 134, 8, 87, 52, 95, 148, 6, 62, 225, 201, 37, 235, 101, 181, 220, 27, 91, 19, 199,
    55, 86, 80, 176, 29, 33, 20, 171, 69, 12, 233, 161, 28, 16, 153, 65, 24, 241, 92, 98, 145, 96,
    144, 177, 14, 136, 69, 42, 19, 82, 165, 129, 119, 115, 222, 6, 67, 5, 114, 26, 57, 10, 38, 82,
    99, 84, 29, 84, 188, 54, 235, 112, 230, 115, 38, 233, 152, 154, 188, 212, 130, 8, 106, 137,
    139, 112, 125, 224, 103, 92, 227, 156, 55, 197, 229, 182, 237, 229, 125, 150, 90, 180, 203, 84,
    156, 31, 241, 18, 242, 170, 201, 215, 17, 70, 76, 240, 89, 95, 10, 166, 164, 204, 103, 251,
    235, 24, 41, 224, 71, 227, 209, 181, 83, 233, 190, 253, 152, 156, 204, 75, 200, 96, 70, 201, 1,
    162, 26, 149, 108, 189, 6, 51, 139, 233, 152, 147, 16, 70, 95, 238, 220, 165, 117, 21, 166,
    117, 24, 30, 238, 88, 100, 67, 250, 112, 164, 66, 160, 224, 179, 92, 160, 184, 55, 182, 68,
    155, 21, 226, 134, 130, 49, 146, 84, 60, 117, 46, 162, 98, 126, 121, 60, 240, 199, 85, 136,
    104, 148, 97, 6, 137, 14, 130, 25, 194, 124, 131, 157, 38, 7, 228, 36, 158, 240, 132, 197, 61,
    200, 101, 179, 138, 194, 109, 25, 142, 204, 6, 172, 0, 11, 161, 203, 195, 168, 176, 97, 121,
    210, 166, 39, 8, 168, 67, 7, 76, 180, 254, 187, 39, 191, 202, 205, 62, 167, 149, 225, 179, 193,
    101, 114, 54, 253, 150, 34, 195, 123, 138, 91, 59, 94, 63, 52, 174, 82, 96, 137, 125, 74, 31,
    165, 25, 20, 217, 97, 42, 25, 10, 150, 183, 16, 162, 57, 152, 169, 4, 69, 65, 62, 107, 153, 70,
    116, 75, 16, 240, 17, 106, 121, 181, 228, 156, 81, 172, 154, 26, 245, 96, 94, 137, 160, 75,
    240, 144, 89, 160, 73, 205, 50, 122, 42, 250, 194, 189, 0, 202, 187, 110, 53, 166, 177, 114,
    86, 86, 96, 28, 71, 65, 194, 70, 19, 47, 250, 71, 137, 251, 148, 64, 165, 58, 18, 134, 24, 203,
    187, 90, 59, 116, 165, 33, 81, 65, 36, 80, 185, 141, 69, 84, 170, 33, 71, 132, 250, 185, 167,
    90, 65, 71, 20, 48, 185, 182, 132, 74, 204, 49, 157, 147, 102, 139, 72, 104, 64, 246, 106, 123,
    117, 2, 122, 58, 97, 51, 35, 87, 35, 214, 26, 150, 178, 19, 79, 180, 43, 171, 210, 68, 158, 94,
    241, 191, 53, 165, 186, 77, 160, 98, 39, 176, 76, 252, 182, 20, 69, 116, 141, 212, 51, 54, 162,
    107, 96, 123, 195, 1, 34, 112, 77, 112, 120, 157, 29, 119, 14, 218, 198, 94, 84, 194, 5, 237,
    246, 197, 19, 71, 75, 127, 68, 66, 218, 114, 122, 247, 17, 206, 154, 7, 57, 235, 194, 78, 196,
    11, 81, 135, 53, 44, 105, 74, 130, 54, 167, 34, 183, 81, 1, 253, 64, 59, 36, 199, 152, 10, 120,
    12, 60, 84, 140, 233, 11, 99, 196, 148, 115, 253, 163, 72, 209, 194, 186, 104, 2, 130, 151, 10,
    198, 143, 80, 169, 102, 140, 53, 226, 37, 17, 122, 114, 122, 239, 58, 100, 12, 80, 107, 125,
    137, 159, 93, 21, 175, 172, 242, 204, 191, 5, 40, 56, 216, 60, 182, 148, 28, 72, 19, 128, 13,
    156, 137, 75, 133, 90, 187, 136, 62, 6, 5, 147, 108, 58, 88, 1, 242, 49, 207, 177, 123, 226,
    204, 206, 186, 155, 15, 23, 102, 99, 239, 184, 104, 97, 91, 166, 223, 215, 0, 182, 86, 58, 16,
    75, 112, 65, 118, 84, 135, 101, 110, 13, 52, 110, 150, 64, 121, 214, 44, 149, 208, 188, 44,
    134, 180, 50, 54, 235, 120, 43, 97, 43, 157, 181, 10, 42, 188, 193, 66, 64, 98, 106, 85, 69,
    173, 81, 96, 241, 64, 53, 197, 183, 192, 142, 107, 40, 179, 168, 20, 227, 115, 9, 118, 19, 169,
    150, 12, 15, 232, 119, 200, 215, 146, 183, 51, 172, 42, 47, 166, 17, 99, 6, 194, 224, 92, 38,
    8, 105, 3, 220, 131, 147, 21, 147, 41, 171, 60, 43, 39, 50, 45, 232, 80, 141, 134, 67, 202,
    234, 11, 125, 99, 80, 204, 22, 202, 142, 231, 75, 116, 123, 129, 145, 50, 69, 47, 68, 156, 71,
    242, 55, 180, 86, 20, 69, 158, 51, 110, 226, 149, 167, 252, 151, 3, 175, 41, 117, 94, 101, 8,
    116, 171, 183, 152, 18, 73, 199, 243, 191, 38, 180, 68, 113, 22, 92, 81, 80, 104, 85, 53, 120,
    58, 200, 74, 124, 137, 103, 166, 105, 182, 74, 22, 199, 132, 129, 191, 125, 39, 21, 142, 202,
    134, 5, 42, 203, 49, 25, 193, 55, 96, 195, 159, 1, 205, 165, 164, 143, 250, 168, 118, 180, 156,
    7, 150, 154, 134, 244, 177, 66, 111, 75, 125, 208, 114, 101, 223, 214, 102, 63, 70, 162, 14,
    172, 89, 105, 153, 169, 244, 249, 78, 102, 249, 207, 179, 217, 122, 37, 129, 145, 223, 231, 85,
    23, 176, 140, 111, 21, 97, 207, 183, 152, 196, 182, 204, 112, 252, 197, 101, 100, 173, 71, 152,
    65, 191, 181, 89, 25, 248, 167, 110, 42, 111, 44, 154, 91, 93, 48, 27, 66, 74, 190, 238, 135,
    139, 159, 69, 115, 177, 176, 121, 6, 89, 66, 137, 185, 114, 198, 101, 124, 229, 10, 140, 250,
    176, 66, 3, 115, 151, 46, 68, 188, 168, 123, 89, 185, 137, 44, 81, 201, 81, 89, 92, 102, 185,
    118, 38, 6, 26, 182, 58, 172, 0, 27, 103, 138, 121, 195, 122, 96, 115, 96, 17, 214, 25, 230,
    101, 4, 157, 233, 48, 97, 153, 127, 189, 133, 212, 179, 146, 217, 103, 173, 29, 227, 85, 144,
    227, 19, 198, 186, 148, 59, 24, 52, 240, 240, 165, 71, 144, 211, 145, 94, 108, 154, 55, 54,
    211, 201, 96, 160, 78, 19, 183, 214, 173, 63, 121, 56, 84, 57, 234, 197, 91, 147, 80, 28, 47,
    238, 88, 139, 211, 131, 46, 239, 134, 88, 168, 189, 159, 94, 251, 201, 166, 178, 233, 130, 180,
    134, 216, 184, 68, 125, 218, 108, 71, 134, 160, 164, 239, 211, 222, 4, 88, 212, 198, 222, 202,
    0, 54, 131,
];
pub const ENCRYPTED_MESSAGE: &[u8] = &[
    227, 234, 21, 13, 124, 194, 35, 79, 43, 9, 1, 197, 216, 172, 197, 74, 77, 117, 225, 225, 109,
    41, 27, 127, 186, 37, 28, 88, 97, 98, 134, 24, 154, 127, 237, 99, 65, 171, 248, 106, 255, 203,
    122, 131, 72, 151, 214, 111, 55, 215, 173, 175, 75, 29, 117, 31, 73, 59, 108, 169, 74, 52, 2,
    116, 33, 134, 144, 101, 255, 185, 165, 99, 242, 31, 129, 126, 95, 150, 18, 154, 177, 21, 112,
    121, 231, 66, 158, 6, 4, 249, 246, 216, 205, 74, 148, 197, 176, 93, 224, 54, 205, 14, 160, 137,
    249, 187, 76, 77, 245, 169, 99, 110, 38, 30, 148, 181, 181, 248, 255, 212, 204, 102, 234, 86,
    134, 221, 9, 182, 7, 184, 153, 95, 30, 215, 234, 149, 235, 9, 234, 214, 86, 226, 209, 213, 128,
    17, 71, 242, 175, 124, 191, 133, 28, 136, 56, 18, 224, 225, 74, 81, 198, 100, 24, 84, 129, 96,
    85, 121, 74, 17, 53, 20, 235, 253, 206, 85, 230, 38, 68, 172, 93, 16, 177, 44, 49, 106, 57, 85,
    208, 59, 89, 62, 213, 164, 115, 46, 112, 8, 155, 207, 29, 112, 173, 211, 26, 73, 206, 205, 125,
    168, 147, 102, 79, 89, 225, 212, 64, 57, 50, 156, 217, 194, 125, 39, 222, 20, 237, 206, 226,
    204, 233, 128, 250, 192, 196, 20, 158, 121, 129, 117, 199, 161, 194, 219, 71, 116, 108, 21,
    201, 172, 29, 77, 122, 67, 222, 54, 87, 43, 85, 229, 239, 41, 75, 11, 90, 167, 97, 43, 176,
    209, 220, 28, 69, 151, 117, 115, 46, 157, 206, 166, 238, 78, 192, 248, 63, 85, 28, 5, 92, 98,
    118, 61, 162, 87, 216, 130, 196, 205, 233, 171, 10, 173, 171, 151, 113, 95, 233, 156, 111, 194,
    23, 243, 247, 204, 169, 79, 55, 119, 140, 156, 172, 102, 176, 226, 79, 29, 166, 103, 148, 181,
    197, 77, 168, 71, 249, 97, 60, 148, 35, 139, 28, 180, 57, 167, 244, 158, 74, 10, 27, 33, 126,
    164, 42, 152, 155, 232, 117, 139, 211, 128, 220, 177, 158, 199, 49, 119, 190, 81, 70, 4, 65,
    117, 206, 64, 84, 104, 217, 126, 92, 19, 189, 188, 171, 102, 103, 204, 199, 80, 185, 20, 200,
    125, 100, 221, 67, 97, 115, 98, 104, 226, 43, 213, 11, 119, 131, 208, 113, 160, 45, 45, 67, 95,
    208, 61, 28, 14, 82, 121, 70, 209, 123, 98, 174, 192, 2, 18, 28, 129, 245, 68, 13, 211, 216,
    120, 128, 72, 3, 25, 196, 225, 173, 34, 71, 127, 68, 128, 20, 151, 57, 199, 239, 2, 67, 24,
    181, 21, 9, 12, 100, 146, 48, 226, 7, 228, 62, 155, 127, 220, 242, 55, 168, 209, 44, 80, 46,
    124, 34, 69, 101, 135, 194, 63, 217, 150, 40, 98, 213, 52, 114, 78, 178, 208, 160, 193, 121,
    173, 156, 51, 90, 81, 33, 89, 184, 62, 248, 3, 26, 188, 115, 113, 251, 205, 110, 18, 217, 69,
    187, 56, 25, 47, 2, 225, 59, 85, 182, 151, 201, 12, 89, 94, 90, 106, 151, 42, 25, 192, 216, 75,
    130, 229, 192, 121, 225, 101, 134, 246, 125, 98, 126, 201, 124, 84, 201, 71, 40, 189, 210, 136,
    137, 194, 216, 113, 67, 250, 102, 21, 186, 200, 54, 123, 186, 241, 203, 182, 196, 180, 13, 203,
    122, 135, 168, 200, 89, 185, 104, 3, 146, 92, 51, 10, 54, 162, 111, 47, 254, 198, 103, 65, 195,
    245, 173, 131, 130, 172, 186, 119, 141, 62, 12, 241, 157, 119, 187, 184, 49, 228, 37, 236, 30,
    60, 251, 180, 225, 30, 154, 156, 172, 148, 204, 68, 132, 58, 230, 163, 89, 144, 137, 131, 118,
    186, 236, 197, 219, 193, 14, 231, 1, 158, 34, 86, 157, 49, 80, 171, 178, 246, 228, 130, 252,
    210, 237, 192, 132, 17, 204, 30, 13, 123, 46, 149, 22, 99, 144, 162, 216, 8, 79, 59, 145, 248,
    112, 124, 71, 169, 21, 123, 89, 131, 71, 127, 54, 233, 122, 192, 152, 167, 144, 239, 20, 98,
    117, 45, 217, 70, 162, 126, 21, 180, 253, 201, 64, 17, 231, 207, 29, 72, 41, 168, 174, 56, 218,
    15, 98, 208, 155, 3, 97, 12, 202, 3, 49, 35, 212, 171, 146, 152, 195, 198, 221, 29, 176, 27,
    194, 126, 95, 109, 188, 114, 26, 187, 109, 127, 51, 62, 172, 201, 126, 5, 122, 218, 206, 198,
    175, 194, 80, 173, 63, 195, 42, 207, 5, 137, 151, 112, 183, 41, 57, 0, 36, 215, 124, 58, 184,
    27, 140, 42, 175, 229, 220, 141, 117, 185, 61, 194, 59, 195, 187, 188, 209, 132, 112, 66, 190,
    203, 184, 169, 51, 155, 207, 132, 96, 112, 149, 226, 202, 250, 97, 2, 92, 34, 38, 166, 51, 165,
    63, 36, 31, 78, 62, 31, 23, 154, 25, 70, 116, 193, 51, 43, 36, 93, 133, 162, 227, 191, 189,
    116, 39, 152, 188, 167, 20, 181, 73, 37, 65, 46, 111, 11, 9, 67, 90, 68, 85, 7, 131, 195, 60,
    89, 170, 196, 192, 200, 122, 29, 47, 5, 232, 20, 66, 102, 101, 129, 199, 107, 72, 170, 135, 91,
    216, 40, 50, 61, 251, 26, 50, 42, 122, 147, 174, 187, 143, 64, 203, 243, 107, 170, 231, 183,
    169, 157, 147, 111, 154, 130, 111, 180, 173, 34, 140, 160, 35, 44, 14, 221, 190, 20, 65, 41,
    108, 132, 160, 173, 217, 208, 102, 160, 47, 255, 136, 132, 53, 104, 225, 94, 18, 106, 233, 45,
    53, 135, 155, 160, 15, 160, 145, 156, 125, 212, 53, 62, 202, 169, 205, 157, 230, 140, 212, 10,
    116, 83, 85, 95, 10, 143, 81, 248, 199, 235, 126, 173, 155, 131, 75, 179, 167, 40, 161, 127,
    202, 60, 240, 28, 178, 105, 189, 225, 172, 70, 200, 252, 210, 220, 100, 127, 48, 178, 35, 81,
    195, 252, 235, 212, 93, 164, 207, 108, 68, 155, 200, 62, 171, 231, 41, 150, 109, 68, 142, 72,
    221, 143, 214, 17, 158, 238, 131, 10, 183, 218, 200, 8, 94, 160, 167, 210, 151, 120, 59, 253,
    141, 208, 116, 73, 230, 28, 112, 95, 236, 149, 127, 60, 56, 233, 58, 234, 108, 61, 56, 164,
    184, 19, 159, 19, 84, 226, 177, 29, 172, 122, 196, 203, 71, 1, 137, 10, 16, 119, 100, 85, 85,
    170, 52, 7, 43, 141, 34, 6, 127, 4, 47, 1, 38, 233, 21, 108, 143, 225, 226, 237, 141, 73, 40,
    104, 224, 38, 198, 225, 23, 230, 83, 240, 197, 152, 224, 6, 49, 246, 172, 50, 180, 68, 15, 199,
    38, 45, 130, 47, 20, 99, 50, 173, 63, 73, 235, 96, 214, 236, 89, 23, 145,
];

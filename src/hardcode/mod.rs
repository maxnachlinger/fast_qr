//! Contains all different levels of quality.
//! And allows to find easily max bits per version/quality pair

#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::ecl::ECL;
use crate::encode::Mode;
use crate::version::Version;

#[cfg(test)]
mod test;

/// Fetches the right array to retrieve the information on **groups**
pub const fn ecc_to_groups(quality: ECL, version: Version) -> [(usize, usize); 2] {
    const L: [u32; 40] = [
        (1 << 24) | (19 << 16) | (0 << 8) | 0,
        (1 << 24) | (34 << 16) | (0 << 8) | 0,
        (1 << 24) | (55 << 16) | (0 << 8) | 0,
        (1 << 24) | (80 << 16) | (0 << 8) | 0,
        (1 << 24) | (108 << 16) | (0 << 8) | 0,
        (2 << 24) | (68 << 16) | (0 << 8) | 0,
        (2 << 24) | (78 << 16) | (0 << 8) | 0,
        (2 << 24) | (97 << 16) | (0 << 8) | 0,
        (2 << 24) | (116 << 16) | (0 << 8) | 0,
        (2 << 24) | (68 << 16) | (2 << 8) | 69,
        (4 << 24) | (81 << 16) | (0 << 8) | 0,
        (2 << 24) | (92 << 16) | (2 << 8) | 93,
        (4 << 24) | (107 << 16) | (0 << 8) | 0,
        (3 << 24) | (115 << 16) | (1 << 8) | 116,
        (5 << 24) | (87 << 16) | (1 << 8) | 88,
        (5 << 24) | (98 << 16) | (1 << 8) | 99,
        (1 << 24) | (107 << 16) | (5 << 8) | 108,
        (5 << 24) | (120 << 16) | (1 << 8) | 121,
        (3 << 24) | (113 << 16) | (4 << 8) | 114,
        (3 << 24) | (107 << 16) | (5 << 8) | 108,
        (4 << 24) | (116 << 16) | (4 << 8) | 117,
        (2 << 24) | (111 << 16) | (7 << 8) | 112,
        (4 << 24) | (121 << 16) | (5 << 8) | 122,
        (6 << 24) | (117 << 16) | (4 << 8) | 118,
        (8 << 24) | (106 << 16) | (4 << 8) | 107,
        (10 << 24) | (114 << 16) | (2 << 8) | 115,
        (8 << 24) | (122 << 16) | (4 << 8) | 123,
        (3 << 24) | (117 << 16) | (10 << 8) | 118,
        (7 << 24) | (116 << 16) | (7 << 8) | 117,
        (5 << 24) | (115 << 16) | (10 << 8) | 116,
        (13 << 24) | (115 << 16) | (3 << 8) | 116,
        (17 << 24) | (115 << 16) | (0 << 8) | 0,
        (17 << 24) | (115 << 16) | (1 << 8) | 116,
        (13 << 24) | (115 << 16) | (6 << 8) | 116,
        (12 << 24) | (121 << 16) | (7 << 8) | 122,
        (6 << 24) | (121 << 16) | (14 << 8) | 122,
        (17 << 24) | (122 << 16) | (4 << 8) | 123,
        (4 << 24) | (122 << 16) | (18 << 8) | 123,
        (20 << 24) | (117 << 16) | (4 << 8) | 118,
        (19 << 24) | (118 << 16) | (6 << 8) | 119,
    ];

    const M: [u32; 40] = [
        (1 << 24) | (16 << 16) | (0 << 8) | 0,
        (1 << 24) | (28 << 16) | (0 << 8) | 0,
        (1 << 24) | (44 << 16) | (0 << 8) | 0,
        (2 << 24) | (32 << 16) | (0 << 8) | 0,
        (2 << 24) | (43 << 16) | (0 << 8) | 0,
        (4 << 24) | (27 << 16) | (0 << 8) | 0,
        (4 << 24) | (31 << 16) | (0 << 8) | 0,
        (2 << 24) | (38 << 16) | (2 << 8) | 39,
        (3 << 24) | (36 << 16) | (2 << 8) | 37,
        (4 << 24) | (43 << 16) | (1 << 8) | 44,
        (1 << 24) | (50 << 16) | (4 << 8) | 51,
        (6 << 24) | (36 << 16) | (2 << 8) | 37,
        (8 << 24) | (37 << 16) | (1 << 8) | 38,
        (4 << 24) | (40 << 16) | (5 << 8) | 41,
        (5 << 24) | (41 << 16) | (5 << 8) | 42,
        (7 << 24) | (45 << 16) | (3 << 8) | 46,
        (10 << 24) | (46 << 16) | (1 << 8) | 47,
        (9 << 24) | (43 << 16) | (4 << 8) | 44,
        (3 << 24) | (44 << 16) | (11 << 8) | 45,
        (3 << 24) | (41 << 16) | (13 << 8) | 42,
        (17 << 24) | (42 << 16) | (0 << 8) | 0,
        (17 << 24) | (46 << 16) | (0 << 8) | 0,
        (4 << 24) | (47 << 16) | (14 << 8) | 48,
        (6 << 24) | (45 << 16) | (14 << 8) | 46,
        (8 << 24) | (47 << 16) | (13 << 8) | 48,
        (19 << 24) | (46 << 16) | (4 << 8) | 47,
        (22 << 24) | (45 << 16) | (3 << 8) | 46,
        (3 << 24) | (45 << 16) | (23 << 8) | 46,
        (21 << 24) | (45 << 16) | (7 << 8) | 46,
        (19 << 24) | (47 << 16) | (10 << 8) | 48,
        (2 << 24) | (46 << 16) | (29 << 8) | 47,
        (10 << 24) | (46 << 16) | (23 << 8) | 47,
        (14 << 24) | (46 << 16) | (21 << 8) | 47,
        (14 << 24) | (46 << 16) | (23 << 8) | 47,
        (12 << 24) | (47 << 16) | (26 << 8) | 48,
        (6 << 24) | (47 << 16) | (34 << 8) | 48,
        (29 << 24) | (46 << 16) | (14 << 8) | 47,
        (13 << 24) | (46 << 16) | (32 << 8) | 47,
        (40 << 24) | (47 << 16) | (7 << 8) | 48,
        (18 << 24) | (47 << 16) | (31 << 8) | 48,
    ];

    const Q: [u32; 40] = [
        (1 << 24) | (13 << 16) | (0 << 8) | 0,
        (1 << 24) | (22 << 16) | (0 << 8) | 0,
        (2 << 24) | (17 << 16) | (0 << 8) | 0,
        (2 << 24) | (24 << 16) | (0 << 8) | 0,
        (2 << 24) | (15 << 16) | (2 << 8) | 16,
        (4 << 24) | (19 << 16) | (0 << 8) | 0,
        (2 << 24) | (14 << 16) | (4 << 8) | 15,
        (4 << 24) | (18 << 16) | (2 << 8) | 19,
        (4 << 24) | (16 << 16) | (4 << 8) | 17,
        (6 << 24) | (19 << 16) | (2 << 8) | 20,
        (4 << 24) | (22 << 16) | (4 << 8) | 23,
        (4 << 24) | (20 << 16) | (6 << 8) | 21,
        (8 << 24) | (20 << 16) | (4 << 8) | 21,
        (11 << 24) | (16 << 16) | (5 << 8) | 17,
        (5 << 24) | (24 << 16) | (7 << 8) | 25,
        (15 << 24) | (19 << 16) | (2 << 8) | 20,
        (1 << 24) | (22 << 16) | (15 << 8) | 23,
        (17 << 24) | (22 << 16) | (1 << 8) | 23,
        (17 << 24) | (21 << 16) | (4 << 8) | 22,
        (15 << 24) | (24 << 16) | (5 << 8) | 25,
        (17 << 24) | (22 << 16) | (6 << 8) | 23,
        (7 << 24) | (24 << 16) | (16 << 8) | 25,
        (11 << 24) | (24 << 16) | (14 << 8) | 25,
        (11 << 24) | (24 << 16) | (16 << 8) | 25,
        (7 << 24) | (24 << 16) | (22 << 8) | 25,
        (28 << 24) | (22 << 16) | (6 << 8) | 23,
        (8 << 24) | (23 << 16) | (26 << 8) | 24,
        (4 << 24) | (24 << 16) | (31 << 8) | 25,
        (1 << 24) | (23 << 16) | (37 << 8) | 24,
        (15 << 24) | (24 << 16) | (25 << 8) | 25,
        (42 << 24) | (24 << 16) | (1 << 8) | 25,
        (10 << 24) | (24 << 16) | (35 << 8) | 25,
        (29 << 24) | (24 << 16) | (19 << 8) | 25,
        (44 << 24) | (24 << 16) | (7 << 8) | 25,
        (39 << 24) | (24 << 16) | (14 << 8) | 25,
        (46 << 24) | (24 << 16) | (10 << 8) | 25,
        (49 << 24) | (24 << 16) | (10 << 8) | 25,
        (48 << 24) | (24 << 16) | (14 << 8) | 25,
        (43 << 24) | (24 << 16) | (22 << 8) | 25,
        (34 << 24) | (24 << 16) | (34 << 8) | 25,
    ];

    const H: [u32; 40] = [
        (1 << 24) | (9 << 16) | (0 << 8) | 0,
        (1 << 24) | (16 << 16) | (0 << 8) | 0,
        (2 << 24) | (13 << 16) | (0 << 8) | 0,
        (4 << 24) | (9 << 16) | (0 << 8) | 0,
        (2 << 24) | (11 << 16) | (2 << 8) | 12,
        (4 << 24) | (15 << 16) | (0 << 8) | 0,
        (4 << 24) | (13 << 16) | (1 << 8) | 14,
        (4 << 24) | (14 << 16) | (2 << 8) | 15,
        (4 << 24) | (12 << 16) | (4 << 8) | 13,
        (6 << 24) | (15 << 16) | (2 << 8) | 16,
        (3 << 24) | (12 << 16) | (8 << 8) | 13,
        (7 << 24) | (14 << 16) | (4 << 8) | 15,
        (12 << 24) | (11 << 16) | (4 << 8) | 12,
        (11 << 24) | (12 << 16) | (5 << 8) | 13,
        (11 << 24) | (12 << 16) | (7 << 8) | 13,
        (3 << 24) | (15 << 16) | (13 << 8) | 16,
        (2 << 24) | (14 << 16) | (17 << 8) | 15,
        (2 << 24) | (14 << 16) | (19 << 8) | 15,
        (9 << 24) | (13 << 16) | (16 << 8) | 14,
        (15 << 24) | (15 << 16) | (10 << 8) | 16,
        (19 << 24) | (16 << 16) | (6 << 8) | 17,
        (34 << 24) | (13 << 16) | (0 << 8) | 0,
        (16 << 24) | (15 << 16) | (14 << 8) | 16,
        (30 << 24) | (16 << 16) | (2 << 8) | 17,
        (22 << 24) | (15 << 16) | (13 << 8) | 16,
        (33 << 24) | (16 << 16) | (4 << 8) | 17,
        (12 << 24) | (15 << 16) | (28 << 8) | 16,
        (11 << 24) | (15 << 16) | (31 << 8) | 16,
        (19 << 24) | (15 << 16) | (26 << 8) | 16,
        (23 << 24) | (15 << 16) | (25 << 8) | 16,
        (23 << 24) | (15 << 16) | (28 << 8) | 16,
        (19 << 24) | (15 << 16) | (35 << 8) | 16,
        (11 << 24) | (15 << 16) | (46 << 8) | 16,
        (59 << 24) | (16 << 16) | (1 << 8) | 17,
        (22 << 24) | (15 << 16) | (41 << 8) | 16,
        (2 << 24) | (15 << 16) | (64 << 8) | 16,
        (24 << 24) | (15 << 16) | (46 << 8) | 16,
        (42 << 24) | (15 << 16) | (32 << 8) | 16,
        (10 << 24) | (15 << 16) | (67 << 8) | 16,
        (20 << 24) | (15 << 16) | (61 << 8) | 16,
    ];

    let version = version as usize;
    let ecgroups = match quality {
        ECL::L => L[version],
        ECL::M => M[version],
        ECL::Q => Q[version],
        ECL::H => H[version],
    };

    let (g1_count, g1_size, g2_count, g2_size) = (
        (ecgroups >> 24) & 0xFF,
        (ecgroups >> 16) & 0xFF,
        (ecgroups >> 8) & 0xFF,
        ecgroups & 0xFF,
    );

    [
        (g1_count as usize, g1_size as usize),
        (g2_count as usize, g2_size as usize),
    ]
}

/// Fetches the right array to retrieve the **format information**
pub const fn ecm_to_format_information(quality: ECL, mask_nb: usize) -> u16 {
    const L: [u16; 8] = [
        0b111011111000100,
        0b111001011110011,
        0b111110110101010,
        0b111100010011101,
        0b110011000101111,
        0b110001100011000,
        0b110110001000001,
        0b110100101110110,
    ];

    const M: [u16; 8] = [
        0b101010000010010,
        0b101000100100101,
        0b101111001111100,
        0b101101101001011,
        0b100010111111001,
        0b100000011001110,
        0b100111110010111,
        0b100101010100000,
    ];

    const Q: [u16; 8] = [
        0b011010101011111,
        0b011000001101000,
        0b011111100110001,
        0b011101000000110,
        0b010010010110100,
        0b010000110000011,
        0b010111011011010,
        0b010101111101101,
    ];

    const H: [u16; 8] = [
        0b001011010001001,
        0b001001110111110,
        0b001110011100111,
        0b001100111010000,
        0b000011101100010,
        0b000001001010101,
        0b000110100001100,
        0b000100000111011,
    ];

    match quality {
        ECL::L => L[mask_nb],
        ECL::M => M[mask_nb],
        ECL::Q => Q[mask_nb],
        ECL::H => H[mask_nb],
    }
}

/// Returns the number of **data codewords** according to `version` and `ecl`
pub const fn data_codewords(version: Version, ecl: ECL) -> usize {
    const L: [u16; 40] = [
        19, 34, 55, 80, 108, 136, 156, 194, 232, 274, 324, 370, 428, 461, 523, 589, 647, 721, 795,
        861, 932, 1006, 1094, 1174, 1276, 1370, 1468, 1531, 1631, 1735, 1843, 1955, 2071, 2191,
        2306, 2434, 2566, 2702, 2812, 2956,
    ];

    const M: [u16; 40] = [
        16, 28, 44, 64, 86, 108, 124, 154, 182, 216, 254, 290, 334, 365, 415, 453, 507, 563, 627,
        669, 714, 782, 860, 914, 1000, 1062, 1128, 1193, 1267, 1373, 1455, 1541, 1631, 1725, 1812,
        1914, 1992, 2102, 2216, 2334,
    ];

    const Q: [u16; 40] = [
        13, 22, 34, 48, 62, 76, 88, 110, 132, 154, 180, 206, 244, 261, 295, 325, 367, 397, 445,
        485, 512, 568, 614, 664, 718, 754, 808, 871, 911, 985, 1033, 1115, 1171, 1231, 1286, 1354,
        1426, 1502, 1582, 1666,
    ];

    const H: [u16; 40] = [
        9, 16, 26, 36, 46, 60, 66, 86, 100, 122, 140, 158, 180, 197, 223, 253, 283, 313, 341, 385,
        406, 442, 464, 514, 538, 596, 628, 661, 701, 745, 793, 845, 901, 961, 986, 1054, 1096,
        1142, 1222, 1276,
    ];

    (match ecl {
        ECL::L => L[version as usize],
        ECL::M => M[version as usize],
        ECL::Q => Q[version as usize],
        ECL::H => H[version as usize],
    }) as usize
}

/// Returns the number **data bits** according to `version` and `ecl`
pub const fn data_bits(version: Version, ecl: ECL) -> usize {
    data_codewords(version, ecl) * 8
}

/// Returns the **number of bits** required to represent a number according to `version` and `mode`
pub const fn cci_bits(version: Version, mode: Mode) -> usize {
    use Version::*;

    match mode {
        Mode::Numeric => match version {
            v if (v as usize) >= (V27 as usize) => 14,
            v if (v as usize) >= (V10 as usize) => 12,
            _ => 10,
        },
        Mode::Alphanumeric => match version {
            v if (v as usize) >= (V27 as usize) => 13,
            v if (v as usize) >= (V10 as usize) => 11,
            _ => 9,
        },
        Mode::Byte => match version {
            v if (v as usize) >= (V10 as usize) => 16,
            _ => 8,
        },
    }
}

/// Returns required **dividing polynomial** according to `version` and `ecl`
pub const fn get_polynomial(version: Version, ecl: ECL) -> &'static [u8] {
    use Version::*;
    use ECL::*;

    match (version, ecl) {
        (V01, L) => &[0, 87, 229, 146, 149, 238, 102, 21],
        (V01, M) | (V02, L) => &[0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45],
        (V01, Q) => &[
            0, 74, 152, 176, 100, 86, 100, 106, 104, 130, 218, 206, 140, 78,
        ],
        (V03, L) => &[
            0, 8, 183, 61, 91, 202, 37, 51, 58, 58, 237, 140, 124, 5, 99, 105,
        ],
        (V02, M) | (V04, H) | (V06, M) => &[
            0, 120, 104, 107, 109, 102, 161, 76, 3, 91, 191, 147, 169, 182, 194, 225, 120,
        ],
        (V01, H) => &[
            0, 43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163, 136,
        ],
        (V03, Q) | (V04, M) | (V05, Q) | (V06, L) | (V07, M) | (V07, Q) | (V10, L) => &[
            0, 215, 234, 158, 94, 184, 97, 118, 170, 79, 187, 152, 148, 252, 179, 5, 98, 96, 153,
        ],
        (V04, L) | (V07, L) | (V09, Q) | (V11, L) | (V14, Q) => &[
            0, 17, 60, 79, 50, 61, 163, 26, 187, 202, 180, 221, 225, 83, 239, 156, 164, 212, 212,
            188, 190,
        ],
        (V02, Q)
        | (V03, H)
        | (V05, H)
        | (V08, M)
        | (V08, Q)
        | (V09, M)
        | (V12, M)
        | (V13, M)
        | (V13, H)
        | (V15, L) => &[
            0, 210, 171, 247, 242, 93, 230, 14, 109, 221, 53, 200, 74, 8, 172, 98, 80, 219, 134,
            160, 105, 165, 231,
        ],
        (V05, M)
        | (V06, Q)
        | (V08, L)
        | (V09, H)
        | (V10, Q)
        | (V11, H)
        | (V12, L)
        | (V13, Q)
        | (V14, M)
        | (V14, H)
        | (V15, M)
        | (V15, H)
        | (V16, L)
        | (V16, Q)
        | (V22, H) => &[
            0, 229, 121, 135, 48, 211, 117, 251, 126, 159, 180, 169, 152, 192, 226, 228, 218, 111,
            0, 117, 232, 87, 96, 227, 21,
        ],
        (V03, M)
        | (V04, Q)
        | (V05, L)
        | (V07, H)
        | (V08, H)
        | (V10, M)
        | (V12, Q)
        | (V13, L)
        | (V18, M)
        | (V19, M)
        | (V19, Q)
        | (V19, H)
        | (V20, M)
        | (V21, M)
        | (V25, L) => &[
            0, 173, 125, 158, 2, 103, 182, 118, 17, 145, 201, 111, 28, 165, 53, 161, 21, 245, 142,
            13, 102, 48, 227, 153, 145, 218, 70,
        ],
        (V02, H)
        | (V06, H)
        | (V10, H)
        | (V11, Q)
        | (V12, H)
        | (V16, M)
        | (V17, L)
        | (V17, M)
        | (V17, Q)
        | (V17, H)
        | (V18, Q)
        | (V18, H)
        | (V19, L)
        | (V20, L)
        | (V20, H)
        | (V21, L)
        | (V21, Q)
        | (V22, L)
        | (V22, M)
        | (V23, M)
        | (V24, M)
        | (V25, M)
        | (V26, L)
        | (V26, M)
        | (V26, Q)
        | (V27, M)
        | (V28, M)
        | (V29, M)
        | (V30, M)
        | (V31, M)
        | (V32, M)
        | (V33, M)
        | (V34, M)
        | (V35, M)
        | (V36, M)
        | (V37, M)
        | (V38, M)
        | (V39, M)
        | (V40, M) => &[
            0, 168, 223, 200, 104, 224, 234, 108, 180, 110, 190, 195, 147, 205, 27, 232, 201, 21,
            43, 245, 87, 42, 195, 212, 119, 242, 37, 9, 123,
        ],
        (V09, L)
        | (V11, M)
        | (V14, L)
        | (V15, Q)
        | (V16, H)
        | (V18, L)
        | (V20, Q)
        | (V21, H)
        | (V22, Q)
        | (V23, L)
        | (V23, Q)
        | (V23, H)
        | (V24, L)
        | (V24, Q)
        | (V24, H)
        | (V25, Q)
        | (V25, H)
        | (V26, H)
        | (V27, L)
        | (V27, Q)
        | (V27, H)
        | (V28, L)
        | (V28, Q)
        | (V28, H)
        | (V29, L)
        | (V29, Q)
        | (V29, H)
        | (V30, L)
        | (V30, Q)
        | (V30, H)
        | (V31, L)
        | (V31, Q)
        | (V31, H)
        | (V32, L)
        | (V32, Q)
        | (V32, H)
        | (V33, L)
        | (V33, Q)
        | (V33, H)
        | (V34, L)
        | (V34, Q)
        | (V34, H)
        | (V35, L)
        | (V35, Q)
        | (V35, H)
        | (V36, L)
        | (V36, Q)
        | (V36, H)
        | (V37, L)
        | (V37, Q)
        | (V37, H)
        | (V38, L)
        | (V38, Q)
        | (V38, H)
        | (V39, L)
        | (V39, Q)
        | (V39, H)
        | (V40, L)
        | (V40, Q)
        | (V40, H) => &[
            0, 41, 173, 145, 152, 216, 31, 179, 182, 50, 48, 110, 86, 239, 96, 222, 125, 42, 173,
            226, 193, 224, 130, 156, 37, 251, 216, 238, 40, 192, 180,
        ],
    }
}

/// Contains the score for [**light/dark module ratio**](https://www.thonky.com/qr-code-tutorial/data-masking#evaluation-condition-4)
pub const PERCENT_SCORE: [u32; 256] = [
    90, 90, 90, 90, 90, 80, 80, 80, 80, 80, 70, 70, 70, 70, 70, 60, 60, 60, 60, 60, 50, 50, 50, 50,
    50, 40, 40, 40, 40, 40, 30, 30, 30, 30, 30, 20, 20, 20, 20, 20, 10, 10, 10, 10, 10, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 10, 10, 10, 10, 10, 20, 20, 20, 20, 20, 30, 30, 30, 30, 30, 40, 40, 40, 40,
    40, 50, 50, 50, 50, 50, 60, 60, 60, 60, 60, 70, 70, 70, 70, 70, 80, 80, 80, 80, 80, 90, 90, 90,
    90, 90, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255,
];

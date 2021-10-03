use crate::bitstring::{self};
use crate::vecl::ECL;
use crate::version::Version;

#[derive(Clone, Copy)]
pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
}

type BitString = bitstring::BitString<23648>;

pub const fn encode(input: &[u8], ecl: ECL, mode: Mode) -> Option<BitString> {
    let version = match Version::get(mode, ecl, input.len()) {
        Some(version) => version,
        None => return None,
    };

    let cci_bits = version.cci_bits(mode);

    let bs = match mode {
        Mode::Numeric => encode_numeric(input, cci_bits),
        Mode::Alphanumeric => encode_alphanumeric(input, cci_bits),
        Mode::Byte => encode_byte(input, cci_bits),
    };

    let bs = match bs {
        Some(bs) => bs,
        None => return None,
    };

    let data_bits = version.data_bits(ecl);

    let bs = add_terminator(bs, data_bits);
    let bs = pad_to_8(bs);
    let bs = fill(bs, data_bits);

    Some(bs)
}

pub const fn best_encoding(input: &[u8]) -> Mode {
    const fn try_encode_numeric(input: &[u8], mut i: usize) -> Mode {
        loop {
            if i == input.len() {
                break;
            }
            if !input[i].is_ascii_digit() {
                return try_encode_alphanumeric(input, i);
            }
            i += 1;
        }
        Mode::Numeric
    }

    const fn try_encode_alphanumeric(input: &[u8], mut i: usize) -> Mode {
        loop {
            if i == input.len() {
                break;
            }
            if !is_qr_alphanumeric(input[i]) {
                return Mode::Byte;
            }
            i += 1;
        }
        Mode::Alphanumeric
    }

    try_encode_numeric(input, 0)
}

const fn encode_numeric(input: &[u8], cci_bits: usize) -> Option<BitString> {
    const fn encode_number(bs: BitString, number: usize) -> BitString {
        match number {
            0..=9 => bitstring::push_bits(bs, number, 4),
            10..=99 => bitstring::push_bits(bs, number, 7),
            /*100..=999*/ _ => bitstring::push_bits(bs, number, 10),
        }
    }

    let bs = BitString::new();

    let bs = bitstring::push_slice(bs, &[false, false, false, true]);

    let mut bs = bitstring::push_bits(bs, input.len(), cci_bits);

    {
        let mut i = 0;
        let len = input.len() - input.len() % 3;

        while i < len {
            let number = ascii_to_digit(input[i]) * 100
                + ascii_to_digit(input[i + 1]) * 10
                + ascii_to_digit(input[i + 2]);

            bs = encode_number(bs, number);

            i += 3;
        }

        if len != input.len() {
            let mut number = 0;

            while i < input.len() {
                number *= 10;

                number += ascii_to_digit(input[i]);

                i += 1;
            }

            bs = encode_number(bs, number);
        }
    }

    Some(bs)
}

const fn encode_alphanumeric(input: &[u8], cci_bits: usize) -> Option<BitString> {
    let bs = BitString::new();

    let bs = bitstring::push_slice(bs, &[false, false, true, false]);

    let mut bs = bitstring::push_bits(bs, input.len(), cci_bits);

    {
        let mut i = 0;
        let len = input.len() - input.len() % 2;

        while i < len {
            let number = ascii_to_alphanumeric(input[i]) * 45 + ascii_to_alphanumeric(input[i + 1]);

            bs = bitstring::push_bits(bs, number, 11);

            i += 2;
        }

        if len != input.len() {
            bs = bitstring::push_bits(bs, ascii_to_alphanumeric(input[i]), 6);
        }
    }

    Some(bs)
}

const fn encode_byte(input: &[u8], cci_bits: usize) -> Option<BitString> {
    let bs = BitString::new();

    let bs = bitstring::push_slice(bs, &[false, true, false, false]);

    let mut bs = bitstring::push_bits(bs, input.len(), cci_bits);

    {
        let mut i = 0;

        while i < input.len() {
            bs = bitstring::push_u8(bs, input[i]);

            i += 1;
        }
    }

    Some(bs)
}

const fn add_terminator(mut bs: BitString, data_bits: usize) -> BitString {
    let mut i = bs.len() - data_bits;

    if i > 4 {
        i = 4;
    }

    while i > 0 {
        bs = bitstring::push(bs, false);

        i -= 1;
    }

    bs
}

const fn pad_to_8(mut bs: BitString) -> BitString {
    let mut i = (8 - bs.len() % 8) % 8;

    while i > 0 {
        bs = bitstring::push(bs, false);

        i -= 1;
    }

    bs
}

const fn fill(mut bs: BitString, data_bits: usize) -> BitString {
    let pad_bytes = [0b11101100, 0b00010001];
    // let pad_bytes = [236, 17];

    let mut byte = false;

    while bs.len() < data_bits {
        bs = bitstring::push_u8(bs, pad_bytes[byte as usize]);
        byte = !byte;
    }

    bs
}

const fn ascii_to_digit(c: u8) -> usize {
    (c - b'0') as usize
}

const fn ascii_to_alphanumeric(c: u8) -> usize {
    match c {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'A' | b'a' => 10,
        b'B' | b'b' => 11,
        b'C' | b'c' => 12,
        b'D' | b'd' => 13,
        b'E' | b'e' => 14,
        b'F' | b'f' => 15,
        b'G' | b'g' => 16,
        b'H' | b'h' => 17,
        b'I' | b'i' => 18,
        b'J' | b'j' => 19,
        b'K' | b'k' => 20,
        b'L' | b'l' => 21,
        b'M' | b'm' => 22,
        b'N' | b'n' => 23,
        b'O' | b'o' => 24,
        b'P' | b'p' => 25,
        b'Q' | b'q' => 26,
        b'R' | b'r' => 27,
        b'S' | b's' => 28,
        b'T' | b't' => 29,
        b'U' | b'u' => 30,
        b'V' | b'v' => 31,
        b'W' | b'w' => 32,
        b'X' | b'x' => 33,
        b'Y' | b'y' => 34,
        b'Z' | b'z' => 35,
        b' ' => 36,
        b'$' => 37,
        b'%' => 38,
        b'*' => 39,
        b'+' => 40,
        b'-' => 41,
        b'.' => 42,
        b'/' => 43,
        b':' => 44,
        _ => usize::MAX, // unreachable!()
    }
}

const fn is_qr_alphanumeric(c: u8) -> bool {
    match c {
        b'A'..=b'Z'
        | b'a'..=b'z'
        | b'0'..=b'9'
        | b' '
        | b'$'
        | b'%'
        | b'*'
        | b'+'
        | b'-'
        | b'.'
        | b'/'
        | b':' => true,
        _ => false,
    }
}

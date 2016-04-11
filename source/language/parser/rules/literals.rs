// Tagua VM
//
//
// New BSD License
//
// Copyright © 2016-2016, Ivan Enderlin.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//     * Redistributions of source code must retain the above copyright
//       notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//     * Neither the name of the Hoa nor the names of its contributors may be
//       used to endorse or promote products derived from this software without
//       specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS AND CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

//! Group of literal rules.

use nom::{digit, oct_digit};
use std::str;
use std::str::FromStr;

named!(
    pub decimal<u64>,
    map_res!(
        digit,
        |string: &[u8]| {
            u64::from_str(unsafe { str::from_utf8_unchecked(string) })
        }
    )
);

named!(
    pub octal<u64>,
    chain!(
        tag!("0") ~
        out: map_res!(
            oct_digit,
            |string: &[u8]| {
                u64::from_str_radix(
                    unsafe { str::from_utf8_unchecked(string) },
                    8
                )
            }
        ),
        || out
    )
);

fn is_identifier_head(character: u8) -> bool {
    match character {
        64...90   => true, // A-Z
        97...122  => true, // a-z
        127...255 => true, // 0x7f-0xff
        b'_'      => true,
        _         => false
    }
}

fn is_identifier_tail(character: u8) -> bool {
    match character {
        48...57 => true, // 0-9
        _       => is_identifier_head(character)
    }
}

named!(
    pub identifier<String>,
    chain!(
        head: take_while1!(is_identifier_head) ~
        tail: take_while!(is_identifier_tail),
        || unsafe { format!("{}{}", str::from_utf8_unchecked(head), str::from_utf8_unchecked(tail)) }
    )
);


#[cfg(test)]
mod tests {
    use nom::IResult::{Done, Error};
    use nom::{Err, ErrorKind};
    use super::{
        decimal,
        octal,
        identifier
    };

    #[test]
    fn case_decimal() {
        assert_eq!(decimal(b"42"), Done(&b""[..], 42u64));
    }

    #[test]
    fn case_octal() {
        assert_eq!(octal(b"052"), Done(&b""[..], 42));
    }

    #[test]
    fn case_invalid_octal_not_starting_by_zero() {
        assert_eq!(octal(b"7"), Error(Err::Position(ErrorKind::Tag, &b"7"[..])));
    }

    #[test]
    fn case_invalid_octal_not_valid_base_range() {
        assert_eq!(octal(b"8"), Error(Err::Position(ErrorKind::Tag, &b"8"[..])));
    }

    #[test]
    fn case_identifier() {
        assert_eq!(identifier(b"abc"), Done(&b""[..], String::from("abc")));
    }
}

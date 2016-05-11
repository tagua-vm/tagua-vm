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
//!
//! The list of all literals is provided by the PHP Language Specification in the [Grammar chapter,
//! Literals section](https://github.com/php/php-langspec/blob/master/spec/19-grammar.md#literals).

use nom::{
    Err,
    ErrorKind,
    IResult,
    hex_digit,
    oct_digit
};
use std::str;
use std::str::FromStr;

named!(
    pub null< Option<()> >,
    map_res!(
        tag!("null"),
        |_: &[u8]| -> Result<Option<()>, ()> {
            Ok(None)
        }
    )
);

named!(
    pub boolean<bool>,
    map_res!(
        alt!(tag!("true") | tag!("false")),
        |string: &[u8]| -> Result<bool, ()> {
            Ok(string[0] == 't' as u8)
        }
    )
);

named!(
    pub binary<u64>,
    map_res!(
        preceded!(
            tag!("0"),
            preceded!(
                alt!(tag!("b") | tag!("B")),
                is_a!("01")
            )
        ),
        |string: &[u8]| {
            u64::from_str_radix(
                unsafe { str::from_utf8_unchecked(string) },
                2
            )
        }
    )
);

named!(
    pub octal<u64>,
    map_res!(
        preceded!(tag!("0"), oct_digit),
        |string: &[u8]| {
            u64::from_str_radix(
                unsafe { str::from_utf8_unchecked(string) },
                8
            )
        }
    )
);

named!(
    pub decimal<u64>,
    map_res!(
        re_bytes_find_static!(r"^[1-9][0-9]*"),
        |string: &[u8]| {
            u64::from_str(unsafe { str::from_utf8_unchecked(string) })
        }
    )
);

named!(
    pub hexadecimal<u64>,
    map_res!(
        preceded!(
            tag!("0"),
            preceded!(
                alt!(tag!("x") | tag!("X")),
                hex_digit
            )
        ),
        |string: &[u8]| {
            u64::from_str_radix(
                unsafe { str::from_utf8_unchecked(string) },
                16
            )
        }
    )
);

named!(
    pub exponential<f64>,
    map_res!(
        re_bytes_find_static!(r"^([0-9]*\.[0-9]+|[0-9]+\.)([eE][+-]?[0-9]+)?"),
        |string: &[u8]| {
            f64::from_str(unsafe { str::from_utf8_unchecked(string) })
        }
    )
);

named!(
    pub string<String>,
    call!(string_single_quoted)
);

#[derive(Debug)]
pub enum StringError {
    TooShort,
    InvalidOpeningCharacter,
    InvalidClosingCharacter,
    InvalidEncoding
}

fn string_single_quoted(input: &[u8]) -> IResult<&[u8], String> {
    if input.len() <= 2 {
        return IResult::Error(Err::Code(ErrorKind::Custom(StringError::TooShort as u32)));
    }

    if input[0] != '\'' as u8 {
        return IResult::Error(Err::Code(ErrorKind::Custom(StringError::InvalidOpeningCharacter as u32)));
    }

    let mut output   = String::new();
    let mut offset   = 1;
    let mut iterator = input[offset..].iter().enumerate();

    while let Some((index, item)) = iterator.next() {
        if *item == '\\' as u8 {
            if let Some((next_index, next_item)) = iterator.next() {
                if *next_item == '\'' as u8 ||
                   *next_item == '\\' as u8 {
                    match str::from_utf8(&input[offset..index + 1]) {
                        Ok(output_tail) => {
                            output.push_str(output_tail);
                            offset = next_index + 1;
                        },

                        Err(_) => {
                            return IResult::Error(Err::Code(ErrorKind::Custom(StringError::InvalidEncoding as u32)));
                        }
                    }
                }
            } else {
                return IResult::Error(Err::Code(ErrorKind::Custom(StringError::InvalidClosingCharacter as u32)));
            }
        } else if *item == '\'' as u8 {
            match str::from_utf8(&input[offset..index + 1]) {
                Ok(output_tail) => {
                    output.push_str(output_tail);

                    return IResult::Done(&input[index + 2..], output);
                },

                Err(_) => {
                    return IResult::Error(Err::Code(ErrorKind::Custom(StringError::InvalidEncoding as u32)));
                }
            }
        }
    }

    IResult::Error(Err::Code(ErrorKind::Custom(StringError::InvalidClosingCharacter as u32)))
}

named!(
    pub identifier,
    re_bytes_find_static!(r"^[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*")
);


#[cfg(test)]
mod tests {
    use nom::IResult::{Done, Error};
    use nom::{Err, ErrorKind};
    use super::{
        StringError,
        binary,
        boolean,
        decimal,
        exponential,
        hexadecimal,
        identifier,
        null,
        octal,
        string
    };

    #[test]
    fn case_null() {
        assert_eq!(null(b"null"), Done(&b""[..], None));
    }

    #[test]
    fn case_boolean_true() {
        assert_eq!(boolean(b"true"), Done(&b""[..], true));
    }

    #[test]
    fn case_boolean_false() {
        assert_eq!(boolean(b"false"), Done(&b""[..], false));
    }

    #[test]
    fn case_binary_lowercase_b() {
        assert_eq!(binary(b"0b101010"), Done(&b""[..], 42u64));
    }

    #[test]
    fn case_binary_uppercase_b() {
        assert_eq!(binary(b"0B101010"), Done(&b""[..], 42u64));
    }

    #[test]
    fn case_invalid_binary_no_number() {
        assert_eq!(binary(b"0b"), Error(Err::Position(ErrorKind::MapRes, &b"0b"[..])));
    }

    #[test]
    fn case_octal() {
        assert_eq!(octal(b"052"), Done(&b""[..], 42u64));
    }

    #[test]
    fn case_invalid_octal_not_starting_by_zero() {
        assert_eq!(octal(b"7"), Error(Err::Position(ErrorKind::Tag, &b"7"[..])));
    }

    #[test]
    fn case_invalid_octal_not_in_base() {
        assert_eq!(octal(b"8"), Error(Err::Position(ErrorKind::Tag, &b"8"[..])));
    }

    #[test]
    fn case_decimal_one_digit() {
        assert_eq!(decimal(b"7"), Done(&b""[..], 7u64));
    }

    #[test]
    fn case_decimal_many_digits() {
        assert_eq!(decimal(b"42"), Done(&b""[..], 42u64));
    }

    #[test]
    fn case_decimal_plus() {
        assert_eq!(decimal(b"42+"), Done(&b"+"[..], 42u64));
    }

    #[test]
    fn case_hexadecimal_lowercase_x() {
        assert_eq!(hexadecimal(b"0x2a"), Done(&b""[..], 42u64));
    }

    #[test]
    fn case_hexadecimal_uppercase_x() {
        assert_eq!(hexadecimal(b"0X2a"), Done(&b""[..], 42u64));
    }

    #[test]
    fn case_hexadecimal_uppercase_alpha() {
        assert_eq!(hexadecimal(b"0x2A"), Done(&b""[..], 42u64));
    }

    #[test]
    fn case_invalid_hexadecimal_no_number() {
        assert_eq!(hexadecimal(b"0x"), Error(Err::Position(ErrorKind::HexDigit, &b""[..])));
    }

    #[test]
    fn case_invalid_hexadecimal_not_in_base() {
        assert_eq!(hexadecimal(b"0xg"), Error(Err::Position(ErrorKind::HexDigit, &b"g"[..])));
    }

    #[test]
    fn case_exponential() {
        assert_eq!(exponential(b"123.456e+78"), Done(&b""[..], 123.456e78f64));
    }

    #[test]
    fn case_exponential_only_with_rational_and_fractional_part() {
        assert_eq!(exponential(b"123.456"), Done(&b""[..], 123.456f64));
    }

    #[test]
    fn case_exponential_only_with_rational_part() {
        assert_eq!(exponential(b"123."), Done(&b""[..], 123.0f64));
    }

    #[test]
    fn case_exponential_only_with_fractional_part() {
        assert_eq!(exponential(b".456"), Done(&b""[..], 0.456f64));
    }

    #[test]
    fn case_exponential_only_with_rational_and_exponent_part_with_lowercase_e() {
        assert_eq!(exponential(b"123.e78"), Done(&b""[..], 123e78f64));
    }

    #[test]
    fn case_exponential_only_with_rational_and_exponent_part_with_uppercase_e() {
        assert_eq!(exponential(b"123.E78"), Done(&b""[..], 123e78f64));
    }

    #[test]
    fn case_exponential_only_with_rational_and_unsigned_exponent_part() {
        assert_eq!(exponential(b"123.e78"), Done(&b""[..], 123e78f64));
    }

    #[test]
    fn case_exponential_only_with_rational_and_positive_exponent_part() {
        assert_eq!(exponential(b"123.e+78"), Done(&b""[..], 123e78f64));
    }

    #[test]
    fn case_exponential_only_with_rational_and_negative_exponent_part() {
        assert_eq!(exponential(b"123.e-78"), Done(&b""[..], 123e-78f64));
    }

    #[test]
    fn case_exponential_only_with_rational_and_negative_zero_exponent_part() {
        assert_eq!(exponential(b"123.e-0"), Done(&b""[..], 123f64));
    }

    #[test]
    fn case_exponential_missing_exponent_part() {
        assert_eq!(exponential(b".7e"), Done(&b"e"[..], 0.7f64));
    }

    #[test]
    fn case_invalid_exponential_only_the_dot() {
        assert_eq!(exponential(b"."), Error(Err::Code(ErrorKind::RegexpFind)));
    }

    #[test]
    fn case_string_single_quoted() {
        assert_eq!(string(b"'foobar'"), Done(&b""[..], String::from("foobar")));
    }

    #[test]
    fn case_string_single_quoted_escaped_quote() {
        assert_eq!(string(b"'foo\\'bar'"), Done(&b""[..], String::from("foo'bar")));
    }

    #[test]
    fn case_string_single_quoted_escaped_backslash() {
        assert_eq!(string(b"'foo\\\\bar'"), Done(&b""[..], String::from("foo\\bar")));
    }

    #[test]
    fn case_string_single_quoted_escaped_any() {
        assert_eq!(string(b"'foo\\nbar'"), Done(&b""[..], String::from("foo\\nbar")));
    }

    #[test]
    fn case_invalid_string_single_quoted_too_short() {
        assert_eq!(string(b"'"), Error(Err::Code(ErrorKind::Custom(StringError::TooShort as u32))));
    }

    #[test]
    fn case_invalid_string_single_quoted_opening_character() {
        assert_eq!(string(b"foobar'"), Error(Err::Code(ErrorKind::Custom(StringError::InvalidOpeningCharacter as u32))));
    }

    #[test]
    fn case_invalid_string_single_quoted_closing_character() {
        assert_eq!(string(b"'foobar"), Error(Err::Code(ErrorKind::Custom(StringError::InvalidClosingCharacter as u32))));
    }

    #[test]
    fn case_invalid_string_single_quoted_closing_character_is_a_backslash() {
        assert_eq!(string(b"'foobar\\"), Error(Err::Code(ErrorKind::Custom(StringError::InvalidClosingCharacter as u32))));
    }

    #[test]
    fn case_identifier() {
        assert_eq!(identifier(b"_fooBar42"), Done(&b""[..], &b"_fooBar42"[..]));
    }

    #[test]
    fn case_identifier_shortest() {
        assert_eq!(identifier(b"x"), Done(&b""[..], &b"x"[..]));
    }

    #[test]
    fn case_identifier_only_head() {
        assert_eq!(identifier(b"aB_\x80"), Done(&b""[..], &b"aB_\x80"[..]));
    }

    #[test]
    fn case_identifier_head_and_tail() {
        assert_eq!(identifier(b"aB_\x80aB7\xff"), Done(&b""[..], &b"aB_\x80aB7\xff"[..]));
    }

    #[test]
    fn case_identifier_copyright() {
        // © = 0xa9
        assert_eq!(identifier(b"\xa9"), Done(&b""[..], &b"\xa9"[..]));
    }

    #[test]
    fn case_identifier_non_breaking_space() {
        //   = 0xa0
        assert_eq!(identifier(b"\xa0"), Done(&b""[..], &b"\xa0"[..]));
    }

    #[test]
    fn case_identifier_invalid() {
        assert_eq!(identifier(b"0x"), Error(Err::Code(ErrorKind::RegexpFind)));
    }
}

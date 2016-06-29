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

//! Group of comment rules.
//!
//! The list of all comments is provided by the PHP Language Specification in
//! the [Grammar chapter, Comments
//! section](https://github.com/php/php-langspec/blob/master/spec/19-grammar.md#comments).

named!(
    pub comment,
    alt!(
        call!(comment_single_line)
      | call!(comment_delimited)
    )
);

named!(
    comment_single_line,
    preceded!(
        alt!(tag!("//") | tag!("#")),
        re_bytes_find_static!(r"^.*?(\r\n|\r|\n|$)")
    )
);

named!(
    comment_delimited,
    preceded!(
        tag!("/*"),
        take_until_and_consume!("*/")
    )
);


#[cfg(test)]
mod tests {
    use nom::IResult::{Done, Error};
    use nom::{Err, ErrorKind};
    use super::{
        comment,
        comment_delimited,
        comment_single_line
    };

    #[test]
    fn case_comment_single_line_double_slash_empty() {
        let input  = b"//";
        let output = Done(&b""[..], &b""[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_with_feed() {
        let input  = b"// foobar\nbazqux";
        let output = Done(&b"bazqux"[..], &b" foobar\n"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_with_carriage_return() {
        let input  = b"// foobar\rbazqux";
        let output = Done(&b"bazqux"[..], &b" foobar\r"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_with_carriage_return_feed() {
        let input  = b"// foobar\r\nbazqux";
        let output = Done(&b"bazqux"[..], &b" foobar\r\n"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_without_ending() {
        let input  = b"// foobar";
        let output = Done(&b""[..], &b" foobar"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_double_slash_embedded() {
        let input  = b"//foo//bar";
        let output = Done(&b""[..], &b"foo//bar"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_hash_empty() {
        let input  = b"#";
        let output = Done(&b""[..], &b""[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_hash_with_line_feed() {
        let input  = b"# foobar\nbazqux";
        let output = Done(&b"bazqux"[..], &b" foobar\n"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_hash_with_carriage_return() {
        let input  = b"# foobar\rbazqux";
        let output = Done(&b"bazqux"[..], &b" foobar\r"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_hash_with_carriage_return_line_feed() {
        let input  = b"# foobar\r\nbazqux";
        let output = Done(&b"bazqux"[..], &b" foobar\r\n"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_hash_without_line_ending() {
        let input  = b"# foobar";
        let output = Done(&b""[..], &b" foobar"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_single_line_hash_embedded() {
        let input  = b"#foo#bar";
        let output = Done(&b""[..], &b"foo#bar"[..]);

        assert_eq!(comment_single_line(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_delimited_empty() {
        let input  = b"/**/xyz";
        let output = Done(&b"xyz"[..], &b""[..]);

        assert_eq!(comment_delimited(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_comment_delimited() {
        let input  = b"/* foo bar\nbaz\r\nqux // hello,\n /*world!*/xyz */";
        let output = Done(&b"xyz */"[..], &b" foo bar\nbaz\r\nqux // hello,\n /*world!"[..]);

        assert_eq!(comment_delimited(input), output);
        assert_eq!(comment(input), output);
    }

    #[test]
    fn case_invalid_comment_delimited_not_closed() {
        let input = b"/*foobar";

        assert_eq!(comment_delimited(input), Error(Err::Position(ErrorKind::TakeUntilAndConsume, &b"foobar"[..])));
        assert_eq!(comment(input), Error(Err::Position(ErrorKind::Alt, &input[..])));
    }
}

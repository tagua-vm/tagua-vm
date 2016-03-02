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

//! The grammar as a set of rules.

use super::ast;
use super::tokens as token;

use nom::digit;
use nom::IResult::Done;
use std::str;
use std::str::FromStr;

/// Hello
named!(
    pub i64_digit<i64>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(
    pub expr<ast::Addition>,
    chain!(
        left: i64_digit ~
        tag!(token::PLUS) ~
        right: i64_digit,
        || { ast::Addition { a: ast::Term { t: left }, b: ast::Term { t: right } } }
    )
);

pub fn root(input: &[u8]) -> ast::Addition {
    match expr(input) {
        Done(_, ast) => ast,
        _ => panic!("Youhouuu")
    }
}


#[cfg(test)]
mod tests {
    use nom::IResult::Done;
    use super::expr;
    use super::i64_digit;
    use super::super::ast;

    #[test]
    fn case_i64_digit() {
        assert_eq!(
            i64_digit(b"42"),
            Done(&b""[..], 42)
        );
    }

    #[test]
    fn case_expr() {
        assert_eq!(
            expr(b"1+2"),
            Done(
                &b""[..], ast::Addition { a: ast::Term { t: 1 }, b: ast::Term { t: 2 } }
            )
        );
    }
}

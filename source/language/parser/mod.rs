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

//! Lexical and syntax analyzers, and AST producer.
//!
//! This is a parser combinator. The immediate consequence is that the lexical
//! and syntax analyzers form a monolithic algorithm. The organization is the
//! following:
//!
//!   * The `tokens` module declares all the lexemes,
//!   * The `rules` module declares the grammar as a set of rules,
//!   * The `ast` module contains the structure that will constitute the AST.
//!
//! The parser is based on [nom](https://github.com/Geal/nom). nom is a parser
//! combinator library with a focus on safe parsing, streaming patterns, and as
//! much as possible zero copy. We try to enforce the zero copy property to
//! hold.

pub mod ast;
pub mod rules;
pub mod tokens;

/// Complete parsing of a datum starting by the sentence symbol of the grammar.
///
/// The grammar is a set of rules. By definition, it has a sentence symbol,
/// also called the root rule. The `parse` function will lex, parse and produce
/// the associated AST of the `input` datum.
///
/// # Examples
///
/// ```ignore
/// use tagua_vm::language::parser;
///
/// let expression = b"1+2";
/// parser::parse(&expression[..]);
/// ```
pub fn parse(input: &[u8]) -> ast::Addition {
    rules::root(input)
}

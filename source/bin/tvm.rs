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

//! Binary to drive the `tagua_vm` library.

extern crate tagua_vm;

use tagua_vm::language;
use tagua_vm::shared::VERSION;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

enum ExitCode {
    Ok,
    InvalidOption,
    MissingFile,
    InvalidFile,
    MultipleFiles,
    Panic
}

fn usage() -> String {
    "Usage: tvm [options] [file]\
    \nOptions:\
    \n    -v, --version    Print version.\
    \n    -h, --help       This help.".to_string()
}

fn version() -> String {
    format!("Tagua VM v{}", VERSION)
}

fn file(filename: &str) {
    match File::open(filename) {
        Ok(mut file) => {
            let mut buffer = Vec::new();

            match file.read_to_end(&mut buffer) {
                Ok(_) =>
                    language::compiler::vm::compile(
                        language::parser::parse(&buffer[..])
                    ),

                Err(error) => {
                    println!(
                        "Error while reading file {}: {:?}.",
                        filename,
                        error
                    );
                    exit(ExitCode::Panic);
                }
            }
        },

        Err(error) => {
            println!(
                "Could not open input file {}; reason: {}.",
                filename,
                error
            );
            exit(ExitCode::InvalidFile);
        }
    };
}

fn exit(code: ExitCode) {
    process::exit(code as i32);
}

pub fn process_options(arguments: Vec<String>) {
    let mut input = None;

    for argument in arguments {
        match argument.chars().next() {
            Some('-') =>
                match argument.as_ref() {
                    "-v" | "--version" => {
                        println!("{}", version());
                        exit(ExitCode::Ok);
                    },

                    "-h" | "--help" => {
                        println!("{}", usage());
                        exit(ExitCode::Ok);
                    },

                    _ => {
                        println!("Invalid option \"{}\".\n", argument);
                        println!("{}", usage());
                        exit(ExitCode::InvalidOption);
                    }
                },

            Some(_) => {
                if input == None
                {
                    input = Some(argument);
                }
                else
                {
                    println!("Multiple input files\n");
                    println!("{}", usage());
                    exit(ExitCode::MultipleFiles);
                }
            }

            None => {
                println!("{}", usage());
                exit(ExitCode::Panic);
            }
        }
    }

    if let Some(f) = input {
        file(&f[..]);
    }
    else {
        println!("No file provided.\n");
        println!("{}", usage());
        exit(ExitCode::MissingFile);
    }
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    process_options(arguments);
}

#[cfg(test)]
mod tests {
    use tagua_vm::shared::VERSION;
    use super::usage;
    use super::version;

    #[test]
    fn case_usage() {
        assert_eq!(
            "Usage: tvm [options] [file]\nOptions:\n    -v, --version    \
            Print version.\n    -h, --help       This help.".to_string(),
            usage()
        );
    }

    #[test]
    fn case_version() {
        assert_eq!(
            format!("Tagua VM v{}", VERSION),
            version()
        );
    }
}

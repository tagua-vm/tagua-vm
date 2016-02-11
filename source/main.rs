/**
 * Tagua VM
 *
 *
 * New BSD License
 * 
 * Copyright © 2016-2016, Ivan Enderlin.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *     * Neither the name of the Hoa nor the names of its contributors may be
 *       used to endorse or promote products derived from this software without
 *       specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS AND CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

use std::env;
use std::process::exit;

const VERSION: &'static str = "0.0.1";

enum ExitCode {
    Ok,
    InvalidOption,
    MissingFile,
    Panic
}

fn usage(exit_code: ExitCode) {
    println!("Usage: tvm [options] [file]");
    println!("Options:");
    println!("    -v, --version    Print version.");
    println!("    -h, --help       This help.");

    exit(exit_code as i32);
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut input              = "";

    for argument in &arguments[1..] {
        match argument.chars().next() {
            Some('-') =>
                match argument.as_ref() {
                    "-v" | "--version" => {
                        println!("Tagua VM v{}", VERSION);
                        exit(0)
                    }

                    "-h" | "--help" =>
                        usage(ExitCode::Ok),

                    _ => {
                        println!("Invalid option “{}”.\n", argument);
                        usage(ExitCode::InvalidOption)
                    }
                },

            Some(_) =>
                input = argument,

            None =>
                usage(ExitCode::Panic)
        }
    }

    if input.is_empty() {
        println!("No file provided.\n");
        usage(ExitCode::MissingFile);
    }

    println!("File to run is {}", input);
}

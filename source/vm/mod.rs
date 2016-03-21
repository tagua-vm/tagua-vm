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

//! Virtual machine implementation.
//!
//! The virtual machine is based on [the LLVM Compiler
//! Infrastructure](http://llvm.org/). Thus, it is based on:
//!
//!   1. An intermediate representation, called IR, which is a small
//!      intermediate typed-language,
//!   2. An execution engine responsible to execute this IR.
//!
//! This module provides safe bindings to LLVM designed for Tagua VM. The API
//! does not aim at being a generic safe bindings implementation. One might not
//! feel lost if already familiar with LLVM infrastructure. The basis of the
//! infrastructure is the context and the module, functions are declared inside
//! a module and the execution engine is MCJIT (see [MCJIT Design and
//! Implementation](http://llvm.org/docs/MCJITDesignAndImplementation.html)).

pub mod builder;
pub mod context;
pub mod engine;
pub mod function;
pub mod module;
pub mod native_type;
pub mod value;

pub trait LLVMRef<R> {
    fn to_ref(&self) -> R;
}

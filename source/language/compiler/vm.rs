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

//! Transform an AST to VM intermediate representation.

use llvm::native_type::VMRepresentation;
use llvm;
use parser::ast;

/// Compile an AST to VM intermediate representation.
pub fn compile(ast: ast::Addition) {
    let context     = llvm::context::Context::new();
    let mut module  = llvm::module::Module::new("foobar", &context);
    let mut builder = llvm::builder::Builder::new(&context);
    let function    = llvm::function::Function::new(
        &module,
        "f",
        &mut [],
        llvm::native_type::int64_type(&context)
    );
    let basic_block = function.new_basic_block("entry");
    builder.move_to_end(basic_block);
    let addition = builder.add(
        7u64.to_vm_representation(&context),
        42u64.to_vm_representation(&context),
        "addition"
    );
    builder.return_value(addition);

    let engine_result = llvm::engine::Engine::new(
        &mut module,
        &llvm::engine::Options {
            level     : llvm::engine::OptimizationLevel::NoOptimizations,
            code_model: llvm::engine::CodeModel::Default
        }
    );

    match engine_result {
        Ok(engine) =>
            println!(
                "THE result is {}.",
                engine.run_function(&function, &mut [])
            ),

        Err(_) =>
            panic!(
                "Cannot execute the following module:\n{}",
                module
            )
    }
}

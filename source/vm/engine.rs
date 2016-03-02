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

//! Execute the VM intermediate representation.
//!
//! An execution engine is responsible to execute the VM intermediate
//! representation, i.e. to compile it to machine code. The execution engine is
//! MCJIT (see [MCJIT Design and
//! Implementation](http://llvm.org/docs/MCJITDesignAndImplementation.html)).

use super::LLVMRef;
use super::function::Function;
use super::module::Module;

use libc::{c_char, c_uint, c_ulonglong};
use llvm::core::LLVMDisposeMessage;
use llvm::execution_engine::{
    LLVMCreateMCJITCompilerForModule,
    LLVMDisposeExecutionEngine,
    LLVMExecutionEngineRef,
    LLVMGenericValueRef,
    LLVMGenericValueToInt,
    LLVMLinkInMCJIT,
    LLVMMCJITCompilerOptions,
    LLVMRunFunction,
};
use llvm::target::{
    LLVM_InitializeNativeAsmPrinter,
    LLVM_InitializeNativeTarget
};
use llvm::target_machine::LLVMCodeModel;
use std::ffi::CStr;
use std::{mem, ptr};

#[derive(Clone)]
pub enum OptimizationLevel {
    NoOptimizations,
    Level1,
    Level2,
    Level3
}

pub enum CodeModel {
    Default,
    JITDefault,
    Kernel,
    Large,
    Medium,
    Small
}

pub struct Options {
    pub level     : OptimizationLevel,
    pub code_model: CodeModel
}

pub struct Engine {
    engine: LLVMExecutionEngineRef,
    owned : bool
}

impl Engine {
    pub fn new(module: &mut Module, options: &Options) -> Result<Engine, String> {
        let mut engine_options = LLVMMCJITCompilerOptions {
            OptLevel : options.level.clone() as c_uint,
            CodeModel: match options.code_model {
                CodeModel::Default    => LLVMCodeModel::LLVMCodeModelDefault,
                CodeModel::JITDefault => LLVMCodeModel::LLVMCodeModelJITDefault,
                CodeModel::Kernel     => LLVMCodeModel::LLVMCodeModelKernel,
                CodeModel::Large      => LLVMCodeModel::LLVMCodeModelLarge,
                CodeModel::Medium     => LLVMCodeModel::LLVMCodeModelMedium,
                CodeModel::Small      => LLVMCodeModel::LLVMCodeModelSmall
            },
            NoFramePointerElim: 0,
            EnableFastISel    : 1,
            MCJMM             : ptr::null_mut()
        };
        let engine_options_size = mem::size_of::<LLVMMCJITCompilerOptions>();
        let mut engine_ref;
        let mut engine_error = 0 as *mut c_char;

        unsafe {
            LLVMLinkInMCJIT();

            if 1 == LLVM_InitializeNativeTarget() {
                return Err("Cannot initialize LLVM native target.".to_string())
            }

            if 1 == LLVM_InitializeNativeAsmPrinter() {
                return Err("Cannot initialize LLVM native ASM printer.".to_string())
            }
        }

        let engine_status;

        unsafe {
            module.unown();
            engine_ref    = mem::uninitialized();
            engine_status = LLVMCreateMCJITCompilerForModule(
                &mut engine_ref,
                module.to_ref(),
                &mut engine_options,
                engine_options_size as usize,
                &mut engine_error
            );
        }

        if 1 == engine_status {
            let error;

            unsafe {
                error = CStr::from_ptr(engine_error).to_string_lossy().into_owned();
                LLVMDisposeMessage(engine_error);
            }

            Err(error)
        } else {
            Ok(
                Engine {
                    engine: engine_ref,
                    owned : true
                }
            )
        }
    }

    pub fn run_function(&self, function: &Function, arguments: &mut [LLVMGenericValueRef]) -> c_ulonglong {
        unsafe {
            LLVMGenericValueToInt(
                LLVMRunFunction(
                    self.engine,
                    function.to_ref(),
                    arguments.len() as c_uint,
                    arguments.as_mut_ptr()
                ),
                0
            )
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                LLVMDisposeExecutionEngine(self.engine);
            }
        }
    }
}

impl LLVMRef<LLVMExecutionEngineRef> for Engine {
    fn to_ref(&self) -> LLVMExecutionEngineRef {
        self.engine
    }
}


#[cfg(test)]
mod tests {
    use super::CodeModel;
    use super::Engine;
    use super::OptimizationLevel;
    use super::Options;
    use super::super::builder::Builder;
    use super::super::context::Context;
    use super::super::function::Function;
    use super::super::module::Module;
    use super::super::native_type::{
        VMRepresentation,
        int8_type
    };

    #[test]
    fn case_ownership() {
        let context    = Context::new();
        let mut module = Module::new("foobar", &context);
        let result     = Engine::new(
            &mut module,
            &Options {
                level     : OptimizationLevel::NoOptimizations,
                code_model: CodeModel::Default
            }
        );

        match result {
            Ok(engine) =>
                assert!(engine.owned),

            Err(_) =>
                assert!(false)
        }
    }

    #[test]
    fn case_run_function() {
        let context     = Context::new();
        let mut module  = Module::new("foobar", &context);
        let mut builder = Builder::new(&context);
        let function    = Function::new(&module, "f", &mut [], int8_type(&context));
        let basic_block = function.new_basic_block("entry");
        builder.move_to_end(basic_block);
        builder.return_value(7u8.to_vm_representation(&context));

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "define i8 @f() {\n" +
            "entry:\n" +
            "  ret i8 7\n" +
            "}\n",
            format!("{}", module)
        );

        let result = Engine::new(
            &mut module,
            &Options {
                level     : OptimizationLevel::NoOptimizations,
                code_model: CodeModel::Default
            }
        );

        match result {
            Ok(engine) => {
                let returned_value = engine.run_function(
                    &function,
                    &mut []
                );

                assert_eq!(7, returned_value);
            }

            Err(_) =>
                assert!(false)
        }
    }
}

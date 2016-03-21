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

//! Function manipulation.

use super::LLVMRef;
use super::module::Module;
use super::builder::BasicBlock;

use libc::c_char;
use llvm::analysis::{
    LLVMVerifierFailureAction,
    LLVMVerifyFunction
};
use llvm::core::{
    LLVMAddFunction,
    LLVMAppendBasicBlockInContext,
    LLVMCountParams,
    LLVMFunctionType,
    LLVMGetTypeContext,
    LLVMTypeOf
};
use llvm::prelude::{
    LLVMBool,
    LLVMTypeRef,
    LLVMValueRef
};
use std::ffi::CString;

pub struct Function {
    function: LLVMValueRef
}

impl Function {
    pub fn new(module: &Module, function_name: &str, function_arguments: &mut [LLVMTypeRef], function_return: LLVMTypeRef) -> Function {
        let function_name = CString::new(function_name).unwrap();
        let function_type = unsafe {
            LLVMFunctionType(
                function_return,
                function_arguments.as_mut_ptr(),
                function_arguments.len() as u32,
                0 as LLVMBool
            )
        };

        Function {
            function: unsafe {
                LLVMAddFunction(
                    module.to_ref(),
                    function_name.as_ptr() as *const c_char,
                    function_type
                )
            }
        }
    }

    pub fn new_basic_block(&self, basic_block_name: &str) -> BasicBlock {
        let basic_block_name = CString::new(basic_block_name).unwrap();

        BasicBlock::from_ref(
            unsafe {
                LLVMAppendBasicBlockInContext(
                    LLVMGetTypeContext(
                        LLVMTypeOf(
                            self.to_ref()
                        )
                    ),
                    self.to_ref(),
                    basic_block_name.as_ptr() as *const c_char
                )
            }
        )
    }

    pub fn arity(&self) -> u32 {
        unsafe {
            LLVMCountParams(self.to_ref()) as u32
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        let status;

        unsafe {
            status = LLVMVerifyFunction(
                self.to_ref(),
                LLVMVerifierFailureAction::LLVMReturnStatusAction
            )
        }

        if 1 == status {
            Err("Unknown error".to_string())
        } else {
            Ok(())
        }
    }
}

impl LLVMRef<LLVMValueRef> for Function {
    fn to_ref(&self) -> LLVMValueRef {
        self.function
    }
}


#[cfg(test)]
mod tests {
    use super::Function;
    use super::super::builder::Builder;
    use super::super::context::Context;
    use super::super::module::Module;
    use super::super::native_type::{
        VMRepresentation,
        array_type,
        double_type,
        int1_type,
        int8_type,
        void_type
    };

    #[test]
    fn case_declare_void_void() {
        let context   = Context::new();
        let module    = Module::new("foobar", &context);
        let _function = Function::new(
            &module,
            "f",
            &mut [],
            void_type(&context)
        );

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "declare void @f()\n",
            format!("{}", module)
        );
    }

    #[test]
    fn case_declare_int8_array_double() {
        let context   = Context::new();
        let module    = Module::new("foobar", &context);
        let _function = Function::new(
            &module,
            "f",
            &mut [int8_type(&context), array_type(int1_type(&context), 7)],
            double_type(&context)
        );

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "declare double @f(i8, [7 x i1])\n",
            format!("{}", module)
        );
    }

    #[test]
    fn case_define_void_void() {
        let context  = Context::new();
        let module   = Module::new("foobar", &context);
        let function = Function::new(
            &module,
            "f",
            &mut [],
            void_type(&context)
        );
        function.new_basic_block("entry");

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "define void @f() {\n" +
            "entry:\n" +
            "}\n",
            format!("{}", module)
        );

        // The block has no terminator, so it must fail.
        match function.verify() {
            Ok(_) =>
                assert!(false),

            Err(_) =>
                assert!(true)
        }
    }

    macro_rules! test_arity {
        ($test_case_name:ident: (#$function:ident([$($argument_types:ident),*]) -> $return_type:ident, $arity:expr)) => (
            #[test]
            fn $test_case_name() {
                let context  = Context::new();
                let module   = Module::new("foobar", &context);
                let function = Function::new(
                    &module,
                    stringify!($function),
                    &mut [$($argument_types(&context)),*],
                    $return_type(&context)
                );

                assert_eq!(
                    $arity,
                    function.arity()
                );
            }
        )
    }

    test_arity!(case_arity_zero: (#f([]) -> void_type, 0));
    test_arity!(case_arity_one : (#f([int1_type]) -> void_type, 1));
    test_arity!(case_arity_two : (#f([int1_type, int1_type]) -> void_type, 2));

    #[test]
    fn case_verify() {
        let mut context = Context::new();
        let mut module  = Module::new("foobar", &mut context);
        let mut builder = Builder::new(&mut context);
        let function    = Function::new(&mut module, "fgi", &mut[], int8_type(&context));
        let basic_block = function.new_basic_block("entry");
        builder.move_to_end(basic_block);
        let addition = builder.add(
            7u8.to_vm_representation(&context),
            42u8.to_vm_representation(&context),
            "addition"
        );
        builder.return_value(addition);

        match function.verify() {
            Ok(_) =>
                assert!(true),

            Err(_) =>
                assert!(false)
        }
    }
}

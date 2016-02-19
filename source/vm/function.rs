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

use super::LLVMRef;
use super::module::Module;

use libc::c_char;
use llvm::core::{
    LLVMAddFunction,
    LLVMDeleteFunction,
    LLVMFunctionType
};
use llvm::prelude::{
    LLVMBool,
    LLVMTypeRef,
    LLVMValueRef
};
use std::ffi::CString;

pub struct Function {
    function: LLVMValueRef,
    owned   : bool
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
            },
            owned: true
        }
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                LLVMDeleteFunction(self.function);
            }
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
    use super::super::context::Context;
    use super::super::module::Module;
    use super::super::native_type::{
        void_type,
        int1_type,
        int8_type,
        double_type,
        array_type
    };

    #[test]
    fn case_ownership() {
        let context  = Context::new();
        let module   = Module::new("foobar", &context);
        let function = Function::new(
            &module,
            "f",
            &mut [],
            void_type()
        );

        assert!(function.owned);
    }

    #[test]
    fn case_intermediate_representation_void_void() {
        let context   = Context::new();
        let module    = Module::new("foobar", &context);
        let _function = Function::new(
            &module,
            "f",
            &mut [],
            void_type()
        );

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "declare void @f()\n",
            format!("{}", module)
        );
    }

    #[test]
    fn case_intermediate_representation_int8_array_double() {
        let context   = Context::new();
        let module    = Module::new("foobar", &context);
        let _function = Function::new(
            &module,
            "f",
            &mut [int8_type(), array_type(int1_type(), 7)],
            double_type()
        );

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "declare double @f(i8, [7 x i1])\n",
            format!("{}", module)
        );
    }
}

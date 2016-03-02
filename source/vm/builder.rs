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

//! Build blocks, statements, … of all kinds.

use super::LLVMRef;
use super::context::Context;
use super::value::Value;

use libc::c_char;
use llvm::core::{
    LLVMBuildAdd,
    LLVMBuildRet,
    LLVMBuildRetVoid,
    LLVMCreateBuilderInContext,
    LLVMDisposeBuilder,
    LLVMPositionBuilderAtEnd
};
use llvm::prelude::{
    LLVMBasicBlockRef,
    LLVMBuilderRef
};
use std::ffi::CString;

#[derive(Debug)]
pub struct Builder {
    builder: LLVMBuilderRef,
    owned  : bool
}

impl Builder {
    pub fn new(context: &Context) -> Builder {
        Builder {
            builder: unsafe {
                LLVMCreateBuilderInContext(context.to_ref())
            },
            owned: true
        }
    }

    pub fn move_to_end(&mut self, basic_block: BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.to_ref(), basic_block.to_ref());
        }
    }

    pub fn return_void(&mut self) -> Value {
        Value::from_ref(
            unsafe {
                LLVMBuildRetVoid(self.to_ref())
            }
        )
    }

    pub fn return_value(&mut self, value: Value) -> Value {
        Value::from_ref(
            unsafe {
                LLVMBuildRet(self.to_ref(), value.to_ref())
            }
        )
    }

    pub fn add(&mut self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();

        Value::from_ref(
            unsafe {
                LLVMBuildAdd(
                    self.to_ref(),
                    lhs.to_ref(),
                    rhs.to_ref(),
                    name.as_ptr() as *const c_char
                )
            }
        )
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                LLVMDisposeBuilder(self.builder);
            }
        }
    }
}

impl LLVMRef<LLVMBuilderRef> for Builder {
    fn to_ref(&self) -> LLVMBuilderRef {
        self.builder
    }
}

pub struct BasicBlock {
    block: LLVMBasicBlockRef
}

impl BasicBlock {
    pub fn from_ref(basic_block_ref: LLVMBasicBlockRef) -> BasicBlock {
        BasicBlock {
            block: basic_block_ref
        }
    }
}

impl LLVMRef<LLVMBasicBlockRef> for BasicBlock {
    fn to_ref(&self) -> LLVMBasicBlockRef {
        self.block
    }
}


#[cfg(test)]
mod tests {
    use super::Builder;
    use super::super::context::Context;
    use super::super::function::Function;
    use super::super::module::Module;
    use super::super::native_type::{
        VMRepresentation,
        int1_type,
        int8_type,
        void_type
    };

    #[test]
    fn case_ownership() {
        let context = Context::new();
        let builder = Builder::new(&context);

        assert!(builder.owned);
    }

    #[test]
    fn case_return_void() {
        let context     = Context::new();
        let module      = Module::new("foobar", &context);
        let mut builder = Builder::new(&context);
        let function    = Function::new(&module, "f", &mut [], void_type(&context));
        let basic_block = function.new_basic_block("entry");
        builder.move_to_end(basic_block);
        builder.return_void();

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "define void @f() {\n" +
            "entry:\n" +
            "  ret void\n" +
            "}\n",
            format!("{}", module)
        );
    }

    #[test]
    fn case_return_bool() {
        let context     = Context::new();
        let module      = Module::new("foobar", &context);
        let mut builder = Builder::new(&context);
        let function    = Function::new(&module, "f", &mut [], int1_type(&context));
        let basic_block = function.new_basic_block("entry");
        builder.move_to_end(basic_block);
        builder.return_value(true.to_vm_representation(&context));

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "define i1 @f() {\n" +
            "entry:\n" +
            "  ret i1 true\n" +
            "}\n",
            format!("{}", module)
        );
    }

    #[test]
    fn case_return_integer() {
        let context     = Context::new();
        let module      = Module::new("foobar", &context);
        let mut builder = Builder::new(&context);
        let function    = Function::new(&module, "f", &mut [], int8_type(&context));
        let basic_block = function.new_basic_block("entry");
        builder.move_to_end(basic_block);
        builder.return_value(42u8.to_vm_representation(&context));

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "define i8 @f() {\n" +
            "entry:\n" +
            "  ret i8 42\n" +
            "}\n",
            format!("{}", module)
        );
    }

    #[test]
    fn case_add_constants() {
        let context     = Context::new();
        let module      = Module::new("foobar", &context);
        let mut builder = Builder::new(&context);
        let function    = Function::new(&module, "f", &mut [], int8_type(&context));
        let basic_block = function.new_basic_block("entry");
        builder.move_to_end(basic_block);
        let addition = builder.add(
            7u8.to_vm_representation(&context),
            42u8.to_vm_representation(&context),
            "addition"
        );
        builder.return_value(addition);

        assert_eq!(
            "; ModuleID = 'foobar'\n".to_string() +
            "\n" +
            "define i8 @f() {\n" +
            "entry:\n" +
            "  ret i8 49\n" +
            "}\n",
            format!("{}", module)
        );
    }
}

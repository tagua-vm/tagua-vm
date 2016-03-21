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

//! Module manipulation.

use super::LLVMRef;
use super::context::Context;

use libc::c_char;
use llvm::analysis::{
    LLVMVerifierFailureAction,
    LLVMVerifyModule
};
use llvm::core::{
    LLVMDisposeMessage,
    LLVMDisposeModule,
    LLVMModuleCreateWithNameInContext,
    LLVMPrintModuleToString
};
use llvm::prelude::LLVMModuleRef;
use std::ffi::{CStr, CString};
use std::fmt;

pub struct Module {
    module: LLVMModuleRef,
    owned : bool
}

impl Module {
    pub fn new(module_id: &str, context: &Context) -> Module {
        let module_id = CString::new(module_id).unwrap();

        Module {
            module: unsafe {
                LLVMModuleCreateWithNameInContext(
                    module_id.as_ptr() as *const c_char,
                    context.to_ref()
                )
            },
            owned: true
        }
    }

    pub unsafe fn unown(&mut self) {
        self.owned = false;
    }

    pub fn verify(&self) -> Result<(), String> {
        let mut verify_error = 0 as *mut c_char;
        let status;

        unsafe {
            status = LLVMVerifyModule(
                self.to_ref(),
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
                &mut verify_error
            )
        }

        if 1 == status {
            let error;

            unsafe {
                let error_buffer = CStr::from_ptr(verify_error);
                error = String::from_utf8_lossy(error_buffer.to_bytes()).into_owned();
                LLVMDisposeMessage(verify_error);
            }

            Err(error)
        } else {
            Ok(())
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                LLVMDisposeModule(self.module);
            }
        }
    }
}

impl LLVMRef<LLVMModuleRef> for Module {
    fn to_ref(&self) -> LLVMModuleRef {
        self.module
    }
}

impl fmt::Display for Module {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            unsafe {
                let ir_as_c_string = LLVMPrintModuleToString(self.to_ref());
                let ir             = CStr::from_ptr(ir_as_c_string).to_string_lossy().into_owned();

                LLVMDisposeMessage(ir_as_c_string);

                ir
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::Module;
    use super::super::context::Context;

    #[test]
    fn case_ownership() {
        let context = Context::new();
        let module  = Module::new("foobar", &context);

        assert!(module.owned);
    }

    #[test]
    fn case_id() {
        let context = Context::new();
        let module  = Module::new("foobar", &context);

        assert_eq!(
            "; ModuleID = 'foobar'\n",
            format!("{}", module)
        );
    }

    #[test]
    fn case_verify() {
        let context = Context::new();
        let module  = Module::new("foobar", &context);

        match module.verify() {
            Ok(_) =>
                assert!(true),

            Err(_) =>
                assert!(false)
        }
    }
}

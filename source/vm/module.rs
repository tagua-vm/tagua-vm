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
use super::context::Context;

use libc::c_char;
use llvm::core::{
    LLVMDisposeMessage,
    LLVMDisposeModule,
    LLVMModuleCreateWithNameInContext,
    LLVMPrintModuleToString
};
use llvm::prelude::LLVMModuleRef;
use std::ffi::{CStr, CString};
use std::{fmt, str};

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
                let module_as_c_string = LLVMPrintModuleToString(self.to_ref());
                let module_as_c_str    = CStr::from_ptr(module_as_c_string);

                LLVMDisposeMessage(module_as_c_string);

                str::from_utf8_unchecked(module_as_c_str.to_bytes())
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
    fn case_format_display() {
        let context = Context::new();
        let module  = Module::new("foobar", &context);

        assert_eq!(
            "; ModuleID = 'foobar'\n",
            format!("{}", module)
        );
    }
}

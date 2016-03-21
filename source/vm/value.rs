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

//! Value manipulation.

use super::LLVMRef;

use llvm::core::{
    LLVMDisposeMessage,
    LLVMPrintValueToString
};
use llvm::prelude::LLVMValueRef;
use std::ffi::CStr;
use std::fmt;

pub struct Value {
    value: LLVMValueRef
}

impl Value {
    pub fn from_ref(value: LLVMValueRef) -> Value {
        Value {
            value: value
        }
    }
}

impl LLVMRef<LLVMValueRef> for Value {
    fn to_ref(&self) -> LLVMValueRef {
        self.value
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            unsafe {
                let ir_as_c_string = LLVMPrintValueToString(self.to_ref());
                let ir             = CStr::from_ptr(ir_as_c_string).to_string_lossy().into_owned();

                LLVMDisposeMessage(ir_as_c_string);

                ir
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::context::Context;
    use super::super::native_type::VMRepresentation;

    #[test]
    fn case_display() {
        let context = Context::new();
        let value   = true.to_vm_representation(&context);

        assert_eq!(
            "i1 true",
            format!("{}", value)
        );
    }
}

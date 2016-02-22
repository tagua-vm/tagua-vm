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

use libc::c_ulonglong;
use llvm::core::{
    LLVMConstInt,
    LLVMInt1TypeInContext,
    LLVMInt8TypeInContext
};
use llvm::prelude::{
    LLVMBool,
    LLVMValueRef
};

pub trait Constant {
    fn as_vm_constant(self, context: &Context) -> LLVMValueRef;
}

impl Constant for bool {
    fn as_vm_constant(self, context: &Context) -> LLVMValueRef {
        unsafe {
            LLVMConstInt(
                LLVMInt1TypeInContext(context.to_ref()),
                self as c_ulonglong,
                0 as LLVMBool
            )
        }
    }
}

impl Constant for u8 {
    fn as_vm_constant(self, context: &Context) -> LLVMValueRef {
        unsafe {
            LLVMConstInt(
                LLVMInt8TypeInContext(context.to_ref()),
                self as c_ulonglong,
                0 as LLVMBool
            )
        }
    }
}

impl Constant for i8 {
    fn as_vm_constant(self, context: &Context) -> LLVMValueRef {
        unsafe {
            LLVMConstInt(
                LLVMInt8TypeInContext(context.to_ref()),
                self as c_ulonglong,
                0 as LLVMBool
            )
        }
    }
}

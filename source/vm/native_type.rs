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

use libc::c_uint;
use llvm::prelude::LLVMTypeRef;

macro_rules! bind_type {
    ($LLVMName:ident, $name:ident) => (
        pub fn $name() -> LLVMTypeRef {
            use llvm::core::$LLVMName;

            unsafe {
                $LLVMName()
            }
        }
    )
}

bind_type!(LLVMInt1Type,     int1_type);
bind_type!(LLVMInt8Type,     int8_type);
bind_type!(LLVMInt16Type,    int16_type);
bind_type!(LLVMInt32Type,    int32_type);
bind_type!(LLVMInt64Type,    int64_type);
bind_type!(LLVMDoubleType,   double_type);
bind_type!(LLVMFloatType,    float_type);
bind_type!(LLVMFP128Type,    fp128_type);
bind_type!(LLVMPPCFP128Type, ppcfp128_type);
bind_type!(LLVMVoidType,     void_type);
bind_type!(LLVMX86FP80Type,  x86fp80_type);
bind_type!(LLVMX86MMXType,   x86mmx_type);

pub fn int_type(size: u32) -> LLVMTypeRef {
    use llvm::core::LLVMIntType;

    unsafe {
        LLVMIntType(size as c_uint)
    }
}

pub fn array_type(elements_type: LLVMTypeRef, size: u32) -> LLVMTypeRef {
    use llvm::core::LLVMArrayType;

    unsafe {
        LLVMArrayType(elements_type, size as c_uint)
    }
}

pub fn pointer_type(element_type: LLVMTypeRef, address_space: u32) -> LLVMTypeRef {
    use llvm::core::LLVMPointerType;

    unsafe {
        LLVMPointerType(element_type, address_space as c_uint)
    }
}

pub fn vector_type(elements_type: LLVMTypeRef, size: u32) -> LLVMTypeRef {
    use llvm::core::LLVMVectorType;

    unsafe {
        LLVMVectorType(elements_type, size as c_uint)
    }
}

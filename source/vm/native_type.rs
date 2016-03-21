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

//! LLVM types manipulation and bindings from Rust types.

use super::LLVMRef;
use super::context::Context;
use super::value::Value;

use libc::{c_char, c_uint, c_ulonglong};
use llvm::core::{
    LLVMConstInt,
    LLVMConstReal,
    LLVMConstStringInContext
};
use llvm::prelude::{
    LLVMBool,
    LLVMTypeRef
};
use std::mem;

macro_rules! bind_type {
    ($LLVM_name:ident => $name:ident; $documentation:expr) => (
        #[doc=$documentation]
        pub fn $name(context: &Context) -> LLVMTypeRef {
            use llvm::core::$LLVM_name;

            unsafe {
                $LLVM_name(context.to_ref())
            }
        }
    )
}

bind_type!(LLVMInt1TypeInContext     => int1_type; "Create a LLVM `int1` type.");
bind_type!(LLVMInt8TypeInContext     => int8_type; "Create a LLVM `int8` type.");
bind_type!(LLVMInt16TypeInContext    => int16_type; "Create a LLVM `int16` type.");
bind_type!(LLVMInt32TypeInContext    => int32_type; "Create a LLVM `int32` type.");
bind_type!(LLVMInt64TypeInContext    => int64_type; "Create a LLVM `int64` type.");
bind_type!(LLVMDoubleTypeInContext   => double_type; "Create a LLVM `double` type.");
bind_type!(LLVMFloatTypeInContext    => float_type; "Create a LLVM `float` type.");
bind_type!(LLVMFP128TypeInContext    => fp128_type; "Create a LLVM `fp128` type.");
bind_type!(LLVMPPCFP128TypeInContext => ppcfp128_type; "Create a LLVM `ppcfp128` type.");
bind_type!(LLVMVoidTypeInContext     => void_type; "Create a LLVM `void` type.");
bind_type!(LLVMX86FP80TypeInContext  => x86fp80_type; "Create a LLVM `x86fp80` type.");
bind_type!(LLVMX86MMXTypeInContext   => x86mmx_type; "Create a LLVM `x86mmx` type.");

/// Create a LLVM `int` type.
pub fn int_type(size: u32, context: &Context) -> LLVMTypeRef {
    use llvm::core::LLVMIntTypeInContext;

    unsafe {
        LLVMIntTypeInContext(context.to_ref(), size as c_uint)
    }
}

/// Create a LLVM `array` type.
pub fn array_type(elements_type: LLVMTypeRef, size: u32) -> LLVMTypeRef {
    use llvm::core::LLVMArrayType;

    unsafe {
        LLVMArrayType(elements_type, size as c_uint)
    }
}

/// Create a LLVM `pointer` type.
pub fn pointer_type(element_type: LLVMTypeRef, address_space: u32) -> LLVMTypeRef {
    use llvm::core::LLVMPointerType;

    unsafe {
        LLVMPointerType(element_type, address_space as c_uint)
    }
}

/// Create a LLVM `vector` type.
pub fn vector_type(elements_type: LLVMTypeRef, size: u32) -> LLVMTypeRef {
    use llvm::core::LLVMVectorType;

    unsafe {
        LLVMVectorType(elements_type, size as c_uint)
    }
}

/// Map a Rust type to a LLVM type.
pub trait VMRepresentation {
    fn to_vm_representation(self, context: &Context) -> Value;
}

macro_rules! to_integer {
    ($type_name:ty as $($alias:ty)as+ => $LLVM_type:ident($($LLVM_type_argument:expr),*)) => (
        impl VMRepresentation for $type_name {
            fn to_vm_representation(self, context: &Context) -> Value {
                use llvm::core::$LLVM_type;

                Value::from_ref(
                    unsafe {
                        LLVMConstInt(
                            $LLVM_type(context.to_ref(), $($LLVM_type_argument),*),
                            self as $($alias)as+,
                            0 as LLVMBool
                        )
                    }
                )
            }
        }
    );

    ($type_name:ty as $($alias:ty)as+ => $LLVM_type:ident) => (
        to_integer!{$type_name as $($alias)as+ => $LLVM_type()}
    )
}

macro_rules! to_float {
    ($type_name:ty as $alias:ty => $LLVM_type:ident) => (
        impl VMRepresentation for $type_name {
            fn to_vm_representation(self, context: &Context) -> Value {
                use llvm::core::$LLVM_type;

                Value::from_ref(
                    unsafe {
                        LLVMConstReal(
                            $LLVM_type(context.to_ref()),
                            self as $alias
                        )
                    }
                )
            }
        }
    )
}

to_integer!(bool  as c_ulonglong        => LLVMInt1TypeInContext);
to_integer!(u8    as c_ulonglong        => LLVMInt8TypeInContext);
to_integer!(i8    as c_ulonglong        => LLVMInt8TypeInContext);
to_integer!(u16   as c_ulonglong        => LLVMInt16TypeInContext);
to_integer!(i16   as c_ulonglong        => LLVMInt16TypeInContext);
to_integer!(u32   as c_ulonglong        => LLVMInt32TypeInContext);
to_integer!(i32   as c_ulonglong        => LLVMInt32TypeInContext);
to_integer!(u64   as c_ulonglong        => LLVMInt64TypeInContext);
to_integer!(i64   as c_ulonglong        => LLVMInt64TypeInContext);
to_integer!(usize as c_ulonglong        => LLVMIntTypeInContext(mem::size_of::<isize>() as c_uint * 8));
to_integer!(isize as c_ulonglong        => LLVMIntTypeInContext(mem::size_of::<isize>() as c_uint * 8));
to_integer!(char  as u32 as c_ulonglong => LLVMInt32TypeInContext);

to_float!(f32 as f64 => LLVMFloatTypeInContext);
to_float!(f64 as f64 => LLVMDoubleTypeInContext);

impl<'a> VMRepresentation for &'a str {
    fn to_vm_representation(self, context: &Context) -> Value {
        self.as_bytes().to_vm_representation(context)
    }
}

impl<'a> VMRepresentation for &'a [u8] {
    fn to_vm_representation(self, context: &Context) -> Value {
        Value::from_ref(
            unsafe {
                LLVMConstStringInContext(
                    context.to_ref(),
                    self.as_ptr() as *const c_char,
                    self.len() as c_uint,
                    1 as LLVMBool
                )
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::super::context::Context;
    use super::VMRepresentation;

    macro_rules! test_vm_representation {
        ($test_case_name:ident: ($value:expr, $expect:expr)) => (
            #[test]
            fn $test_case_name() {
                let context = Context::new();
                let value   = $value.to_vm_representation(&context);

                assert_eq!(
                    $expect,
                    format!("{}", value)
                );
            }
        )
    }

    test_vm_representation!(case_boolean_true : (true,   "i1 true"));
    test_vm_representation!(case_boolean_false: (false,  "i1 false"));
    test_vm_representation!(case_u8           : (7u8,    "i8 7"));
    test_vm_representation!(case_i8           : (7i8,    "i8 7"));
    test_vm_representation!(case_u16          : (7u16,   "i16 7"));
    test_vm_representation!(case_i16          : (7i16,   "i16 7"));
    test_vm_representation!(case_u32          : (7u32,   "i32 7"));
    test_vm_representation!(case_i32          : (7i32,   "i32 7"));
    test_vm_representation!(case_u64          : (7u64,   "i64 7"));
    test_vm_representation!(case_i64          : (7i64,   "i64 7"));
    test_vm_representation!(case_usize        : (7usize, "i64 7"));
    test_vm_representation!(case_isize        : (7isize, "i64 7"));
    test_vm_representation!(case_char         : ('*',    "i32 42"));
    test_vm_representation!(case_f32          : (4.2f32, "float 0x4010CCCCC0000000"));
    test_vm_representation!(case_f64          : (4.2f64, "double 4.200000e+00"));
    test_vm_representation!(case_str          : ("foo",  "[3 x i8] c\"foo\""));
    test_vm_representation!(case_u8_slice     : (b"bar", "[3 x i8] c\"bar\""));
    test_vm_representation!(case_unicode_str  : ("😄",    "[4 x i8] c\"\\F0\\9F\\98\\84\""));
}

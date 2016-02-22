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
use super::value::Value;

use libc::{c_char, c_uint, c_ulonglong};
use llvm::core::{
    LLVMConstInt,
    LLVMConstReal,
    LLVMConstStringInContext
};
use llvm::prelude::LLVMBool;
use std::mem;

pub trait Constant {
    fn as_vm_constant(self, context: &Context) -> Value;
}

macro_rules! constantify_integer {
    ($type_name:ty as $($alias:ty)as+ => $LLVMType:ident($($LLVMTypeArgument:expr),*)) => (
        impl Constant for $type_name {
            fn as_vm_constant(self, context: &Context) -> Value {
                use llvm::core::$LLVMType;

                Value::from_ref(
                    unsafe {
                        LLVMConstInt(
                            $LLVMType(context.to_ref(), $($LLVMTypeArgument),*),
                            self as $($alias)as+,
                            0 as LLVMBool
                        )
                    }
                )
            }
        }
    );

    ($type_name:ty as $($alias:ty)as+ => $LLVMType:ident) => (
        constantify_integer!{$type_name as $($alias)as+ => $LLVMType()}
    )
}

macro_rules! constantify_float {
    ($type_name:ty as $alias:ty => $LLVMType:ident) => (
        impl Constant for $type_name {
            fn as_vm_constant(self, context: &Context) -> Value {
                use llvm::core::$LLVMType;

                Value::from_ref(
                    unsafe {
                        LLVMConstReal(
                            $LLVMType(context.to_ref()),
                            self as $alias
                        )
                    }
                )
            }
        }
    )
}

constantify_integer!(bool  as c_ulonglong        => LLVMInt1TypeInContext);
constantify_integer!(u8    as c_ulonglong        => LLVMInt8TypeInContext);
constantify_integer!(i8    as c_ulonglong        => LLVMInt8TypeInContext);
constantify_integer!(u16   as c_ulonglong        => LLVMInt16TypeInContext);
constantify_integer!(i16   as c_ulonglong        => LLVMInt16TypeInContext);
constantify_integer!(u32   as c_ulonglong        => LLVMInt32TypeInContext);
constantify_integer!(i32   as c_ulonglong        => LLVMInt32TypeInContext);
constantify_integer!(u64   as c_ulonglong        => LLVMInt64TypeInContext);
constantify_integer!(i64   as c_ulonglong        => LLVMInt64TypeInContext);
constantify_integer!(usize as c_ulonglong        => LLVMIntTypeInContext(mem::size_of::<isize>() as c_uint * 8));
constantify_integer!(isize as c_ulonglong        => LLVMIntTypeInContext(mem::size_of::<isize>() as c_uint * 8));
constantify_integer!(char  as u32 as c_ulonglong => LLVMInt32TypeInContext);

constantify_float!(f32 as f64 => LLVMFloatTypeInContext);
constantify_float!(f64 as f64 => LLVMDoubleTypeInContext);

impl<'a> Constant for &'a str {
    fn as_vm_constant(self, context: &Context) -> Value {
        self.as_bytes().as_vm_constant(context)
    }
}

impl<'a> Constant for &'a [u8] {
    fn as_vm_constant(self, context: &Context) -> Value {
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
    use super::Constant;

    macro_rules! test {
        ($test_case_name:ident: [$value:expr, $expect:expr]) => (
            #[test]
            fn $test_case_name() {
                let context = Context::new();
                let value   = $value.as_vm_constant(&context);

                assert_eq!(
                    $expect,
                    format!("{}", value)
                );
            }
        )
    }

    test!(case_boolean_true : [true,   "i1 true"]);
    test!(case_boolean_false: [false,  "i1 false"]);
    test!(case_u8           : [7u8,    "i8 7"]);
    test!(case_i8           : [7i8,    "i8 7"]);
    test!(case_u16          : [7u16,   "i16 7"]);
    test!(case_i16          : [7i16,   "i16 7"]);
    test!(case_u32          : [7u32,   "i32 7"]);
    test!(case_i32          : [7i32,   "i32 7"]);
    test!(case_u64          : [7u64,   "i64 7"]);
    test!(case_i64          : [7i64,   "i64 7"]);
    test!(case_usize        : [7usize, "i64 7"]);
    test!(case_isize        : [7isize, "i64 7"]);
    test!(case_char         : ['*',    "i32 42"]);
    test!(case_f32          : [4.2f32, "float 0x4010CCCCC0000000"]);
    test!(case_f64          : [4.2f64, "double 4.200000e+00"]);
    test!(case_str          : ["foo",  "[3 x i8] c\"foo\""]);
    test!(case_u8_slice     : [b"bar", "[3 x i8] c\"bar\""]);
}

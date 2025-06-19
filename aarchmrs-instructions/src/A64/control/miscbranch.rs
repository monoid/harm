/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod RETAASPPC_only_miscbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RETAASPPC_only_miscbranch {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl RETAASPPC_only_miscbranch {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01010101000u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod RETABSPPC_only_miscbranch {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct RETABSPPC_only_miscbranch {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl RETABSPPC_only_miscbranch {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b01010101001u32 << 21u32 | self.imm16.into_inner() << 5u32 | 0b11111u32 << 0u32,
            )
        }
    }
}

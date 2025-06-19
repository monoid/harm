/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod UDF_only_perm_undef {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct UDF_only_perm_undef {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl UDF_only_perm_undef {
        #[inline]
        pub const fn new(imm16: ::aarchmrs_types::BitValue<16>) -> Self {
            Self { imm16 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b0000000000000000u32 << 16u32 | self.imm16.into_inner() << 0u32,
            )
        }
    }
}

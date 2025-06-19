/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod zero_za_i_ {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct zero_za_i_ {
        pub imm8: ::aarchmrs_types::BitValue<8>,
    }
    impl zero_za_i_ {
        #[inline]
        pub const fn new(imm8: ::aarchmrs_types::BitValue<8>) -> Self {
            Self { imm8 }
        }
        #[inline]
        pub const fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000000000100000000000u32 << 8u32 | self.imm8.into_inner() << 0u32,
            )
        }
    }
}

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
        pub fn new(imm8: impl Into<::aarchmrs_types::BitValue<8>>) -> Self {
            Self { imm8: imm8.into() }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b110000000000100000000000u32 << 8u32 | u32::from(self.imm8) << 0u32,
            )
        }
    }
}

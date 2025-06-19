/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod AUTIASPPC_only_dp_1src_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIASPPC_only_dp_1src_imm {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl AUTIASPPC_only_dp_1src_imm {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110011100u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b11111u32 << 0u32,
            )
        }
    }
}
pub mod AUTIBSPPC_only_dp_1src_imm {
    #[derive(Copy, Clone, Debug, Default)]
    pub struct AUTIBSPPC_only_dp_1src_imm {
        pub imm16: ::aarchmrs_types::BitValue<16>,
    }
    impl AUTIBSPPC_only_dp_1src_imm {
        #[inline]
        pub fn new(imm16: impl Into<::aarchmrs_types::BitValue<16>>) -> Self {
            Self {
                imm16: imm16.into(),
            }
        }
        #[inline]
        pub fn build(&self) -> ::aarchmrs_types::InstructionCode {
            ::aarchmrs_types::InstructionCode::from_u32(
                0b11110011101u32 << 21u32 | u32::from(self.imm16) << 5u32 | 0b11111u32 << 0u32,
            )
        }
    }
}

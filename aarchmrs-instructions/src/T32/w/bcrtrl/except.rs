/* Copyright (c) 2010-2025 Arm Limited or its affiliates. All rights reserved.
 *
 * This document is Non-confidential and licensed under the BSD 3-clause license.
 */

pub mod HVC_T1 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111111000001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "HVC_T1";
    #[inline]
    pub const fn HVC_T1(
        imm4: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101111110u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b1000u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
pub mod SMC_T1_AS {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111111111111111u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111111100001000000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000111111111111u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "SMC_T1_AS";
    #[inline]
    pub const fn SMC_T1_AS(
        imm4: ::aarchmrs_types::BitValue<4>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101111111u32 << 20u32 | imm4.into_inner() << 16u32 | 0b1000000000000000u32 << 0u32,
        )
    }
}
pub mod UDF_T2 {
    #[cfg(feature = "meta")]
    pub const OPCODE_MASK: u32 = 0b11111111111100001111000000000000u32;
    #[cfg(feature = "meta")]
    pub const OPCODE: u32 = 0b11110111111100001010000000000000u32;
    #[cfg(feature = "meta")]
    pub const SHOULD_BE_MASK: u32 = 0b00000000000000000000000000000000u32;
    #[cfg(feature = "meta")]
    pub const NAME: &str = "UDF_T2";
    #[inline]
    pub const fn UDF_T2(
        imm4: ::aarchmrs_types::BitValue<4>,
        imm12: ::aarchmrs_types::BitValue<12>,
    ) -> ::aarchmrs_types::InstructionCode {
        ::aarchmrs_types::InstructionCode::from_u32(
            0b111101111111u32 << 20u32
                | imm4.into_inner() << 16u32
                | 0b1010u32 << 12u32
                | imm12.into_inner() << 0u32,
        )
    }
}
